// Copyright 2018 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Safe abstraction for [`libmnl`]. A minimalistic user-space library oriented to Netlink
//! developers. See [`mnl-sys`] for the low level FFI bindings to the C library.
//!
//! This is work in progress and does not implement all of [`libmnl`] yet. Feel free to submit PRs
//! to support the parts you need!
//!
//! The initial focus here was to support sockets and the parsing of responses. So so far the best
//! covered parts are `mnl_socket_*` and `mnl_cb_run`. However the netlink messages are just
//! treated as raw byte buffers. It might make sense to add some abstraction struct at some point.
//!
//! # Prior/related work
//!
//! The [`crslmnl`] crate is another wrapper around [`libmnl`]. At this stage it is a far more
//! complete abstraction of the library than this is. There are a few reasons I decided to start a
//! new wrapper crate. I'm not going to go into details on why, but basically it did not support
//! part of my use-case and I was no fan of the design choices made. Instead of having local
//! definitions of all the Linux header constants I made sure to get everything needed for netlink
//! [merged into `libc`]. I also want a separate [`mnl-sys`] crate that is pure FFI bindings
//! without logic or abstractions.
//!
//! [`libmnl`]: https://netfilter.org/projects/libmnl/
//! [`mnl-sys`]: https://crates.io/crates/mnl-sys
//! [`crslmnl`]: https://crates.io/crates/crslmnl
//! [merged into `libc`]: https://github.com/rust-lang/libc/pull/922

#![cfg(target_os = "linux")]
#![deny(missing_docs)]

extern crate libc;
#[macro_use]
extern crate log;
pub extern crate mnl_sys;

/// Module for helper functions checking FFI return values for error codes.
/// "cvt" stands for "check value T", where T is the return value
mod cvt;

mod callback;
pub use callback::*;

mod socket;
pub use socket::*;
