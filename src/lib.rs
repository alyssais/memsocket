// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! memsocket provides a asynchronous socket-like interface for connecting
//! clients and servers in-memory.
//!
//! The [`new`](fn.new.html) method returns a pair of objects, both of which are
//! [`AsyncRead`](https://docs.rs/tokio/0.1/tokio/io/trait.AsyncRead.html) and
//! [`AsyncWrite`](https://docs.rs/tokio/0.1/tokio/io/trait.AsyncWrite.html).
//! Data written to one can be read from the other, and vice versa,
//! thus emulating a socket interface.

extern crate futures;
extern crate tokio;

mod read;
mod write;

use futures::stream::{Fuse, Stream};
use futures::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Debug)]
pub struct Socket {
    sender: Option<UnboundedSender<u8>>,
    receiver: Fuse<UnboundedReceiver<u8>>,
}

pub fn new() -> (Socket, Socket) {
    use futures::sync::mpsc::unbounded;

    let (left_sender, right_receiver) = unbounded();
    let (right_sender, left_receiver) = unbounded();

    let left = Socket {
        sender: Some(left_sender),
        receiver: left_receiver.fuse(),
    };
    let right = Socket {
        sender: Some(right_sender),
        receiver: right_receiver.fuse(),
    };

    (left, right)
}
