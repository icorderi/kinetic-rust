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

use docopt::Docopt;
use std::vec;
use kinetic::KineticResult;
use std::ascii::OwnedAsciiExt;
use cli::{CliDispatcher, CliCommand};


#[deriving(Decodable, Show)]
pub struct Args {
    arg_command: Option<Command>,
    arg_args: Vec<String>,
    flag_list: bool,
    flag_help: bool,
    flag_version: bool,
}

#[deriving(Decodable, Show)]
pub enum Command {
    Help,
    Write,
    Info,
}

impl CliDispatcher for Command {

    fn dispatch(&self, mut argv: vec::Vec<String>) -> KineticResult<()> {
        argv.insert(0, format!("{}", self).into_ascii_lower());
        argv.insert(0, "kinetic-rust".to_string());

        let cmd: Box<CliCommand> =
            match *self {
                Command::Write => {
                    let x: ::write::WriteArgs = CliCommand::from_argv(argv); (box x) as Box<CliCommand>
                },
                Command::Info  => {
                    let x: ::info::InfoArgs = CliCommand::from_argv(argv); (box x) as Box<CliCommand>
                }
                Command::Help => {
                    let x: ::help::HelpArgs = CliCommand::from_argv(argv); (box x) as Box<CliCommand>
                }
            };

        let result = try!(cmd.execute());
        Ok(result) // return
    }

}


#[stable]
fn version() -> String {
    format!("kinetic-rust {}\nkinetic-protocol {}", ::kinetic::version(), ::kinetic::protocol_version())
}


// Write the Docopt usage string.
static USAGE: &'static str = "
Kinetic from Rust!

Usage: kinetic-rust <command> [<args>...]
       kinetic-rust [options]

Options:
  -h, --help       Show this message.
  --version        Show the version of kinetic-rust.
  --list           List installed commands
  -v, --verbose    Use verbose output

Some common kinetic-rust commands are:
    write          Write objects to a kinetic device
    info           Show information for a kinetic device

See 'kinetic-rust help <command>' for more information on a specific command.
";
pub fn main_with_args(argv: vec::Vec<String>) -> KineticResult<()> {
    let docopt = Docopt::new(USAGE).unwrap()
                            .options_first(true)
                            .argv(argv.into_iter())
                            .help(true)
                            .version(Some(version()));

    let args: Args = docopt.decode().unwrap_or_else(|e| e.exit());

    // FIXME: figure how to make generic...
    if args.flag_list {
        println!("Installed Commands:");
        println!("    write");
        println!("    info");
        println!("    help");
        return Ok(());
    }

    match args.arg_command {
        Some(cmd) => cmd.dispatch(args.arg_args),
        None => {
            println!("{}", USAGE)
            Ok(())
        },
    } // return
}
