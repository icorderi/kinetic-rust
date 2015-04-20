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

use core::Command;
use std::vec;

/// Deletes the key
///
/// There are two kinds of delete operations: `Versioned` and `Forced`.
/// A `Versioned` delete will delete the key only if the version matches
/// while a `Forced` delete will delete the key without doing a version check.
///
/// # Return value
/// Deleting a key will return `Ok()` if the key was deleted or an `Err(...)` if
/// it failed to find the key or match the version.
///
/// # Examples
///
/// ## Deleting a specific version
/// ```no_run
/// use kinetic::Client;
/// use kinetic::commands::Delete;
///
/// let c = Client::new("127.0.0.1:8123").unwrap();
/// c.send(Delete::Versioned { key: "hello".as_bytes().to_vec(),
///                            version: "1.0.0".as_bytes().to_vec() }).unwrap();
/// ```
///
/// ## Forcing a delete
/// ```no_run
/// use kinetic::Client;
/// use kinetic::commands::Delete;
///
/// let c = Client::new("127.0.0.1:8123").unwrap();
/// c.send(Delete::Forced { key: "hello".as_bytes().to_vec() }).unwrap();
/// ```
///
/// # Performance notes
/// A `Forced` delete will perform faster on certaing devices given that
/// it does not require a metadata version check.
///
pub enum Delete {
    Versioned { key: vec::Vec<u8>,
                version: vec::Vec<u8>, },
    Forced { key: vec::Vec<u8>, },
}

impl Command<::responses::DeleteResponse> for Delete {

    fn build_proto(self) -> (::proto::Command, Option<vec::Vec<u8>>) {
        let mut cmd = ::proto::Command::new();
        let mut header = ::proto::command::Header::new();

        // Set command type
        header.set_messageType(::proto::command::MessageType::DELETE);
        cmd.set_header(header);

        // Build the actual command
        let mut kv = ::proto::command::KeyValue::new();
        match self {
            Delete::Versioned { key, version } => {
                kv.set_key(key);
                kv.set_dbVersion(version);
            },
            Delete::Forced { key } => {
                kv.set_key(key);
                kv.set_force(true);
            },
        }

        // Fill the body
        let mut body = ::proto::command::Body::new();
        body.set_keyValue(kv);
        cmd.set_body(body);

        (cmd, None) // return command
    }

}
