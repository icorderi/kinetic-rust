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
use std::ascii::OwnedAsciiExt;


#[deriving(Decodable, Show)]
pub struct HelpArgs {
    flag_verbose: bool,
    arg_command: ::main::Command,
}

static USAGE: &'static str = "
Show help for a command

Usage: kinetic-rust help [options] <command>
       kinetic-rust help (-h | --help)

Options:
  -h, --help            Print this message
  -v, --verbose         Use verbose output
";

fn execute(cmd: &HelpArgs) -> KineticResult<()> {
    let argv = vec!["kinetic-rust".to_string(),
                    format!("{}", cmd.arg_command).into_ascii_lower(),
                    "-h".to_string()];

    ::main::main_with_args(argv) //return
}

cmd!(HelpArgs, execute, USAGE)
