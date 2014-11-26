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

use core::Command;
use std::vec;


/// Stores the value asociated with the key
#[unstable]
pub struct Put {
    pub key: vec::Vec<u8>,
    pub value: vec::Vec<u8>
}

#[unstable]
impl Command<::responses::PutResponse> for Put {

    fn build_proto(self) -> (::proto::Command, Option<vec::Vec<u8>>) {
        let mut cmd = ::proto::Command::new();
        let mut header = ::proto::command::Header::new();

        // Set command type
        header.set_messageType(::proto::command::MessageType::PUT);
        cmd.set_header(header);

        // Build the actual command
        let mut kv = ::proto::command::KeyValue::new();
        kv.set_key(self.key);
        kv.set_synchronization(::proto::command::Synchronization::WRITEBACK);
        kv.set_force(true);
        kv.set_tag(vec![1,2,3,4]);
        kv.set_algorithm(::proto::command::Algorithm::SHA1);

        // Fill the body
        let mut body = ::proto::command::Body::new();
        body.set_keyValue(kv);
        cmd.set_body(body);

        (cmd, Some(self.value)) // return command
    }

}
