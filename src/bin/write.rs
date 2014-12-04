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

use std::time::duration::Duration;
use std::vec;
use std::default::Default;
use kinetic::commands::{Put, Get};
use kinetic::KineticResult;


#[deriving(Decodable, Show)]
pub struct WriteArgs {
    flag_verbose: bool,
    flag_count: Option<uint>,
    flag_size: Option<uint>,
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

fn execute(cmd: &WriteArgs) -> KineticResult<()> {
    println!("Connecting to {}", cmd.arg_target);

    let c = try!(::kinetic::Client::new(format!("{}:8123", cmd.arg_target).as_slice()));

    c.send(Put { key: "rust".as_bytes().to_vec(),
                 value: format!("Hello from {}!", ::kinetic::version()).as_bytes().to_vec(),
                 ..Default::default() }).unwrap();
    let v = try!(c.send(Get { key: "rust".as_bytes().to_vec() }));

    println!("Read back: {}", String::from_utf8(v.value).unwrap());

    let items = cmd.flag_count.unwrap_or(10u);
        // benchmark
    let d = Duration::span(|| {
        let mut responses = vec::Vec::with_capacity(items);

        for i in range(0u, items) {
            let data = vec::Vec::from_elem(cmd.flag_size.unwrap_or(1024*1024u), 0u8);
            let r = c.send_future(Put { key: format!("opt-bench.{}", i).as_bytes().to_vec(),
                                        value: data,
                                        ..Default::default()});
            responses.push(r);
        }

        // wait on all
        for r in responses.into_iter() {
            r.into_inner().unwrap();
        }
    });
    let bw = items as f64 / (d.num_milliseconds() as f64 / 1000.0);
    println!("Benchmark took {}ms ({} MB/s)", d.num_milliseconds(), bw);

    Ok(()) //return
}

cmd!(WriteArgs, execute, USAGE)
