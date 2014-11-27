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
use proto::command;
use std::default::Default;

/// Stores the value asociated with the key
#[unstable]
pub struct Put {
    /// Key to store.
    pub key: vec::Vec<u8>,
    /// Value to store associated with the `key`.
    pub value: vec::Vec<u8>,
    /// The version of the `value` being stored.
    pub new_version: vec::Vec<u8>,
    /// The version of value currently stored in the device.
    ///
    /// If this version does not match the version currently in store,
    /// the operation will fail.
    pub current_version: vec::Vec<u8>,
    /// Force the put, skipping the version check
    pub force: bool,
    /// Write synchronization mode
    pub synchronization: command::Synchronization,
    /// End to end data integrity
    pub integrity: Option<Integrity>,
}

// FIXME: Rust doesn't seem to supprot having only **some** attributes with default values
impl Default for Put {
    fn default() -> Put {
        Put { key: vec![],
              value: vec![],
              new_version: vec![],
              current_version: vec![],
              force: false,
              synchronization: command::Synchronization::WRITEBACK,
              integrity: None }
    }
}

#[unstable]
#[deriving(Show)]
pub struct Integrity {
    pub tag : vec::Vec<u8>,
    pub algorithm: command::Algorithm,
}

#[unstable]
impl Command<::responses::PutResponse> for Put {

    fn build_proto(self) -> (::proto::Command, Option<vec::Vec<u8>>) {
        let mut cmd = ::proto::Command::new();
        let mut header = ::proto::command::Header::new();

        // Set command type
        header.set_messageType(command::MessageType::PUT);
        cmd.set_header(header);

        // Build the actual command
        let mut kv = command::KeyValue::new();
        kv.set_key(self.key);
        kv.set_dbVersion(self.current_version);
        kv.set_newVersion(self.new_version);
        kv.set_synchronization(self.synchronization);
        kv.set_force(self.force);
        match self.integrity {
            Some(integrity) => {
                kv.set_tag(integrity.tag);
                kv.set_algorithm(integrity.algorithm);
            },
            None => {
                // FIXME: this should probable be changed... the simulator barks if no tag is sent
                kv.set_tag(vec![1,2,3,4]);
                kv.set_algorithm(command::Algorithm::SHA1);
            },
        }

        // Fill the body
        let mut body = command::Body::new();
        body.set_keyValue(kv);
        cmd.set_body(body);

        (cmd, Some(self.value)) // return command
    }

}
