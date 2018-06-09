// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io::{Error, ErrorKind};
use tokio::prelude::*;
use Socket;

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

impl AsyncWrite for Socket {
    fn shutdown(&mut self) -> Result<Async<()>, Error> {
        self.write_closed.set(true);
        Ok(Async::Ready(()))
    }
}
