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

//! Available Kinetic commands

pub use commands::get::Get;
pub use commands::put::Put;
pub use commands::get_log::GetLog;
pub use commands::delete::Delete;
pub use commands::get_key_range::GetKeyRange;
pub use commands::get_version::GetVersion;
pub use commands::get_next::GetNext;
pub use commands::get_previous::GetPrevious;
pub use commands::noop::Noop;

mod get;
mod put;
mod get_log;
mod delete;
mod get_key_range;
mod get_version;
mod get_next;
mod get_previous;
mod noop;

pub mod common {

    use std::vec;
    use proto::command;

    /// Version checking modes for operations
    #[unstable]
    pub enum Versioning {
        /// Match current version
        Match(vec::Vec<u8>),
        /// Force the operation without checks
        Force,
    }

    /// Point-to-point data integrity
    ///
    /// The drive can check the data integrity if the `algorithm` used is known.
    #[unstable]
    #[deriving(Show)]
    pub struct Integrity {
        pub tag : vec::Vec<u8>,
        pub algorithm: command::Algorithm,
    }


}
