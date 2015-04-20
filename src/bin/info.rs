// Copyright (c) 2014 Seagate Technology

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

// author: Ignacio Corderi

use kinetic::KineticResult;


#[derive(RustcDecodable, Debug)]
pub struct InfoArgs {
    flag_verbose: bool,
    flag_detailed: bool,
    arg_target: String,
}

static USAGE: &'static str = "
Get info from kinetic device

Usage: kinetic-rust info [options] <target>
       kinetic-rust info (-h | --help)

Options:
  -h, --help            Print this message
  -d, --detailed        Shows more detailed information
  -v, --verbose         Use verbose output
";

fn to_utf8(s: &[u8]) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn execute(cmd: &InfoArgs, shell: &mut ::shell::MultiShell) -> KineticResult<()> {
    //debug!("executing; cmd=kinetic-rust-info; args={}", ::std::env::args());
    shell.set_verbose(cmd.flag_verbose);

    try!(shell.status("Connecting", format!("device at {}:8123", cmd.arg_target)));

    let c = try!(::kinetic::Client::new(format!("{}:8123", cmd.arg_target).as_str()));

    if cmd.flag_detailed {
        try!(shell.header("Device"));
        try!(shell.tag("Vendor", c.get_config().get_vendor()));
        if c.get_config().get_model() == "Simulator" {
            try!(shell.tag_color("Model", c.get_config().get_model(), ::term::color::BRIGHT_YELLOW));
        } else {
            try!(shell.tag("Model", c.get_config().get_model()));
        }
        try!(shell.tag("SN", to_utf8(c.get_config().get_serialNumber())));
    } else {
        if c.get_config().get_model() == "Simulator" {
            try!(shell.tag_color("Simulator", to_utf8(c.get_config().get_serialNumber()), ::term::color::BRIGHT_YELLOW));
        } else {
            let msg = format!("{} {} (SN: {})", c.get_config().get_vendor(),
                            c.get_config().get_model(), to_utf8(c.get_config().get_serialNumber()));
            try!(shell.tag("Device", msg));
        }
    }
    try!(shell.tag("WWN", to_utf8(c.get_config().get_worldWideName())));

    if cmd.flag_detailed {
        try!(shell.header("Device firmware"));
        try!(shell.tag("Version", c.get_config().get_version()));
        try!(shell.tag(".(date)", c.get_config().get_compilationDate()));
        try!(shell.tag(".(hash)", c.get_config().get_sourceHash()));
    } else {
        try!(shell.tag("Firmware", c.get_config().get_version()));
    }

    let v = ::kinetic::protocol_version();
    if cmd.flag_detailed {
        try!(shell.header("Kinetic protocol"));
        if v.as_str() == c.get_config().get_protocolVersion() {
            try!(shell.tag_color("Version", c.get_config().get_protocolVersion(), ::term::color::GREEN));
        } else {
            try!(shell.tag_color("Version", c.get_config().get_protocolVersion(), ::term::color::BRIGHT_RED));
        }
        try!(shell.tag(".(date)", c.get_config().get_protocolCompilationDate()));
        try!(shell.tag(".(hash)", c.get_config().get_protocolSourceHash()));
    } else {
        if v.as_str() == c.get_config().get_protocolVersion() {
            try!(shell.tag_color("Protocol", c.get_config().get_protocolVersion(), ::term::color::GREEN));
        } else {
            try!(shell.tag_color("Protocol", c.get_config().get_protocolVersion(), ::term::color::BRIGHT_RED));
        }
    }

    if cmd.flag_detailed {
        try!(shell.header("Network"));
        try!(shell.tag("Port", c.get_config().get_port()));
        try!(shell.tag("Tls port", c.get_config().get_tlsPort()));

        for interface in c.get_config().get_interface().iter() {
            try!(shell.tag("Interface", interface.get_name()));
            if interface.has_MAC() {
                try!(shell.tag(".(MAC)", to_utf8(interface.get_MAC())));
            }
            if interface.has_ipv4Address() {
                try!(shell.tag(".(IPv4)", to_utf8(interface.get_ipv4Address())));
            }
            if interface.has_ipv6Address() {
                try!(shell.tag(".(IPv6)", to_utf8(interface.get_ipv6Address())));
            }
        }
    } else {
        for interface in c.get_config().get_interface().iter() {
            if interface.has_MAC() && interface.has_ipv4Address() {
                try!(shell.tag("Network", format!("{} ({})",
                                                  to_utf8(interface.get_ipv4Address()),
                                                  to_utf8(interface.get_MAC()))));
            }
        }
    }

    Ok(()) //return
}

impl ::cli::CliCommand for InfoArgs {
    fn from_argv(argv: ::std::vec::Vec<String>) -> InfoArgs {
        ::docopt::Docopt::new(::cli::CliCommand::usage(None::<InfoArgs>))
            .and_then(|d| d.argv(argv.clone().into_iter()).decode() )
            .unwrap_or_else(|e| e.exit())
    }

    #[inline]
    fn execute(&self, shell: &mut ::shell::MultiShell) -> ::kinetic::KineticResult<()> {
        execute(self, shell)
    }

    #[inline]
    fn usage(_: Option<InfoArgs>) -> &'static str { USAGE }
}
