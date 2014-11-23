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

use protobuf::{parse_from_reader, parse_from_bytes, Message};
use std::io;
use result::KineticResult;
use error::KineticError;


#[unstable]
pub fn recv(stream: &mut io::Reader) -> KineticResult<(::proto::Message, ::proto::Command, ::std::vec::Vec<u8>)> {
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

    let msg = try!(parse_from_reader::<::proto::Message>(&mut proto_reader));
    let cmd = try!(parse_from_bytes::<::proto::Command>(msg.get_commandBytes()));

    Ok((msg, cmd, value))
}

#[unstable]
pub fn send(stream: &mut io::Writer, proto: &::proto::Message, value: &[u8]) -> KineticResult<()> {
    let s = proto.serialized_size();

    let mut hw = io::BufferedWriter::with_capacity(9u + s as uint, stream);
    try!(hw.write_u8(70u8)); // Magic number
    try!(hw.write_be_i32(s as i32));
    try!(hw.write_be_i32(value.len() as i32));
    try!(proto.write_to_writer(&mut hw));

    let stream = hw.unwrap();

    if value.len() > 0 {
        try!(stream.write(value));
        try!(stream.flush());
    }

    Ok(())
}
