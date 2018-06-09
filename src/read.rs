// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io::{Cursor, Error, ErrorKind};
use tokio::prelude::*;
use Socket;

impl Read for Socket {
    fn read(&mut self, bytes: &mut [u8]) -> Result<usize, Error> {
        let len = bytes.len();
        let mut cursor = Cursor::new(bytes);

        for _ in 0..len {
            match self
                .receiver
                .poll()
                .expect("Fuse<UnboundedReceiver>::poll never errors")
            {
                Async::Ready(Some(byte)) => cursor
                    .write(&[byte])
                    .expect("<Cursor as Write>::write never errors"),
                Async::NotReady => return Err(Error::new(ErrorKind::WouldBlock, "no data")),
                Async::Ready(None) => break,
            };
        }

        Ok(cursor.position() as usize)
    }
}

impl AsyncRead for Socket {}
