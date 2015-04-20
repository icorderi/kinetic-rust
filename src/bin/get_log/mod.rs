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
pub struct LogArgs {
    flag_verbose: bool,
    arg_target: String,
}

static USAGE: &'static str = "
Get log information from kinetic device

Usage: kinetic-rust log messages [options] <target>
       kinetic-rust log (-h | --help)

Options:
  -h, --help            Print this message
  -v, --verbose         Use verbose output
";

fn to_utf8(s: &[u8]) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn execute(cmd: &LogArgs, shell: &mut ::shell::MultiShell) -> KineticResult<()> {
    //debug!("executing; cmd=kinetic-rust-log; args={}", ::std::env::args());
    shell.set_verbose(cmd.flag_verbose);

    try!(shell.status("Connecting", format!("device at {}:8123", cmd.arg_target)));

    let c = try!(::kinetic::Client::new(format!("{}:8123", cmd.arg_target).as_str()));

    let x = try!(c.send(::kinetic::commands::GetLog { log_types: vec![::kinetic::proto::command::LogType::MESSAGES]}));

    let s = x.get_messages();

    println!("{}", to_utf8(s));

    Ok(()) //return
}

impl ::cli::CliCommand for LogArgs {
    fn from_argv(argv: ::std::vec::Vec<String>) -> LogArgs {
        ::docopt::Docopt::new(::cli::CliCommand::usage(None::<LogArgs>))
        .and_then(|d| d.argv(argv.clone().into_iter()).decode() )
        .unwrap_or_else(|e| e.exit())
    }

    #[inline]
    fn execute(&self, shell: &mut ::shell::MultiShell) -> ::kinetic::KineticResult<()> {
        execute(self, shell)
    }

    #[inline]
    fn usage(_: Option<LogArgs>) -> &'static str { USAGE }
}
