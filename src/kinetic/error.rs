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

//! Kinetic error handling

use std::error::Error;
use std::io;
use protobuf::error::ProtobufError;
use proto::command::Status;
use std::fmt;
//use core::convert::From;

/// Enum representing possible Kinetic errors
#[derive(Debug)]
pub enum KineticError {
    IoError(io::Error),
    ProtobufError(ProtobufError),
    InvalidMagicNumber,
    RemoteError(Status)
}

impl fmt::Display for KineticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Error for KineticError {
    fn description(&self) -> &str {
        "Confused"
//        match *self {
//             KineticError::IoError(e) => e.description(),
//             KineticError::ProtobufError(e) => e.description(),
//             KineticError::InvalidMagicNumber => "Invalid magic number received.",
//             KineticError::RemoteError(ref status) => {
//                 let msg = status.get_statusMessage();
//                 if msg.len() > 0 { msg }
//                 else { "Kinetic remote error." }
//             },
//        }
    }

//     fn detail(&self) -> Option<String> {
//         match *self {
//             KineticError::IoError(ref err) => Some(err.description().to_string()),
//             KineticError::ProtobufError(ref err) => Some(err.description().to_string()),
//             KineticError::RemoteError(ref status) => {
//                 let x = format!("{}: {}", status.get_code(), status.get_statusMessage());
//                 if status.has_detailedMessage() {
//                     String::from_utf8(status.get_detailedMessage().to_vec()).ok() }
//                 else { None }
//             },
//             _ => None,
//         }
//     }

    fn cause(&self) -> Option<&Error> {
        match *self {
            KineticError::IoError(ref err) => Some(err as &Error),
            KineticError::ProtobufError(ref err) => Some(err as &Error),
            _ => None,
        }
    }
}

impl From<io::Error> for KineticError {
    #[inline]
    fn from(err: io::Error) -> KineticError {
        KineticError::IoError(err)
    }
}

impl<W> From<io::IntoInnerError<W>> for KineticError {
    #[inline]
    fn from(_: io::IntoInnerError<W>) -> KineticError {
        // TODO:: implement...
        //KineticError::IoError(err.error())
        KineticError::InvalidMagicNumber
    }
}

impl From<ProtobufError> for KineticError {
    #[inline]
    fn from(err: ProtobufError) -> KineticError {
        KineticError::ProtobufError(err)
    }
}

impl From<::byteorder::Error> for KineticError {
    #[inline]
    fn from(err: ::byteorder::Error) -> KineticError {
        match err {
            ::byteorder::Error::Io(err) => KineticError::IoError(err),
            ::byteorder::Error::UnexpectedEOF =>
                KineticError::IoError(io::Error::new(io::ErrorKind::InvalidInput, "Unexpected EOF"))
        }
    }
}

