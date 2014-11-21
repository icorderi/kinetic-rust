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

//! Kinetic protocol library in Rust

extern crate protobuf;
extern crate "rust-crypto" as rust_crypto;
extern crate test;
extern crate serialize;
extern crate docopt;

use docopt::Docopt;

use test::Bencher;
use protobuf::parse_from_reader;
use protobuf::parse_from_bytes;
use protobuf::Message;
use protobuf::error::ProtobufError;
use rust_crypto::digest::Digest;
use rust_crypto::mac::Mac;
use std::vec;
use std::io;
use std::error::FromError;
use std::io::net::ip::ToSocketAddr;
use std::collections;
use std::sync::{Mutex, Arc};
use std::sync::Future;
use std::time::duration::Duration;
use std::num::Int;

mod kinetic;

#[stable]
pub type KineticResult<T> = Result<T, KineticError>;

#[deriving(Show,Eq,PartialEq)]
#[unstable]
pub enum KineticError {
    IoError(io::IoError),
    ProtobufError(ProtobufError),
    InvalidMagicNumber,
    RemoteError(kinetic::Command_Status_StatusCode, String)
}

#[unstable]
impl FromError<io::IoError> for KineticError {
    fn from_error(err: io::IoError) -> KineticError {
        KineticError::IoError(err)
    }
}

#[unstable]
impl FromError<ProtobufError> for KineticError {
    fn from_error(err: ProtobufError) -> KineticError {
        KineticError::ProtobufError(err)
    }
}

#[experimental]
pub type KineticResponse = (kinetic::Message, kinetic::Command, vec::Vec<u8>);

#[experimental]
pub type KineticCommand = (Sender<KineticResponse>, kinetic::Message, kinetic::Command, vec::Vec<u8>);

#[unstable]
fn network_recv(stream: &mut io::Reader) -> KineticResult<KineticResponse> {
    let mut header = [0u8,..9];
    try!(stream.read_at_least(9, &mut header));

    let mut r = io::BufReader::new(&header);
    let magic_number = try!(r.read_byte());
    if magic_number != 70u8 { return Err(KineticError::InvalidMagicNumber); }
    let proto_length = try!(r.read_be_i32()) as uint;
    let value_length = try!(r.read_be_i32()) as uint;

    let proto_vec = try!(stream.read_exact(proto_length));

    let value = if value_length == 0 { vec![] }
                else { try!(stream.read_exact(value_length)) };

    let mut proto_reader = io::MemReader::new(proto_vec);

    let msg = try!(parse_from_reader::<kinetic::Message>(&mut proto_reader));
    let cmd = try!(parse_from_bytes::<kinetic::Command>(msg.get_commandBytes()));

    Ok((msg, cmd, value))
}

#[unstable]
fn network_send(stream: &mut io::Writer, proto: &kinetic::Message, value: &[u8]) -> KineticResult<()> {
    let s = proto.serialized_size();

    let mut hw = io::BufferedWriter::with_capacity(9u + s as uint, stream);
    try!(hw.write_u8(70u8)); // Magic number
    try!(hw.write_be_i32(s as i32));
    try!(hw.write_be_i32(value.len() as i32));
    try!(proto.write_to_writer(&mut hw));

    let stream = hw.unwrap();

    if value.len() > 0 {
        try!(stream.write(value.as_slice()));
        try!(stream.flush());
    }

    Ok(())
}

#[experimental]
pub struct KineticChannel {
    stream: io::TcpStream,
    writer_tx: std::comm::SyncSender<KineticCommand>
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
    pub fn connect<A: ToSocketAddr>(addr: A) -> KineticResult<KineticChannel> {
        let mut s = try!(io::TcpStream::connect(addr));
        try!(s.set_nodelay(true));

        // Handshake
        let (_, cmd, _) = network_recv(&mut s).unwrap();
        let connection_id = cmd.get_header().get_connectionID();

        // Other state like pending requests...
        let pending_mutex = Arc::new(Mutex::new(collections::HashMap::with_capacity(10)));

        // reader
        let mut reader = s.clone();
        let pending_mutex_reader = pending_mutex.clone();
        spawn(proc() {
            let pending_mutex = pending_mutex_reader;
            loop {
                let r = network_recv(&mut reader);
                if r.is_err() { break } ; // TODO: this is correct only if we closed it
                let (msg, cmd, value) = r.unwrap();
                if msg.get_authType() != kinetic::Message_AuthType::UNSOLICITEDSTATUS
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
                msg.set_authType(kinetic::Message_AuthType::HMACAUTH);
                let mut auth = kinetic::Message_HMACauth::new();
                auth.set_identity(1); // TODO: move to attribute

                // Calculate hmac_sha1 of the command
                let mut hmac = rust_crypto::hmac::Hmac::new(rust_crypto::sha1::Sha1::new(), key); // TODO: move to attribute

                buffer = unsafe { std::mem::transmute((cmd_bytes.len() as u32).to_be()) };

                hmac.input(&buffer);
                hmac.input(cmd_bytes.as_slice());

                auth.set_hmac(hmac.result().code().to_vec());
                msg.set_hmacAuth(auth);


                msg.set_commandBytes(cmd_bytes);

                {
                    let mut pending = pending_mutex.lock();
                    pending.insert(seq, callback);
                }
                network_send(&mut writer, &msg, value.as_slice()).unwrap();
                seq += 1;
            }
        });

        Ok(KineticChannel { stream: s, writer_tx: w_tx})
    }

    #[experimental]
    pub fn send(&self, p: KineticCommand) {
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

    #[experimental]
    pub fn connect<A: ToSocketAddr>(addr: A) -> KineticResult<Client> {
        let c = try!(KineticChannel::connect(addr));
        Ok( Client { channel: c,
                     cluster_version: 0 })
    }

    /// Sends a Put command for the given key/value pair to the target device
    #[experimental]
    pub fn put(&self, key: vec::Vec<u8>, value: vec::Vec<u8>) -> Future<KineticResult<()>> {
        let mut cmd = kinetic::Command::new();

        // fill header
        let mut header = kinetic::Command_Header::new();
        header.set_clusterVersion(self.cluster_version);

        // Set command type to put
        header.set_messageType(kinetic::Command_MessageType::PUT);

        cmd.set_header(header);

        // Build the actual command
        let mut kv = kinetic::Command_KeyValue::new();
        kv.set_key(key);
        kv.set_synchronization(kinetic::Command_Synchronization::WRITEBACK);
        kv.set_force(true);
        kv.set_tag(vec![1,2,3,4]);
        kv.set_algorithm(kinetic::Command_Algorithm::SHA1);

        let mut body = kinetic::Command_Body::new();
        body.set_keyValue(kv);
        cmd.set_body(body);

        // Message wrapping the command
        let msg = kinetic::Message::new();

        // Send to device
        let (tx, rx) = channel();
        self.channel.send((tx, msg, cmd, value));

        Future::spawn(proc() {
            // Receive response
            let (_, cmd, _) = rx.recv();

            let status = cmd.get_status();
            if status.get_code() == kinetic::Command_Status_StatusCode::SUCCESS { Ok(()) }
            else { Err(KineticError::RemoteError(status.get_code(), String::from_str(status.get_statusMessage()))) } // TODO: return the entire status, not just the code
        })
    }

    #[experimental]
    pub fn get(&self, key: vec::Vec<u8>) -> Future<KineticResult<Vec<u8>>> {
        let mut cmd = kinetic::Command::new();

        // fill header
        let mut header = kinetic::Command_Header::new();
        header.set_clusterVersion(self.cluster_version);

        // Set command type to put
        header.set_messageType(kinetic::Command_MessageType::GET);

        cmd.set_header(header);

        // Build the actual command
        let mut kv = kinetic::Command_KeyValue::new();
        kv.set_key(key);

        let mut body = kinetic::Command_Body::new();
        body.set_keyValue(kv);
        cmd.set_body(body);

        // Message wrapping the command
        let msg = kinetic::Message::new();

        // Send to device
        let (tx, rx) = channel();
        self.channel.send((tx, msg, cmd, vec::Vec::new()));

        Future::spawn(proc() {
            // Receive response
            let (_, cmd, value) = rx.recv();

            let status = cmd.get_status();
            if status.get_code() == kinetic::Command_Status_StatusCode::SUCCESS { Ok(value) }
            else { Err(KineticError::RemoteError(status.get_code(), String::from_str(status.get_statusMessage()))) } // TODO: return the entire status, not just the code
        })
    }
}

#[bench]
fn put_one_megabyte(b: &mut Bencher) {
    let c = Client::connect("127.0.0.1:8123").unwrap();

    b.iter(|| {
        let data = vec::Vec::from_elem(1024*1024, 0u8); // 1 MB
        c.put("bench".as_bytes().to_vec(), data).unwrap().unwrap();
    });
    b.bytes = 1024*1024;
}

#[bench]
fn put_ten_megabytes(b: &mut Bencher) {
    let c = Client::connect("127.0.0.1:8123").unwrap();

    let items = 10i;
    b.iter(|| {
        let data = Arc::new(box [0u8,..1024*1024]); // 1 MB
        let mut responses = vec::Vec::with_capacity(items as uint);
        for i in range(0i, items) {
            let data = data.clone().to_vec();
            let r = c.put(format!("bench.{}", i).as_bytes().to_vec(), data);
            responses.push(r);
        }
        // wait on all
        for r in responses.into_iter() {
            r.unwrap().unwrap();
        }
    });
    b.bytes = 1024*1024*10;
}

// Write the Docopt usage string.
static USAGE: &'static str = "
Kinetic from Rust!

Usage: kinetic-rust write <target> [<count>]
       kinetic-rust read <target>
       kinetic-rust [options]

Options:
  -h, --help       Show this message.
  --version        Show the version of kinetic-rust.
";

#[deriving(Decodable, Show)]
struct Args {
   cmd_write: Option<WriteArgs>,
   cmd_read: Option<ReadArgs>,
   flag_help: bool,
   flag_version: bool
}

#[deriving(Decodable, Show)]
struct WriteArgs{
    arg_target: String,
    arg_count: Option<int>
}

#[deriving(Decodable, Show)]
struct ReadArgs{
    arg_target: String
}

pub fn version() -> String {
    format!("{} {}", "kinetic-rust" ,match option_env!("CFG_VERSION") {
        Some(s) => s.to_string(),
        None => format!("{}.{}.{}{}",
                        env!("CARGO_PKG_VERSION_MAJOR"),
                        env!("CARGO_PKG_VERSION_MINOR"),
                        env!("CARGO_PKG_VERSION_PATCH"),
                        option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""))
    })
}

#[cfg(not(test))]
fn main() {

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.flag_help {
        println!("{}", USAGE);
        return;
    }

    if args.flag_version {
        println!("{}", version());
        return;
    }

    println!("{}", args);

    let cmd = args.cmd_write.unwrap();
    let target = cmd.arg_target;

    println!("Connecting to {}", target);

    let c = Client::connect(format!("{}:8123", target).as_slice()).unwrap();

    c.put("rust".as_bytes().to_vec(), "Hello from rust v0.0.4!".as_bytes().to_vec()).unwrap().unwrap();
    let v = c.get("rust".as_bytes().to_vec()).unwrap().unwrap();

    println!("Read back: {}", String::from_utf8(v).unwrap());

    let items = cmd.arg_count.unwrap_or(10i);
    // benchmark
    let d = Duration::span(|| {
        let data = Arc::new(box [0u8,..1024*1024]); // 1 MB
        let mut responses = vec::Vec::with_capacity(items as uint);
        for i in range(0i, items) {
            let data = data.clone().to_vec();
            let r = c.put(format!("opt-bench.{}", i).as_bytes().to_vec(), data);
            responses.push(r);
        }
        // wait on all
        for r in responses.into_iter() {
            r.unwrap().unwrap();
        }
    });
    let bw = items as f64 / (d.num_milliseconds() as f64 / 1000.0);
    println!("Benchmark took {}ms ({} MB/s)", d.num_milliseconds(), bw);
}
