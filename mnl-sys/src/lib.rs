// Copyright 2018 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Low level FFI bindings to [`libmnl`]. A minimalistic user-space library oriented to Netlink
//! developers. See [`mnl`] for a higher level safe abstraction.
//!
//! These bindings were generated with bindgen. See the `generate_bindings.sh` script in the
//! repository.
//!
//! # Linking to libmnl
//!
//! By default this crate uses pkg-config to find and link to [`libmnl`]. To manually configure
//! where to look for the library, set the environment variable `LIBMNL_LIB_DIR` to point to the
//! directory where `libmnl.so` or `libmnl.a` resides.
//!
//! [`libmnl`]: https://netfilter.org/projects/libmnl/
//! [`mnl`]: https://crates.io/crates/mnl

#![no_std]
#![cfg(target_os = "linux")]

extern crate libc;

#[allow(non_snake_case)]
pub fn MNL_SOCKET_BUFFER_SIZE() -> libc::c_long {
    const MAX: libc::c_long = 8192;
    let pagesize = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    ::core::cmp::min(pagesize, MAX)
}

#[allow(non_snake_case)]
pub fn MNL_ALIGN(len: i32) -> i32 {
    ((len) + MNL_ALIGNTO - 1) & !(MNL_ALIGNTO - 1)
}

#[allow(non_camel_case_types)]
mod bindings;
pub use bindings::*;
