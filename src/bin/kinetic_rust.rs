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

#![crate_name = "kinetic_rust"]

// Skip entire crate
#![cfg(not(test))]

#![feature(std_misc)]
#![feature(convert)]

extern crate libc;
extern crate rustc_serialize;
extern crate docopt;
extern crate kinetic;
extern crate term;
#[macro_use] extern crate log;

use std::env;

mod cli;
mod main;
mod help;
mod info;
mod write;
mod bench;
mod get_log;
pub mod shell;

#[cfg(not(test))]
fn main() {
    let mut shell = ::shell::MultiShell::new_stdio(false);
    let args: Vec<_> = env::args().collect();
    let r = main::main_with_args(args.as_ref(), &mut shell);
    match r {
        Ok(_) => (),
        Err(e) => shell.error_full(&e, true).unwrap(),
    }
}
