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

//! Kinetic responses for available commands

pub use responses::get::GetResponse;
pub use responses::get_log::GetLogResponse;

mod get;
mod get_log;

#[unstable]     pub type PutResponse = ();
#[experimental] pub type DeleteResponse = ();

#[unstable]
impl ::core::Response for () {

    fn from_proto(_: ::proto::Message, mut cmd: ::proto::Command, _: ::std::vec::Vec<u8>)
        -> ::result::KineticResult<()> {

        let status = cmd.take_status();

        if status.get_code() == ::proto::StatusCode::SUCCESS {
            Ok(())
        } else {
            Err(::error::KineticError::RemoteError(status))
        }
    }

}
