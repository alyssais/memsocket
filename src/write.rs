// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io::{Error, ErrorKind};
use tokio::prelude::*;
use Socket;

impl Write for Socket {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        self.sender
            .as_ref()
            .and_then(|sender| {
                bytes.iter().fold(Some(()), |acc, byte| {
                    acc.and_then(|_| sender.unbounded_send(*byte).ok())
                })
            })
            .map(|_| bytes.len())
            .ok_or_else(|| Error::new(ErrorKind::BrokenPipe, "closed"))
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl AsyncWrite for Socket {
    fn shutdown(&mut self) -> Result<Async<()>, Error> {
        self.sender = None;
        Ok(Async::Ready(()))
    }
}
