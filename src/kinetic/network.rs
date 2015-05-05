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

use protobuf::{parse_from_bytes, Message};
use std::io::{Read, Write, BufWriter};
use result::KineticResult;
use error::KineticError;
use std::io;
use byteorder::{ByteOrder, BigEndian, WriteBytesExt};

#[inline]
pub fn recv(stream: &mut Read) -> KineticResult<(::proto::Message, ::proto::Command, ::std::vec::Vec<u8>)> {
    let mut header = [0u8;9];
    try!(read_exact_into(stream, 9, &mut header));

    let magic_number = header[0];
    if magic_number != 70u8 { return Err(KineticError::InvalidMagicNumber); }

    let proto_length =  <BigEndian as ByteOrder>::read_i32(header[1..5].as_ref()) as usize;
    let value_length = <BigEndian as ByteOrder>::read_i32(header[5..9].as_ref()) as usize;

    let proto_vec = try!(read_exact(stream, proto_length));

    let value = if value_length == 0 { vec![] }
                else { try!(read_exact(stream, value_length)) };

    let msg = try!(parse_from_bytes::<::proto::Message>(proto_vec.as_ref()));
    let cmd = try!(parse_from_bytes::<::proto::Command>(msg.get_commandBytes()));

    Ok((msg, cmd, value))
}

#[inline]
pub fn send(stream: &mut Write, proto: &::proto::Message, value: &[u8]) -> KineticResult<()> {
    let s = proto.compute_size();

    let mut hw = BufWriter::with_capacity(9 + s as usize, stream);
    try!(hw.write_u8(70u8)); // Magic number
    try!(hw.write_i32::<BigEndian>(s as i32));
    try!(hw.write_i32::<BigEndian>(value.len() as i32));
    try!(proto.write_to_writer(&mut hw));

    let mut stream = try!(hw.into_inner());

    if value.len() > 0 {
        try!(stream.write(value));
        try!(stream.flush());
    }

    Ok(())
}

#[inline]
fn read_exact_into(reader: &mut Read, nbytes: usize, buff: &mut[u8]) -> io::Result<()> {
    let mut bytes_read = 0;

    while bytes_read < nbytes {
        let ln = try!(reader.read(buff[bytes_read..].as_mut()));
        bytes_read += ln;
    }

    Ok(())
}

#[inline]
fn read_exact(reader: &mut Read, n: usize) -> io::Result<Vec<u8>> {
    let mut buf = vec![];
    try!(io::copy(&mut reader.take(n as u64), &mut buf));
    Ok(buf)
}
