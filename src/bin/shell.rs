// Lifted from https://github.com/rust-lang/cargo/blob/master/src/cargo/core/shell.rs
// under MIT license https://github.com/rust-lang/cargo/blob/master/LICENSE-MIT
//
// Modified by: Ignacio Corderi

use term::{Terminal, TerminfoTerminal, color};
use term::color::{Color, BLACK, RED, GREEN, YELLOW, BRIGHT_YELLOW, BRIGHT_RED};
use term::attr::{Attr, Bold};
use std::io::{IoResult, stderr, stdout};
use std::io::stdio::{stdout_raw, stderr_raw};
use std::fmt::Show;
use std::error::Error;

use self::AdequateTerminal::{NoColor, Colored};

static GREY: u16 = 8;

pub struct ShellConfig {
    pub color: bool,
    pub verbose: bool,
    pub tty: bool
}

enum AdequateTerminal {
    NoColor(Box<Writer + Send>),
    Colored(Box<Terminal<UghWhyIsThisNecessary> + Send>)
}

pub struct Shell {
    terminal: AdequateTerminal,
    config: ShellConfig,
}

pub struct MultiShell {
    out: Shell,
    err: Shell,
    verbose: bool
}

pub type Callback<'a> = |&mut MultiShell|:'a -> IoResult<()>;

struct UghWhyIsThisNecessary {
    inner: Box<Writer + Send>,
}

impl MultiShell {

    pub fn new_stdio(verbose: bool) -> MultiShell {
        let tty = stderr_raw().isatty();
        let stderr = box stderr() as Box<Writer + Send>;

        let config = ShellConfig { color: true, verbose: verbose, tty: tty };
        let err = Shell::create(stderr, config);

        let tty = stdout_raw().isatty();
        let stdout = box stdout() as Box<Writer + Send>;

        let config = ShellConfig { color: true, verbose: verbose, tty: tty };
        let out = Shell::create(stdout, config);

        MultiShell::new(out, err, verbose)
    }

    pub fn new(out: Shell, err: Shell, verbose: bool) -> MultiShell {
        MultiShell { out: out, err: err, verbose: verbose }
    }

    pub fn out(&mut self) -> &mut Shell {
        &mut self.out
    }

    pub fn err(&mut self) -> &mut Shell {
        &mut self.err
    }

    pub fn say<T: ToString>(&mut self, message: T, color: Color) -> IoResult<()> {
        self.out().say(message, color)
    }

    pub fn status<T: Show, U: Show>(&mut self, status: T, message: U) -> IoResult<()> {
        self.out().say_status(status, message, GREEN)
    }

    pub fn verbose(&mut self, callback: Callback) -> IoResult<()> {
        if self.verbose { return callback(self) }
        Ok(())
    }

    pub fn concise(&mut self, callback: Callback) -> IoResult<()> {
        if !self.verbose { return callback(self) }
        Ok(())
    }

    pub fn error<T: ToString>(&mut self, message: T) -> IoResult<()> {
        self.err().say(message, RED)
    }

    pub fn warn<T: ToString>(&mut self, message: T) -> IoResult<()> {
        self.err().say(message, YELLOW)
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn tag<T: Show, U: Show>(&mut self, tag: T, message: U) -> IoResult<()>{
        self.out().say_status(tag, message, BLACK)
    }

    pub fn header<T: Show>(&mut self, message: T) -> IoResult<()> {
        self.out().say_attr(message, BLACK, Attr::Underline(true), true)
    }

    pub fn comment<T: Show>(&mut self, message: T) -> IoResult<()> {
        self.out().say_attr(message, GREY, Attr::Dim, true)
    }

    pub fn tag_color<T: Show, U: Show>(&mut self, tag: T, message: U, color: Color) -> IoResult<()>{
        self.out().say_status(tag, message, color)
    }

    pub fn error_full(&mut self, e: &Error, mut show_cause: bool) -> IoResult<()>{
        try!(self.err().say_write(      "error: ", BRIGHT_RED));
        try!(self.err().say_attr(format!("{}", e.description()), BLACK, Attr::Bold, true));

        let mut detail = e.detail();

        if detail.is_some() {
            try!(self.err().say(format!("       {}", e.detail().unwrap()), BLACK));
        }
        let mut e = e;
        while show_cause {
            if e.cause().is_some() {
                e = e.cause().unwrap();
                try!(self.err().say_write(      "Caused by: ", BRIGHT_YELLOW));
                let mut lead = "";
                if detail.is_none() || detail.unwrap().as_slice() != e.description() {
                    try!(self.err().say(format!("{}", e.description()), BLACK));
                    lead = "           ";
                }
                detail = e.detail();
                if detail.is_some() {
                    try!(self.err().say(format!("{}{}", lead, e.detail().unwrap()), BLACK));
                }
            } else { show_cause = false; }
        }

        Ok(())
    }
}

pub type ShellCallback<'a> = |&mut Shell|:'a -> IoResult<()>;

impl Shell {
    pub fn create(out: Box<Writer + Send>, config: ShellConfig) -> Shell {
        let out = UghWhyIsThisNecessary { inner: out };
        if config.tty && config.color {
            let term = TerminfoTerminal::new(out);
            term.map(|t| Shell {
                terminal: Colored(t),
                config: config
            }).unwrap_or_else(|| {
                Shell { terminal: NoColor(box stderr()), config: config }
            })
        } else {
            Shell { terminal: NoColor(out.inner), config: config }
        }
    }

    pub fn verbose(&mut self, callback: ShellCallback) -> IoResult<()> {
        if self.config.verbose { return callback(self) }
        Ok(())
    }

    pub fn concise(&mut self, callback: ShellCallback) -> IoResult<()> {
        if !self.config.verbose { return callback(self) }
        Ok(())
    }

    pub fn say_write<T: ToString>(&mut self, message: T, color: Color) -> IoResult<()> {
        try!(self.reset());
        if color != BLACK { try!(self.fg(color)); }
        try!(self.write(message.to_string().as_slice().as_bytes()));
        try!(self.reset());
        try!(self.flush());
        Ok(())
    }

    pub fn say_attr<T: ToString>(&mut self, message: T, color: Color, attr: Attr, new_line: bool) -> IoResult<()> {
        try!(self.reset());
        try!(self.attr(attr));
        if color != BLACK { try!(self.fg(color)); }
        if new_line {
            try!(self.write_line(message.to_string().as_slice()));
        } else {
            try!(self.write(message.to_string().as_slice().as_bytes()));
        }
        try!(self.reset());
        try!(self.flush());
        Ok(())
    }

    pub fn say<T: ToString>(&mut self, message: T, color: Color) -> IoResult<()> {
        try!(self.reset());
        if color != BLACK { try!(self.fg(color)); }
        try!(self.write_line(message.to_string().as_slice()));
        try!(self.reset());
        try!(self.flush());
        Ok(())
    }

    pub fn say_status<T: Show, U: Show>(&mut self, status: T, message: U,
                                        color: Color) -> IoResult<()> {
        try!(self.reset());
        if color != BLACK { try!(self.fg(color)); }
        if self.supports_attr(Bold) { try!(self.attr(Bold)); }
        try!(self.write_str(format!("{:>12}", status).as_slice()));
        try!(self.reset());
        try!(self.write_line(format!(" {}", message).as_slice()));
        try!(self.flush());
        Ok(())
    }

    fn fg(&mut self, color: color::Color) -> IoResult<bool> {
        match self.terminal {
            Colored(ref mut c) => c.fg(color),
            NoColor(_) => Ok(false)
        }
    }

    fn attr(&mut self, attr: Attr) -> IoResult<bool> {
        match self.terminal {
            Colored(ref mut c) => c.attr(attr),
            NoColor(_) => Ok(false)
        }
    }

    fn supports_attr(&self, attr: Attr) -> bool {
        match self.terminal {
            Colored(ref c) => c.supports_attr(attr),
            NoColor(_) => false
        }
    }

    fn reset(&mut self) -> IoResult<()> {
        match self.terminal {
            Colored(ref mut c) => c.reset(),
            NoColor(_) => Ok(())
        }
    }
}

impl Writer for Shell {
    fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        match self.terminal {
            Colored(ref mut c) => c.write(buf),
            NoColor(ref mut n) => n.write(buf)
        }
    }

    fn flush(&mut self) -> IoResult<()> {
        match self.terminal {
            Colored(ref mut c) => c.flush(),
            NoColor(ref mut n) => n.flush()
        }
    }
}

impl Writer for UghWhyIsThisNecessary {
    fn write(&mut self, bytes: &[u8]) -> IoResult<()> {
        self.inner.write(bytes)
    }
}
