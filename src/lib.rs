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

extern crate tokio;

use std::cell::{Cell, RefCell};
use std::sync::Arc;

mod read;
mod write;

#[derive(Debug)]
pub struct Socket {
    read_buffer: Arc<RefCell<Vec<u8>>>,
    read_closed: Arc<Cell<bool>>,

    write_buffer: Arc<RefCell<Vec<u8>>>,
    write_closed: Arc<Cell<bool>>,
}

pub fn new() -> (Socket, Socket) {
    let left = Socket {
        read_buffer: Default::default(),
        read_closed: Arc::new(Cell::new(false)),

        write_buffer: Default::default(),
        write_closed: Arc::new(Cell::new(false)),
    };

    let right = Socket {
        read_buffer: left.write_buffer.clone(),
        read_closed: left.write_closed.clone(),

        write_buffer: left.read_buffer.clone(),
        write_closed: left.read_closed.clone(),
    };

    (left, right)
}
