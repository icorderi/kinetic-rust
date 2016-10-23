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

use std::time::Duration;
use std::vec;
use std::default::Default;
use kinetic::commands::{Put, Get};
use kinetic::KineticResult;


#[derive(RustcDecodable, Debug)]
pub struct WriteArgs {
    flag_verbose: bool,
    flag_count: Option<usize>,
    flag_size: Option<usize>,
    arg_target: String,
}

static USAGE: &'static str = "
Write an object to a kinetic device

Usage: kinetic-rust write [options] <target>
       kinetic-rust write (-h | --help)

Options:
  -h, --help               Print this message
  -s, --size BYTES         Size of the value in bytes (default: 1 MB)
  -c, --count COUNT        Number of key/value pairs to send (default: 10)
  -v, --verbose            Use verbose output
";

fn execute(cmd: &WriteArgs, shell: &mut ::shell::MultiShell) -> KineticResult<()> {
    //debugı!("executing; cmd=kinetic-rust-write; args={}", ::std::env::args());
    shell.set_verbose(cmd.flag_verbose);
    unimplemented!();

    // try!(shell.status("Connecting", format!("device at {}:8123", cmd.arg_target)));

    // let c = try!(::kinetic::Client::new(format!("{}:8123", cmd.arg_target).as_str()));

    // c.send(Put { key: "rust".as_bytes().to_vec(),
    //              value: format!("Hello from {}!", ::kinetic::version()).as_bytes().to_vec(),
    //              ..Default::default() }).unwrap();
    // let v = try!(c.send(Get { key: "rust".as_bytes().to_vec() }));

    // try!(shell.status("Response", format!("{}", String::from_utf8(v.value).unwrap())));

    // let items = cmd.flag_count.unwrap_or(10);
    //     // benchmark
    // let size = cmd.flag_size.unwrap_or(1024*1024);
    // let d = Duration::span(|| {
    //     let mut responses = vec::Vec::with_capacity(items);

    //     for i in (0.. items) {
    //         let data = vec![0u8; size];
    //         let r = c.send_future(Put { key: format!("opt-bench.{}", i).as_bytes().to_vec(),
    //                                     value: data,
    //                                     ..Default::default()});
    //         responses.push(r);
    //     }

    //     // wait on all
    //     for r in responses.into_iter() {
    //         r.into_inner().unwrap();
    //     }
    // });
    // let ops = items as f64  / (d.num_milliseconds() as f64 / 1000.0);
    // let transfered = (items as f64 * size as f64) / (1024.0 * 1024.0);
    // let bw = transfered / (d.num_milliseconds() as f64 / 1000.0);

    // try!(shell.status("Done", format!("benchmark took {}ms ({:.2} MB/s, {:.2} op/s)", d.num_milliseconds(), bw, ops)));

    // Ok(()) //return
}

impl ::cli::CliCommand for WriteArgs {
    fn from_argv(argv: ::std::vec::Vec<String>) -> WriteArgs {
        ::docopt::Docopt::new(::cli::CliCommand::usage(None::<WriteArgs>))
            .and_then(|d| d.argv(argv.clone().into_iter()).decode() )
            .unwrap_or_else(|e| e.exit())
    }

    #[inline]
    fn execute(&self, shell: &mut ::shell::MultiShell) -> ::kinetic::KineticResult<()> {
        execute(self, shell)
    }

    #[inline]
    fn usage(_: Option<WriteArgs>) -> &'static str { USAGE }
}
