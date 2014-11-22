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
use std::sync::{Mutex, Arc};
use std::sync::Future;
use std::num::Int;
use core::{Command, Response, KineticResponse};
use result::KineticResult;
use error::KineticError;


#[experimental]
type KineticCommand = (Sender<KineticResponse>, ::proto::Message, ::proto::Command, vec::Vec<u8>);

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
    fn connect<A: ToSocketAddr>(addr: A) -> KineticResult<KineticChannel> {
        let mut s = try!(io::TcpStream::connect(addr));
        try!(s.set_nodelay(true));

        // Handshake
        let (_, cmd, _) = ::network::recv(&mut s).unwrap();
        let connection_id = cmd.get_header().get_connectionID();

        // Other state like pending requests...
        let pending_mutex = Arc::new(Mutex::new(collections::HashMap::with_capacity(10)));

        // reader
        let mut reader = s.clone();
        let pending_mutex_reader = pending_mutex.clone();
        spawn(proc() {
            let pending_mutex = pending_mutex_reader;
            loop {
                let r = ::network::recv(&mut reader);
                if r.is_err() { break } ; // TODO: this is correct only if we closed it
                let (msg, cmd, value) = r.unwrap();
                if msg.get_authType() != ::proto::Message_AuthType::UNSOLICITEDSTATUS
                {
                    let ack = cmd.get_header().get_ackSequence();
                    let req: Option<Sender<KineticResponse>>;
                    {
                        let mut pending = pending_mutex.lock();
                        req = pending.remove(&ack); // returns the value if it was there
                    }
                    match req {
                        None => println!("No match for ack: {} found.", ack),
                        Some(callback) => callback.send((msg, cmd, value))
                    }
                }
                //r_tx.send(Ok((msg, cmd, value)));
            }
        });

        // writer
        let (w_tx, w_rx): (_, Receiver<KineticCommand>) = sync_channel(10); // TODO: move to argument
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

                auth.set_hmac(hmac.result().code().to_vec());
                msg.set_hmacAuth(auth);


                msg.set_commandBytes(cmd_bytes);

                {
                    let mut pending = pending_mutex.lock();
                    pending.insert(seq, callback);
                }
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
/// let c = Client::connect("127.0.0.1:8123").unwrap();
/// c.put("hello".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap().unwrap();
/// ```
#[experimental]
impl Client {

    #[unstable]
    pub fn connect<A: ToSocketAddr>(addr: A) -> KineticResult<Client> {
        let c = try!(KineticChannel::connect(addr));
        Ok( Client { channel: c,
                     cluster_version: 0 })
    }

    #[experimental]
    pub fn send<R : Response, C: Command<R>> (&self, cmd: C) -> KineticResult<R> {
        Ok(cmd.get_response())
    }

    /// Sends a Put command for the given key/value pair to the target device
    #[experimental]
    fn put(&self, key: vec::Vec<u8>, value: vec::Vec<u8>) -> Future<KineticResult<()>> {
        let mut cmd = ::proto::Command::new();

        // fill header
        let mut header = ::proto::Command_Header::new();
        header.set_clusterVersion(self.cluster_version);

        // Set command type to put
        header.set_messageType(::proto::Command_MessageType::PUT);

        cmd.set_header(header);

        // Build the actual command
        let mut kv = ::proto::Command_KeyValue::new();
        kv.set_key(key);
        kv.set_synchronization(::proto::Command_Synchronization::WRITEBACK);
        kv.set_force(true);
        kv.set_tag(vec![1,2,3,4]);
        kv.set_algorithm(::proto::Command_Algorithm::SHA1);

        let mut body = ::proto::Command_Body::new();
        body.set_keyValue(kv);
        cmd.set_body(body);

        // Message wrapping the command
        let msg = ::proto::Message::new();

        // Send to device
        let (tx, rx) = channel();
        self.channel.send((tx, msg, cmd, value));

        Future::spawn(proc() {
            // Receive response
            let (_, cmd, _) = rx.recv();

            let status = cmd.get_status();
            if status.get_code() == ::proto::Command_Status_StatusCode::SUCCESS { Ok(()) }
            else { Err(KineticError::RemoteError(status.get_code(), String::from_str(status.get_statusMessage()))) } // TODO: return the entire status, not just the code
        })
    }

    #[experimental]
    fn get(&self, key: vec::Vec<u8>) -> Future<KineticResult<Vec<u8>>> {
        let mut cmd = ::proto::Command::new();

        // fill header
        let mut header = ::proto::Command_Header::new();
        header.set_clusterVersion(self.cluster_version);

        // Set command type to put
        header.set_messageType(::proto::Command_MessageType::GET);

        cmd.set_header(header);

        // Build the actual command
        let mut kv = ::proto::Command_KeyValue::new();
        kv.set_key(key);

        let mut body = ::proto::Command_Body::new();
        body.set_keyValue(kv);
        cmd.set_body(body);

        // Message wrapping the command
        let msg = ::proto::Message::new();

        // Send to device
        let (tx, rx) = channel();
        self.channel.send((tx, msg, cmd, vec::Vec::new()));

        Future::spawn(proc() {
            // Receive response
            let (_, cmd, value) = rx.recv();

            let status = cmd.get_status();
            if status.get_code() == ::proto::Command_Status_StatusCode::SUCCESS { Ok(value) }
            else { Err(KineticError::RemoteError(status.get_code(), String::from_str(status.get_statusMessage()))) } // TODO: return the entire status, not just the code
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
