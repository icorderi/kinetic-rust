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
//! Module representing raw communication channels with a kinetic device

use protobuf::Message;
use std::{vec, io, collections};
use std::io::net::ip::ToSocketAddr;
use std::sync::{Mutex, Arc};
use result::KineticResult;


#[unstable]
pub type Operation= (::authentication::Method, ::proto::Command, Option<::std::vec::Vec<u8>>);

#[unstable]
pub type Result = (::proto::Message, ::proto::Command, ::std::vec::Vec<u8>);

#[experimental]
pub trait KineticChannel<T> {

    #[experimental]
    fn is_closed(&self) -> bool;

    #[experimental]
    fn get_configuration<'r>(&'r self) -> &'r ::proto::command::log::Configuration;

    #[experimental]
    fn get_limits<'r>(&'r self) -> &'r ::proto::command::log::Limits;

    #[experimental]
    fn send(&self, op: Operation) -> T;

    #[experimental]
    fn receive(T) -> Result;

    #[experimental]
    fn get_unsolicited_receiver<'r>(&'r self) -> &'r ::std::comm::Receiver<Result>;

}

#[unstable]
pub struct AsyncChannel {
    stream: io::TcpStream,
    writer_tx: ::std::comm::SyncSender<(Operation,Sender<Result>)>,
    unsol_rx: ::std::comm::Receiver<Result>,
    closed: bool,
    configuration: ::proto::command::log::Configuration,
    limits: ::proto::command::log::Limits,
}

#[unstable]
impl Drop for AsyncChannel {

    #[unstable]
    #[inline]
    fn drop(&mut self) {
        self.closed = true;
        self.stream.close_read().unwrap();
    }

}


#[unstable]
impl AsyncChannel {

    #[unstable]
    pub fn new<A: ToSocketAddr>(addr: A, max_pending: uint) -> KineticResult<AsyncChannel> {
        let mut s = try!(io::TcpStream::connect(addr));
        try!(s.set_nodelay(true));

        // Handshake
        let (_, mut cmd, _) = ::network::recv(&mut s).unwrap();
        if cmd.get_status().get_code() != ::proto::StatusCode::SUCCESS {
            return Err(::error::KineticError::RemoteError(cmd.take_status()));
        }

        let connection_id = cmd.get_header().get_connectionID();
        let mut the_log = cmd.take_body().take_getLog();
        let configuration = the_log.take_configuration();
        let limits = the_log.take_limits();

        // Other state like pending requests...
        let pending_mutex = Arc::new(Mutex::new(collections::HashMap::with_capacity(max_pending)));

        // reader
        let mut reader = s.clone();
        let pending_mutex_reader = pending_mutex.clone();
        // for unsolicited status
        let (unsol_tx, unsol_rx) = channel();
        spawn(proc() {
            let pending_mutex = pending_mutex_reader;
            loop {
                let r = ::network::recv(&mut reader);
                if r.is_err() { break; } // FIXME: this is only ok if *we* closed it

                let (msg, cmd, value) = r.unwrap();

                match  msg.get_authType() {
                    ::proto::message::AuthType::UNSOLICITEDSTATUS => unsol_tx.send((msg, cmd, value)),
                    ::proto::message::AuthType::HMACAUTH => {
                        // FIXME: verify HMAC integrity
                        let ack = cmd.get_header().get_ackSequence();
                        let req: Option<Sender<Result>>;
                        // lock the pendings and grab the request that matches the ACK
                        {
                            let mut pending = pending_mutex.lock();
                            // *remove* returns the value if it was there
                            req = pending.remove(&ack);
                        }

                        match req {
                            // FIXME: What shjould we do if we get a result for an operation
                            //        we did not send?
                            None => println!("No match for ack: {} found.", ack),
                            Some(callback) => callback.send((msg, cmd, value))
                        }
                    },
                    ::proto::message::AuthType::PINAUTH => {
                        // FIXME: duplicate code with HMACAUTH except no need to veriy HMAC, refactor.
                        let ack = cmd.get_header().get_ackSequence();
                        let req: Option<Sender<Result>>;
                        // lock the pendings and grab the request that matches the ACK
                        {
                            let mut pending = pending_mutex.lock();
                            // *remove* returns the value if it was there
                            req = pending.remove(&ack);
                        }

                        match req {
                            // FIXME: What shjould we do if we get a result for an operation
                            //        we did not send?
                            None => println!("No match for ack: {} found.", ack),
                            Some(callback) => callback.send((msg, cmd, value))
                        }
                    },
                    ::proto::message::AuthType::INVALID_AUTH_TYPE =>
                        // FIXME: where do we send errors?
                        println!("Invalid authentication type."),
                }
            }
        });

        // writer
        let (w_tx, w_rx): (_, Receiver<(Operation,Sender<Result>)>) = sync_channel(max_pending);
        let mut writer = s.clone();
        let pending_mutex_writer = pending_mutex.clone();
        spawn(proc() {
            let pending_mutex = pending_mutex_writer;
            let mut seq = 0;

            for ((auth, mut cmd, value), callback) in w_rx.iter(){
                cmd.mut_header().set_sequence(seq);
                cmd.mut_header().set_connectionID(connection_id);

                let cmd_bytes = cmd.write_to_bytes().unwrap();

                let mut msg = auth.authenticate_proto(&cmd_bytes);
                msg.set_commandBytes(cmd_bytes);

                {
                    let mut pending = pending_mutex.lock();
                    pending.insert(seq, callback);
                }

                let value = value.unwrap_or(vec::Vec::new());
                ::network::send(&mut writer, &msg, value.as_slice()).unwrap();
                seq += 1;
            }
        });

        Ok(AsyncChannel { stream: s,
                          writer_tx: w_tx,
                          unsol_rx: unsol_rx,
                          configuration: configuration,
                          limits: limits,
                          closed: false, })
    }

}

#[experimental]
impl KineticChannel<Receiver<Result>> for AsyncChannel {

    #[stable]
    #[inline]
    fn is_closed(&self) -> bool { self.closed }

    #[experimental]
    #[inline]
    fn get_unsolicited_receiver<'r>(&'r self) -> &'r ::std::comm::Receiver<Result> {
        &self.unsol_rx
    }

    #[experimental]
    #[inline]
    fn get_configuration<'r>(&'r self) -> &'r ::proto::command::log::Configuration {
        &self.configuration
    }

    #[experimental]
    #[inline]
    fn get_limits<'r>(&'r self) -> &'r ::proto::command::log::Limits {
        &self.limits
    }

    #[unstable]
    #[inline]
    fn send(&self, op: Operation) -> Receiver<Result> {
        let (tx,rx) = channel();
        self.writer_tx.send((op,tx));
        rx //return rx
    }

    #[experimental]
    #[inline]
    fn receive(rx: Receiver<Result>) -> Result {
        rx.recv()
    }

}
