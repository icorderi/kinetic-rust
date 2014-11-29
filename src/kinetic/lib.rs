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

#![license = "MIT"]
#![crate_type = "lib"]
#![crate_name = "kinetic"]

#![experimental]

//! Kinetic protocol library in Rust

extern crate protobuf;
extern crate "rust-crypto" as crypto;
extern crate serialize;


pub use core::version;
pub use proto::version as protocol_version;
pub use core::{Command, Response};
pub use result::KineticResult;
pub use error::KineticError;
pub use client::Client;


pub mod commands;
pub mod responses;
pub mod error;
pub mod result;
pub mod proto;
pub mod channel;
pub mod authentication;

mod client;
mod core;
mod network;
