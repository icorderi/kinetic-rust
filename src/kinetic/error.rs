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

#![stable]

use std::error::Error;
use std::error::FromError;
use std::io::IoError;
use protobuf::error::ProtobufError;
use proto::command::Status;

#[stable]
#[deriving(Show)]
pub enum KineticError {
    IoError(IoError),
    ProtobufError(ProtobufError),
    InvalidMagicNumber,
    RemoteError(Status)
}

impl Error for KineticError {
    fn description(&self) -> &str {
        match *self {
            KineticError::IoError(_) => "An I/O error occurred",
            KineticError::ProtobufError(_) => "There was an error with the protobuf library",
            KineticError::InvalidMagicNumber => "Invalid magic number received",
            KineticError::RemoteError(ref status) =>
                format!("{}: {}", status.get_code(), status.get_statusMessage()).as_slice().clone(),
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            KineticError::IoError(ref err) => Some(err.description().to_string()),
            KineticError::ProtobufError(ref err) => Some(err.description().to_string()),
            KineticError::RemoteError(ref status) =>
                if status.has_detailedMessage() {
                    String::from_utf8(status.get_detailedMessage().to_vec()).ok() }
                else { None },
            _ => None,
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            KineticError::IoError(ref err) => Some(err as &Error),
            KineticError::ProtobufError(ref err) => Some(err as &Error),
            _ => None,
        }
    }
}

#[stable]
impl FromError<IoError> for KineticError {
    fn from_error(err: IoError) -> KineticError {
        KineticError::IoError(err)
    }
}

#[stable]
impl FromError<ProtobufError> for KineticError {
    fn from_error(err: ProtobufError) -> KineticError {
        KineticError::ProtobufError(err)
    }
}
