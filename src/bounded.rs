// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use futures::stream::{Fuse, Stream};
use futures::sync::mpsc::{Receiver, Sender};
use std::io::{Cursor, Error, ErrorKind};
use tokio::prelude::*;

#[derive(Debug)]
pub struct BoundedSocket {
    sender: Option<Sender<u8>>,
    receiver: Fuse<Receiver<u8>>,
}

pub fn bounded(buffer: usize) -> (BoundedSocket, BoundedSocket) {
    use futures::sync::mpsc::channel;

    let (left_sender, right_receiver) = channel(buffer);
    let (right_sender, left_receiver) = channel(buffer);

    let left = BoundedSocket {
        sender: Some(left_sender),
        receiver: left_receiver.fuse(),
    };
    let right = BoundedSocket {
        sender: Some(right_sender),
        receiver: right_receiver.fuse(),
    };

    (left, right)
}

impl Read for BoundedSocket {
    fn read(&mut self, bytes: &mut [u8]) -> Result<usize, Error> {
        let len = bytes.len();
        let mut cursor = Cursor::new(bytes);

        for _ in 0..len {
            match self
                .receiver
                .poll()
                .expect("Fuse<Receiver>::poll never errors")
            {
                Async::Ready(Some(byte)) => cursor.write(&[byte])?,
                Async::Ready(None) => break,
                Async::NotReady => if cursor.position() == 0 {
                    return Err(Error::new(ErrorKind::WouldBlock, "no data"));
                } else {
                    break;
                },
            };
        }

        Ok(cursor.position() as usize)
    }
}

impl AsyncRead for BoundedSocket {}

impl Write for BoundedSocket {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        let sender = self
            .sender
            .as_mut()
            .ok_or_else(|| Error::new(ErrorKind::BrokenPipe, "closed"))?;

        for byte in bytes {
            sender.try_send(*byte).map_err(|err| {
                if err.is_full() {
                    Error::new(ErrorKind::WouldBlock, "would block")
                } else {
                    Error::new(ErrorKind::BrokenPipe, "closed")
                }
            })?;
        }

        Ok(bytes.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl AsyncWrite for BoundedSocket {
    fn shutdown(&mut self) -> Result<Async<()>, Error> {
        self.sender = None;
        Ok(Async::Ready(()))
    }
}
