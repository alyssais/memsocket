// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! memsocket provides a asynchronous socket-like interface for connecting
//! clients and servers in-memory.
//!
//! The [`bounded`](fn.bounded.html) [`unbounded`](fn.unbounded.html) methods
//! (analogous to bounded and unbounded
//! [Channels](https://docs.rs/futures/0.1.21/futures/sync/mpsc/index.html))
//! return a pair of objects, both of which are
//! [`AsyncRead`](https://docs.rs/tokio/0.1/tokio/io/trait.AsyncRead.html) and
//! [`AsyncWrite`](https://docs.rs/tokio/0.1/tokio/io/trait.AsyncWrite.html).
//! Data written to one can be read from the other, and vice versa,
//! thus emulating a socket interface.

extern crate futures;
extern crate tokio;

mod bounded;
mod compat;
mod unbounded;

pub use bounded::*;
pub use compat::*;
pub use unbounded::*;
