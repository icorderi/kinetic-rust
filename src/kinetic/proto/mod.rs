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

//! Public exports of the raw proto generated files

pub use proto::raw::Command_Status_StatusCode as StatusCode;
pub use proto::raw::Command;
pub use proto::raw::Message;

mod raw;

#[stable]
pub mod message {

    pub use proto::raw::Message_AuthType as AuthType;
    pub use proto::raw::Message_HMACauth as HmacAuth;
    pub use proto::raw::Message_PINauth as PinAuth;

}

#[stable]
pub mod command {

    pub use proto::raw::Command_Header as Header;
    pub use proto::raw::Command_MessageType as MessageType;
    pub use proto::raw::Command_Body as Body;
    pub use proto::raw::Command_Status as Status;

    pub use proto::raw::Command_KeyValue as KeyValue;
    pub use proto::raw::Command_Algorithm as Algorithm;
    pub use proto::raw::Command_Synchronization as Synchronization;

    pub use proto::raw::Command_Range as Range;

    pub use proto::raw::Command_GetLog as GetLog;
    pub use proto::raw::Command_GetLog_Type as LogType;

    pub use proto::raw::Command_PinOperation as PinOperation;
    pub use proto::raw::Command_PinOperation_PinOpType as  PinOpTypes;

    #[unstable]
    pub mod log {

        pub use proto::raw::Command_GetLog_Utilization as Utilization;
        pub use proto::raw::Command_GetLog_Temperature as Temperature;
        pub use proto::raw::Command_GetLog_Capacity as Capacity;
        pub use proto::raw::Command_GetLog_Configuration as Configuration;
        pub use proto::raw::Command_GetLog_Statistics as Statistics;
        pub use proto::raw::Command_GetLog_Limits as Limits;
        pub use proto::raw::Command_GetLog_Configuration_Interface as Interface;
    }
}

/// Returns the version of the Kinetic Protocol
#[frozen]
pub fn version() -> String {
    String::from_str(::proto::raw::Local::default_instance().get_protocolVersion())
}
