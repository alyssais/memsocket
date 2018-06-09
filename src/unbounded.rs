// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use futures::stream::{Fuse, Stream};
use futures::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use std::io::{Cursor, Error, ErrorKind};
use tokio::prelude::*;

#[derive(Debug)]
pub struct UnboundedSocket {
    sender: Option<UnboundedSender<u8>>,
    receiver: Fuse<UnboundedReceiver<u8>>,
}

pub fn unbounded() -> (UnboundedSocket, UnboundedSocket) {
    use futures::sync::mpsc::unbounded;

    let (left_sender, right_receiver) = unbounded();
    let (right_sender, left_receiver) = unbounded();

    let left = UnboundedSocket {
        sender: Some(left_sender),
        receiver: left_receiver.fuse(),
    };
    let right = UnboundedSocket {
        sender: Some(right_sender),
        receiver: right_receiver.fuse(),
    };

    (left, right)
}

impl Read for UnboundedSocket {
    fn read(&mut self, bytes: &mut [u8]) -> Result<usize, Error> {
        let len = bytes.len();
        let mut cursor = Cursor::new(bytes);

        for _ in 0..len {
            match self
                .receiver
                .poll()
                .expect("Fuse<UnboundedReceiver>::poll never errors")
            {
                Async::Ready(Some(byte)) => cursor.write(&[byte])?,
                Async::NotReady => return Err(Error::new(ErrorKind::WouldBlock, "no data")),
                Async::Ready(None) => break,
            };
        }

        Ok(cursor.position() as usize)
    }
}

impl AsyncRead for UnboundedSocket {}

impl Write for UnboundedSocket {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        let sender = self
            .sender
            .as_mut()
            .ok_or_else(|| Error::new(ErrorKind::BrokenPipe, "closed"))?;

        for byte in bytes {
            sender
                .unbounded_send(*byte)
                .map_err(|_| Error::new(ErrorKind::BrokenPipe, "closed"))?;
        }

        Ok(bytes.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl AsyncWrite for UnboundedSocket {
    fn shutdown(&mut self) -> Result<Async<()>, Error> {
        self.sender = None;
        Ok(Async::Ready(()))
    }
}
