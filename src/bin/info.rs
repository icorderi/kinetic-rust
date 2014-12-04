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


#[deriving(Decodable, Show)]
pub struct InfoArgs {
    flag_verbose: bool,
    arg_target: String,
}

static USAGE: &'static str = "
Get info from kinetic device

Usage: kinetic-rust info [options] <target>
       kinetic-rust info (-h | --help)

Options:
  -h, --help            Print this message
  -v, --verbose         Use verbose output
";

fn execute(cmd: &InfoArgs) -> KineticResult<()> {
    let c = try!(::kinetic::Client::new(format!("{}:8123", cmd.arg_target).as_slice()));
    println!("{}", c.get_config());

    Ok(()) //return
}

cmd!(InfoArgs, execute, USAGE)
