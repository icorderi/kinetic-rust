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

use protobuf::Message;
use crypto::digest::Digest;
use crypto::mac::Mac;
use std::{vec, io, collections};
use std::io::net::ip::ToSocketAddr;
use std::sync::{Mutex, Arc, Future};
use std::num::Int;
use core::{Command, Response, KineticResult};

static DEFAULT_MAX_PENDING: uint = 10;

#[experimental]
type KineticCommand = (Sender<KineticResponse>, ::proto::Message, ::proto::Command, Option<::std::vec::Vec<u8>>);

#[experimental]
type KineticResponse = (::proto::Message, ::proto::Command, ::std::vec::Vec<u8>);

#[experimental]
struct KineticChannel {
    stream: io::TcpStream,
    writer_tx: ::std::comm::SyncSender<KineticCommand>
}

#[experimental]
impl Drop for KineticChannel {
    #[experimental]
    fn drop(&mut self) {
        // TODO: mark somewhere that we are closing it
        self.stream.close_read().unwrap();
    }
}

#[experimental]
impl KineticChannel {

    #[experimental]
    fn connect<A: ToSocketAddr>(addr: A, max_pending: uint) -> KineticResult<KineticChannel> {
        let mut s = try!(io::TcpStream::connect(addr));
        try!(s.set_nodelay(true));

        // Handshake
        let (_, cmd, _) = ::network::recv(&mut s).unwrap();
        let connection_id = cmd.get_header().get_connectionID();

        // Other state like pending requests...
        let pending_mutex = Arc::new(Mutex::new(collections::HashMap::with_capacity(max_pending)));

        // reader
        let mut reader = s.clone();
        let pending_mutex_reader = pending_mutex.clone();
        spawn(proc() {
            let pending_mutex = pending_mutex_reader;
            loop {
                let r = ::network::recv(&mut reader);
                if r.is_err() { break } ; // TODO: this is correct only if we closed it

                let (msg, cmd, value) = r.unwrap();

                // TODO : add support for unsolicited status
                if msg.get_authType() != ::proto::Message_AuthType::UNSOLICITEDSTATUS
                {
                    let ack = cmd.get_header().get_ackSequence();
                    let req: Option<Sender<KineticResponse>>;
                    // lock the pendings and grab the request that matches the ACK
                    {
                        let mut pending = pending_mutex.lock();
                        // *remove* returns the value if it was there
                        req = pending.remove(&ack);
                    }

                    match req {
                        None => println!("No match for ack: {} found.", ack), // TODO: error
                        Some(callback) => callback.send((msg, cmd, value))
                    }
                }
            }
        });

        // writer
        let (w_tx, w_rx): (_, Receiver<KineticCommand>) = sync_channel(max_pending); // TODO: move to argument
        let mut writer = s.clone();
        let pending_mutex_writer = pending_mutex.clone();
        let key = "asdfasdf".as_bytes();
        spawn(proc() {
            let pending_mutex = pending_mutex_writer;
            let mut seq = 0;

            let mut buffer: [u8, ..4];
            for (callback, mut msg, mut cmd, value) in w_rx.iter(){
                cmd.mut_header().set_sequence(seq);
                cmd.mut_header().set_connectionID(connection_id);

                let cmd_bytes = cmd.write_to_bytes().unwrap();

                // Set message authentication
                msg.set_authType(::proto::Message_AuthType::HMACAUTH);
                let mut auth = ::proto::Message_HMACauth::new();
                auth.set_identity(1); // TODO: move to attribute

                // Calculate hmac_sha1 of the command
                let mut hmac = ::crypto::hmac::Hmac::new (::crypto::sha1::Sha1::new(), key); // TODO: move to attribute

                buffer = unsafe { ::std::mem::transmute((cmd_bytes.len() as u32).to_be()) };

                hmac.input(&buffer);
                hmac.input(cmd_bytes.as_slice());

                auth.set_hmac(hmac.result().code().to_vec()); // TODO: Code is backed by a Vec, we should have a method to get it.
                msg.set_hmacAuth(auth);

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

        Ok(KineticChannel { stream: s, writer_tx: w_tx})
    }

    #[experimental]
    fn send(&self, p: KineticCommand) {
        self.writer_tx.send(p);
    }
}


#[experimental]
pub struct Client {
    channel: KineticChannel,
    cluster_version: i64
}


/// Kinetic protocol client
/// # Example
/// ```no_run
/// use kinetic::Client;
/// use kinetic::commands::Put;
///
/// let c = Client::connect("127.0.0.1:8123").unwrap();
/// c.send(Put { key: "hello".as_bytes().to_vec(), value: "world".as_bytes().to_vec() }).unwrap();
/// ```
#[unstable]
impl Client {

    #[stable]
    pub fn connect<A: ToSocketAddr>(addr: A) -> KineticResult<Client> {
        let c = try!(KineticChannel::connect(addr, DEFAULT_MAX_PENDING));
        Ok( Client { channel: c,
                     cluster_version: 0 })
    }

    /// Sends commands to target device an waits for response
    #[stable]
    pub fn send<R : Response, C: Command<R>> (&self, cmd: C) -> KineticResult<R> {
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
        let (tx, rx) = channel();
        self.channel.send((tx, msg, cmd, value));

        // Receive response
        let (msg, cmd, value) = rx.recv();

        // create response for the command
        let r:KineticResult<R> = Response::from_proto(msg, cmd, value);
        r // return it
    }

    // Returns a Future<T> instead of waiting for the response
    #[experimental]
    pub fn send_future<R : Response, C: Command<R>> (&self, cmd: C) -> Future<KineticResult<R>> {
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
        let (tx, rx) = channel();
        self.channel.send((tx, msg, cmd, value));

        Future::spawn(proc() {
            // Receive response
            let (msg, cmd, value) = rx.recv();

            let r:KineticResult<R> = Response::from_proto(msg, cmd, value);
            r // return it
        })
    }

}

// #[bench]
// fn put_one_megabyte(b: &mut Bencher) {
//     let c = Client::connect("127.0.0.1:8123").unwrap();

//     b.iter(|| {
//         let data = vec::Vec::from_elem(1024*1024, 0u8); // 1 MB
//         c.put("bench".as_bytes().to_vec(), data).unwrap().unwrap();
//     });
//     b.bytes = 1024*1024;
// }

// #[bench]
// fn put_ten_megabytes(b: &mut Bencher) {
//     let c = Client::connect("127.0.0.1:8123").unwrap();

//     let items = 10i;
//     b.iter(|| {
//         let data = Arc::new(box [0u8,..1024*1024]); // 1 MB
//         let mut responses = vec::Vec::with_capacity(items as uint);
//         for i in range(0i, items) {
//             let data = data.clone().to_vec();
//             let r = c.put(format!("bench.{}", i).as_bytes().to_vec(), data);
//             responses.push(r);
//         }
//         // wait on all
//         for r in responses.into_iter() {
//             r.unwrap().unwrap();
//         }
//     });
//     b.bytes = 1024*1024*10;
// }
