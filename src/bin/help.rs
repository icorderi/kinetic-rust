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


#[derive(RustcDecodable, Debug)]
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

fn execute(cmd: &HelpArgs, shell: &mut ::shell::MultiShell) -> KineticResult<()> {
    //debug!("executing; cmd=kinetic-rust-help; args={}", ::std::env::args());
    shell.set_verbose(cmd.flag_verbose);

    let argv = ["kinetic-rust".to_string(),
                format!("{:?}", cmd.arg_command).into_ascii_lowercase(),
                "-h".to_string()];

    ::main::main_with_args(&argv, shell) //return
}

impl ::cli::CliCommand for HelpArgs {
    fn from_argv(argv: ::std::vec::Vec<String>) -> HelpArgs {
        ::docopt::Docopt::new(::cli::CliCommand::usage(None::<HelpArgs>))
            .and_then(|d| d.argv(argv.clone().into_iter()).decode() )
            .unwrap_or_else(|e| e.exit())
    }

    #[inline]
    fn execute(&self, shell: &mut ::shell::MultiShell) -> ::kinetic::KineticResult<()> {
        execute(self, shell)
    }

    #[inline]
    fn usage(_: Option<HelpArgs>) -> &'static str { USAGE }
}
