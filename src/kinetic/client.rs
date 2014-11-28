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

#![unstable]

use std::io::net::ip::ToSocketAddr;
use std::sync::Future;
use core::{Command, Response};
use result::KineticResult;
use channel::Result;

static DEFAULT_MAX_PENDING: uint = 10;

/// The Kinetic device client
///
/// The Kinetic device client represents the main point of access for this library
///
/// # Example
/// ```no_run
/// use kinetic::Client;
/// use kinetic::commands::Put;
/// use std::default::Default;
///
/// let c = Client::connect("127.0.0.1:8123").unwrap();
/// c.send(Put { key: "hello".as_bytes().to_vec(),
///              value: "world".as_bytes().to_vec(),
///              ..Default::default() }).unwrap();
/// ```
///
#[unstable]
pub struct Client<T: ::channel::KineticChannel<U>,U> {
    channel: T,
    cluster_version: i64
}

#[unstable]
impl<T: ::channel::KineticChannel<U>, U> Client<T,U> {

    #[stable]
    #[inline]
    pub fn get_config<'r>(&'r self) -> &'r ::proto::command::log::Configuration {
        self.channel.get_configuration()
    }

    #[stable]
    #[inline]
    pub fn get_limits<'r>(&'r self) -> &'r ::proto::command::log::Limits {
        self.channel.get_limits()
    }

    #[inline]
    fn send_raw<R : Response, C: Command<R>> (&self, cmd: C) -> U {
        // build specific command
        let (mut cmd, value) = cmd.build_proto();

        // set extra client specific fields on the header
        {
            let mut h = cmd.mut_header();
            h.set_clusterVersion(self.cluster_version);
        }

        // Message wrapping the command
        let msg = ::proto::Message::new();

        // Send to device
        self.channel.send((msg, cmd, value)) //return
    }

    #[inline]
    fn receive_raw<R : Response> (token: U) -> KineticResult<R> {
        // Receive response
        let (msg, cmd, value) = ::channel::KineticChannel::receive(token);

        let r:KineticResult<R> = Response::from_proto(msg, cmd, value);

        r // return
    }

    /// Sends commands to target device an waits for response
    #[stable]
    #[inline]
    pub fn send<R : Response, C: Command<R>> (&self, cmd: C) -> KineticResult<R> {
        let token = self.send_raw(cmd);
        Client::receive_raw(token) // return
    }

}

#[experimental]
impl Client<::channel::AsyncChannel, Receiver<Result>> {

    #[unstable]
    #[inline]
    pub fn connect<A: ToSocketAddr>(addr: A) -> KineticResult<Client<::channel::AsyncChannel, Receiver<Result>>> {
        let c = try!(::channel::AsyncChannel::connect(addr, DEFAULT_MAX_PENDING));

        Ok( Client { channel: c,
                     cluster_version: 0, })
    }

    // Returns a Future<T> instead of waiting for the response
    #[experimental]
    #[inline]
    pub fn send_future<R : Response, C: Command<R>> (&self, cmd: C) -> Future<KineticResult<R>> {
        let rx = self.send_raw(cmd);
        Future::spawn(proc() { Client::receive_raw(rx) })
    }

}
