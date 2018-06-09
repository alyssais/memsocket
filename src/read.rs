// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io::{Error, ErrorKind};
use tokio::prelude::*;
use Socket;

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

impl AsyncRead for Socket {}
