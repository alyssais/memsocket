// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! memsocket provides a asynchronous socket-like interface for connecting
//! clients and servers in-memory.
//!
//! The [`new_pair`](fn.new_pair.html) method returns a pair of objects, both of which are
//! [`AsyncRead`](https://docs.rs/tokio/0.1/tokio/io/trait.AsyncRead.html) and
//! [`AsyncWrite`](https://docs.rs/tokio/0.1/tokio/io/trait.AsyncWrite.html).
//! Data written to one can be read from the other, and vice versa,
//! thus emulating a socket interface.

extern crate tokio;

use std::cell::{Cell, RefCell};
use std::io::{Error, ErrorKind};
use std::rc::Rc;
use tokio::prelude::*;

#[derive(Debug)]
pub struct Socket {
    read_buffer: Rc<RefCell<Vec<u8>>>,
    read_closed: Rc<Cell<bool>>,

    write_buffer: Rc<RefCell<Vec<u8>>>,
    write_closed: Rc<Cell<bool>>,
}

impl Read for Socket {
    fn read(&mut self, bytes: &mut [u8]) -> Result<usize, Error> {
        use std::cmp::min;

        let mut read_buffer = self.read_buffer.borrow_mut();

        if read_buffer.is_empty() {
            if self.read_closed.get() {
                Ok(0)
            } else {
                Err(Error::new(ErrorKind::WouldBlock, "no data"))
            }
        } else {
            let length_to_read = min(bytes.len(), read_buffer.len());
            bytes[..length_to_read].copy_from_slice(&read_buffer[..length_to_read]);
            read_buffer.drain(..length_to_read);
            Ok(length_to_read)
        }
    }
}

impl Write for Socket {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        if self.write_closed.get() {
            Err(Error::new(ErrorKind::BrokenPipe, "closed"))
        } else {
            self.write_buffer.borrow_mut().extend_from_slice(bytes);
            task::current().notify();
            Ok(bytes.len())
        }
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl AsyncRead for Socket {}

impl AsyncWrite for Socket {
    fn shutdown(&mut self) -> Result<Async<()>, Error> {
        self.write_closed.set(true);
        Ok(Async::Ready(()))
    }
}

pub fn new_pair() -> (Socket, Socket) {
    let left = Socket {
        read_buffer: Default::default(),
        read_closed: Rc::new(Cell::new(false)),

        write_buffer: Default::default(),
        write_closed: Rc::new(Cell::new(false)),
    };

    let right = Socket {
        read_buffer: left.write_buffer.clone(),
        read_closed: left.write_closed.clone(),

        write_buffer: left.read_buffer.clone(),
        write_closed: left.read_closed.clone(),
    };

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::new_pair;
    use std::str::from_utf8;
    use tokio::{self, prelude::*};

    #[test]
    fn async_write_then_read() {
        let (client, server) = new_pair();

        tokio::runtime::current_thread::Runtime::new()
            .unwrap()
            .block_on({
                tokio::io::write_all(client, "hello world")
                    .map(|(mut client, _)| client.shutdown())
                    .and_then(|_| tokio::io::read_to_end(server, vec![]))
                    .map(|(_, result)| assert_eq!(from_utf8(&result), Ok("hello world")))
                    .map_err(|error| panic!("{:?}", error))
            })
            .unwrap();
    }

    #[test]
    fn async_read_then_write() {
        let (client, server) = new_pair();

        tokio::runtime::current_thread::Runtime::new()
            .unwrap()
            .block_on({
                tokio::io::read_to_end(server, vec![])
                    .map(|(_, result)| assert_eq!(from_utf8(&result), Ok("hello world")))
                    .join(
                        tokio::io::write_all(client, "hello world")
                            .map(|(mut client, _)| client.shutdown()),
                    )
                    .map_err(|error| panic!("{:?}", error))
            })
            .unwrap();
    }
}
