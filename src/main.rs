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

extern crate protobuf;
extern crate "rust-crypto" as rust_crypto;

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

mod kinetic;

#[stable]
pub type KineticResult<T> = Result<T, KineticError>;

#[deriving(Show,Eq,PartialEq)]
#[unstable]
pub enum KineticError {
    KineticIoError(io::IoError),
    KineticProtobufError(ProtobufError),
    KineticInvalidMagicNumber,
    KineticRemoteError(kinetic::Command_Status_StatusCode)
}

impl FromError<io::IoError> for KineticError {
    fn from_error(err: io::IoError) -> KineticError {
        KineticError::KineticIoError(err)
    }
}

impl FromError<ProtobufError> for KineticError {
    fn from_error(err: ProtobufError) -> KineticError {
        KineticError::KineticProtobufError(err)
    }
}

#[unstable]
pub type KineticResponse = (kinetic::Message, kinetic::Command, vec::Vec<u8>);

#[unstable]
pub type KineticCommand = (kinetic::Message, vec::Vec<u8>);

#[unstable]
fn receive(stream: &mut io::Reader) -> KineticResult<KineticResponse> {
    let mut header = [0u8,..9];
    try!(stream.read_at_least(9, header));

    let mut r = io::BufReader::new(header);
    let magic_number = try!(r.read_byte());
    if magic_number != 70u8 { return Err(KineticError::KineticInvalidMagicNumber); }
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
fn send(stream: &mut io::Writer, p: KineticCommand) -> KineticResult<()> {
    let (msg, value) = p;

    let s = msg.serialized_size();

    let mut hw = io::BufferedWriter::with_capacity(9u + s as uint, stream);
    try!(hw.write_u8(70u8)); // Magic number
    try!(hw.write_be_i32(s as i32));
    try!(hw.write_be_i32(value.len() as i32));
    try!(msg.write_to_writer(&mut hw));

    let stream = hw.unwrap();

    if value.len() > 0 {
        try!(stream.write(value.as_slice()));
        try!(stream.flush());
    }

    Ok(())
}

#[experimental]
pub struct KineticChannel {
    reader_rx: std::comm::Receiver<KineticResult<KineticResponse>>,
    writer_tx: std::comm::Sender<KineticCommand>
}

impl KineticChannel {
    pub fn connect<A: ToSocketAddr>(addr: A) -> KineticResult<KineticChannel> {
        let mut s = try!(io::TcpStream::connect(addr));
        try!(s.set_nodelay(true));

        // reader
        let (r_tx, r_rx) = channel();
        let mut reader = s.clone();
        spawn(proc() {
            loop {
                let resp = receive(&mut reader);
                r_tx.send(resp);
            }
        });

        // writer
        let (w_tx, w_rx) = channel();
        let mut writer = s.clone();
        spawn(proc() {
            for p in w_rx.iter(){
                send(&mut writer, p).unwrap();
            }
            println!("Writer closed.")
        });

        Ok(KineticChannel {reader_rx: r_rx , writer_tx: w_tx})
    }

    pub fn send(&self, p: KineticCommand) {
        self.writer_tx.send(p);
    }

    pub fn recv(&self) -> KineticResult<KineticResponse> {
        self.reader_rx.recv()
    }
}

#[unstable]
pub struct Client {
    channel: KineticChannel,
    connection_id: i64,
    identity: i64,
    key: vec::Vec<u8>,
    cluster_version: i64
}

impl Client {

    #[unstable]
    pub fn connect<A: ToSocketAddr>(addr: A) -> KineticResult<Client> {
        let c = try!(KineticChannel::connect(addr));
        let (_, cmd, _) = try!(c.recv());
        Ok( Client { channel: c,
                     cluster_version: 0,
                     connection_id: cmd.get_header().get_connectionID(),
                     identity: 1,
                     key: "asdfasdf".as_bytes().to_vec()})
    }

    #[unstable]
    pub fn put(&self, key: vec::Vec<u8>, value: vec::Vec<u8>) -> KineticResult<()> {
        let mut cmd = kinetic::Command::new();

        // fill header
        let mut header = kinetic::Command_Header::new();
        header.set_connectionID(self.connection_id);
        header.set_sequence(0); // TODO: move this to an attribute? Writer should be able to set it
        header.set_clusterVersion(self.cluster_version);

        // Set command type to put
        header.set_messageType(kinetic::Command_MessageType::Command_PUT);

        cmd.set_header(header);

        // Build the actual command
        let mut kv = kinetic::Command_KeyValue::new();
        kv.set_key(key);

        let mut body = kinetic::Command_Body::new();
        body.set_keyValue(kv);
        cmd.set_body(body);

        let cmd_bytes = cmd.write_to_bytes().unwrap();

        // Message wrapping the command
        let mut msg = kinetic::Message::new();

        // Set message authentication
        msg.set_authType(kinetic::Message_AuthType::Message_HMACAUTH);
        let mut auth = kinetic::Message_HMACauth::new();
        auth.set_identity(self.identity);

        // Calculate Hmac_sha1 of the command
        let mut hmac = rust_crypto::hmac::Hmac::new(rust_crypto::sha1::Sha1::new(), self.key.as_slice());
        let mut w = io::MemWriter::with_capacity(4);
        w.write_be_u32(cmd_bytes.len().to_u32().unwrap()).unwrap();
        hmac.input(w.unwrap().as_slice());
        hmac.input(cmd_bytes.as_slice());

        auth.set_hmac(hmac.result().code().to_vec());
        msg.set_hmacAuth(auth);

        msg.set_commandBytes(cmd_bytes);

        // Send to device
        self.channel.send((msg, value));

        // TODO: this is wrong... use seq/ackSeq to match response to request

        // Receive response
        let (_, cmd, _) = try!(self.channel.recv());

        let status = cmd.get_status();
        if status.get_code() == kinetic::Command_Status_SUCCESS { Ok(()) }
        else { Err(KineticRemoteError(status.get_code())) } // TODO: return the entire status, not just the code
    }

    #[unstable]
    pub fn get(&self, key: vec::Vec<u8>) -> KineticResult<Vec<u8>> {
        let mut cmd = kinetic::Command::new();

        // fill header
        let mut header = kinetic::Command_Header::new();
        header.set_connectionID(self.connection_id);
        header.set_sequence(0); // TODO: move this to an attribute? Writer should be able to set it
        header.set_clusterVersion(self.cluster_version);

        // Set command type to put
        header.set_messageType(kinetic::Command_MessageType::Command_GET);

        cmd.set_header(header);

        // Build the actual command
        let mut kv = kinetic::Command_KeyValue::new();
        kv.set_key(key);

        let mut body = kinetic::Command_Body::new();
        body.set_keyValue(kv);
        cmd.set_body(body);

        let cmd_bytes = cmd.write_to_bytes().unwrap();

        // Message wrapping the command
        let mut msg = kinetic::Message::new();

        // Set message authentication
        msg.set_authType(kinetic::Message_AuthType::Message_HMACAUTH);
        let mut auth = kinetic::Message_HMACauth::new();
        auth.set_identity(self.identity);

        // Calculate Hmac_sha1 of the command
        let mut hmac = rust_crypto::hmac::Hmac::new(rust_crypto::sha1::Sha1::new(), self.key.as_slice());
        let mut w = io::MemWriter::with_capacity(4);
        w.write_be_u32(cmd_bytes.len().to_u32().unwrap()).unwrap();
        hmac.input(w.unwrap().as_slice());
        hmac.input(cmd_bytes.as_slice());

        auth.set_hmac(hmac.result().code().to_vec());
        msg.set_hmacAuth(auth);

        msg.set_commandBytes(cmd_bytes);

        // Send to device
        self.channel.send((msg, vec::Vec::new()));

        // TODO: this is wrong... use seq/ackSeq to match response to request

        // Receive response
        let (_, cmd, value) = try!(self.channel.recv());

        let status = cmd.get_status();
        if status.get_code() == kinetic::Command_Status_SUCCESS { Ok(value) }
        else { Err(KineticRemoteError(status.get_code())) } // TODO: return the entire status, not just the code
    }
}

#[cfg(not(test))]
fn main() {
    println!("Kinetic from Rust!")

    let c = Client::connect("127.0.0.1:8123").unwrap();

    c.put("rust".as_bytes().to_vec(), "Hello from rust v0.2!".as_bytes().to_vec()).unwrap();
    let v = c.get("rust".as_bytes().to_vec()).unwrap();

    println!("Read back: {}", String::from_utf8(v).unwrap());
}
