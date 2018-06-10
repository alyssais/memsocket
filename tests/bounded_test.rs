// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate memsocket;
extern crate tokio;

use std::str::from_utf8;
use tokio::prelude::*;

#[test]
fn write_then_read() {
    let (client, server) = memsocket::bounded(64);

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
fn read_then_write() {
    let (client, server) = memsocket::bounded(64);

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

#[test]
fn write_then_read_partial() {
    let (client, mut server) = memsocket::bounded(64);

    tokio::runtime::current_thread::Runtime::new()
        .unwrap()
        .block_on({
            tokio::io::write_all(client, "hello world")
                .and_then(|(client, _)| {
                    let mut buf = vec![0; 12];
                    server.read(&mut buf).map(|_| (client, buf))
                })
                .map(|(mut client, buf)| {
                    client.shutdown().unwrap();
                    buf
                })
                .map(|result| assert_eq!(from_utf8(&result), Ok("hello world\u{0}")))
                .map_err(|error| panic!("{:?}", error))
        })
        .unwrap();
}
