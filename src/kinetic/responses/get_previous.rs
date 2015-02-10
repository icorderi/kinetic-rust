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

use core::Response;
use result::KineticResult;
use error::KineticError;
use proto::{Message, Command};
use std::vec;
use commands::common::Integrity;

/// A `GetPrevious` command result
///
/// A `GetPrevious` command returns the value before the given key
#[unstable]
#[derive(Debug)]
pub struct GetPreviousResponse {
    pub value: vec::Vec<u8>,
    pub version: vec::Vec<u8>,
    pub integrity: Integrity,
}

#[unstable]
impl Response for GetPreviousResponse {

    fn from_proto(_: Message, mut cmd: Command, value: vec::Vec<u8>) -> KineticResult<GetPreviousResponse> {
        let status = cmd.take_status();

        if status.get_code() == ::proto::StatusCode::SUCCESS {
            let mut kv = cmd.take_body().take_keyValue();

            Ok(GetPreviousResponse { value: value,
                                     version: kv.take_dbVersion(),
                                     integrity: Integrity { tag: kv.take_tag(),
                                                            algorithm: kv.get_algorithm() }})
        } else {
            Err(KineticError::RemoteError(status))
        }
    }

}
