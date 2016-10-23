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
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::iter::Iterator;
use kinetic::commands::Put;
use std::default::Default;
use std::vec;
use std::time::Duration;
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread::spawn;

#[derive(RustcDecodable, Debug)]
pub struct BenchArgs {
    flag_verbose: bool,
    flag_count: Option<usize>,
    flag_size: Option<usize>,
    flag_path: Option<String>,
}

static USAGE: &'static str = "
Writes to a set of drives

Usage: kinetic-rust bench [options]
       kinetic-rust bench (-h | --help)

Options:
  -h, --help               Print this message
  -s, --size BYTES         Size of the value in bytes (default: 1 MB)
  -c, --count COUNT        Number of key/value pairs to send (default: 10)
  -p, --path PATH          Path to the file with the kinetic devices (default: drives)
  -v, --verbose            Use verbose output
";

fn to_utf8(s: &[u8]) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

// fn bench(c: &::kinetic::AsyncClient, size: usize, count: usize) -> KineticResult<(&::kinetic::AsyncClient, String)> {
//     let d = Duration::span(|| {
//         let mut responses = vec::Vec::with_capacity(count);

//         for i in (0.. count) {
//             let data = vec::Vec::from_elem(size, 0u8);
//             let r = c.send_future(Put { key: format!("opt-bench.{}", i).as_bytes().to_vec(),
//                                         value: data,
//                                         ..Default::default()});
//             responses.push(r);
//         }

//         // wait on all
//         for r in responses.into_iter() {
//             r.into_inner().unwrap();
//         }
//     });

//     let ops = count as f64  / (d.num_milliseconds() as f64 / 1000.0);
//     let transfered = (count as f64 * size as f64) / (1024.0 * 1024.0);
//     let bw = transfered / (d.num_milliseconds() as f64 / 1000.0);

//     Ok((c, format!("operation took {}ms ({:.2} MB/s, {:.2} op/s)", d.num_milliseconds(), bw, ops)))
// }

fn execute(cmd: &BenchArgs, shell: &mut ::shell::MultiShell) -> KineticResult<()> {
//     // debug!("executing; cmd=kinetic-rust-bench; args={}", ::std::env::args());
//     shell.set_verbose(cmd.flag_verbose);

//     let path = Path::new(&cmd.flag_path.clone().unwrap_or("drives".to_string()));
//     let file = try!(File::open(&path));
//     let mut file = BufReader::new(file);
//     let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();

//     let clients: Vec<KineticResult<::kinetic::AsyncClient>> = lines.into_iter().map(
//         |mut x| {
//             x = x.replace("\n","");
//             if ! x.contains(":") {
//                 x = x + ":8123";
//             }
//             println!("{}", x);
//             ::kinetic::Client::new(x.as_ref())
//         }).collect();;

//     let size = cmd.flag_size.unwrap_or(1024*1024);
//     let count = cmd.flag_count.unwrap_or(10);

//     let (tx,rx) = channel();
//     let mut pending = clients.len();

//     let d = Duration::span(|| {
//         //let clients = clients.clone(); // ugly... but apparently clients doesnt live long enough...
//         for c in clients.iter() {
//             match *c {
//                 Ok(ref c)  => {
//                     shell.status("Connected", to_utf8(c.get_config().get_serialNumber())).unwrap(); //ugly unwrap
//                     let tx = tx.clone();
//                     spawn(move|| {
//                         let r = bench(c, size, count);
//                         tx.send(r);
//                     });
//                 },
//                 Err(ref e) => shell.error_full(e, true).unwrap(), //ungly unwrap
//             };
//         }

//         while pending > 0 {
//             let r = rx.recv();
//             match r {
//                 Ok((c,r)) => shell.tag(to_utf8(c.get_config().get_serialNumber()), r).unwrap(), //ugly unwrap
//                 Err(e) => shell.error_full(&e, true).unwrap(), //ugly unwrap
//             };

//             pending -= 1;
//         }
//     });

//     let count = count * clients.len();
//     try!(shell.status("Count?", count));
//     let ops = count as f64  / (d.num_milliseconds() as f64 / 1000.0);
//     let transfered = (count as f64 * size as f64) / (1024.0 * 1024.0);
//     let bw = transfered / (d.num_milliseconds() as f64 / 1000.0);

//     try!(shell.status("Done",format!("benchmark took {}ms ({:.2} MB/s, {:.2} op/s)", d.num_milliseconds(), bw, ops)));

    Ok(()) //return
}

impl ::cli::CliCommand for BenchArgs {
    fn from_argv(argv: ::std::vec::Vec<String>) -> BenchArgs {
        ::docopt::Docopt::new(::cli::CliCommand::usage(None::<BenchArgs>))
            .and_then(|d| d.argv(argv.clone().into_iter()).decode() )
            .unwrap_or_else(|e| e.exit())
    }

    #[inline]
    fn execute(&self, shell: &mut ::shell::MultiShell) -> ::kinetic::KineticResult<()> {
        execute(self, shell)
    }

    #[inline]
    fn usage(_: Option<BenchArgs>) -> &'static str { USAGE }
}
