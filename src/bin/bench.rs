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
use std::io::BufferedReader;
use std::io::File;

#[deriving(Decodable, Show)]
pub struct BenchArgs {
    flag_verbose: bool,
    flag_count: Option<uint>,
    flag_size: Option<uint>,
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

fn execute(cmd: &BenchArgs, shell: &mut ::shell::MultiShell) -> KineticResult<()> {
    debug!("executing; cmd=kinetic-rust-bench; args={}", ::std::os::args());
    shell.set_verbose(cmd.flag_verbose);

    let path = Path::new(cmd.flag_path.clone().unwrap_or("drives".to_string()));
    let file = try!(File::open(&path));
    let mut file = BufferedReader::new(file);
    let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();

    try!(shell.error("Code me"));

    Ok(()) //return
}

cmd!(BenchArgs, execute, USAGE)
