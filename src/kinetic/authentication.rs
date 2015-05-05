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

//! Kinetic authentication mechanisms

use std::vec;
use proto::message::AuthType;
use crypto::{hmac, sha1};
use crypto::digest::Digest;
use crypto::mac::Mac;

/// Kinetic authentication credentials
///
/// Kinetic authentication credentials include `Hmac` and `Pin`.
#[derive(Clone)]
pub enum Credentials {
    /// Authenticates a message with an `identity` and a `key`
    Hmac { identity: i64, key: vec::Vec<u8>, },
    /// Authenticates a message with an `pin`
    Pin { pin: vec::Vec<u8> }
}

impl Credentials {

    #[inline]
    fn calculate_hmac(key: &vec::Vec<u8>, data: &[u8]) -> vec::Vec<u8> {
        let mut hmac = hmac::Hmac::new(sha1::Sha1::new(), key.as_ref());

        let buffer: [u8;4] = unsafe { ::std::mem::transmute((data.len() as u32).to_be()) };

        hmac.input(&buffer);
        hmac.input(data);

        // TODO: Code is backed by a Vec, we should have an unwrap() method to get it.
        hmac.result().code().to_vec() // return
    }

    pub fn authenticate_proto(&self, command_bytes: &vec::Vec<u8>) -> ::proto::Message {
        let mut msg = ::proto::Message::new();

        match *self {
            Credentials::Hmac { identity, ref key } => {
                msg.set_authType(AuthType::HMACAUTH);
                let mut auth = ::proto::message::HmacAuth::new();
                auth.set_identity(identity);

                auth.set_hmac(Credentials::calculate_hmac(key, command_bytes.as_ref()));
                msg.set_hmacAuth(auth);
            },
            Credentials::Pin { ref pin } => {
                msg.set_authType(AuthType::PINAUTH);

                let mut pin_auth = ::proto::message::PinAuth::new();
                pin_auth.set_pin(pin.clone()); // FIXME: find a way not to clone the pin

                msg.set_pinAuth(pin_auth);
            }
        }

        msg // return
    }

    pub fn verify_proto(&self, msg: &::proto::Message) -> bool {
        match *self {
            Credentials::Hmac { identity, ref key } => {
                if msg.get_authType() != AuthType::HMACAUTH { return false; }
                if msg.get_hmacAuth().get_identity() != identity { return false; }

                let received_hmac = msg.get_hmacAuth().get_hmac();
                let calculated_hmac_vec = Credentials::calculate_hmac(key, msg.get_commandBytes());
                let calculated_hmac: &[u8] = calculated_hmac_vec.as_ref();
                (received_hmac == calculated_hmac)
            },
            Credentials::Pin { .. } => msg.get_authType() == AuthType::PINAUTH
        }
    }
}



impl ::std::default::Default for Credentials {

    fn default() -> Credentials {
        Credentials::Hmac { identity: 1,
                            key: "asdfasdf".as_bytes().to_vec(), }
    }

}
