# mnl-sys

Low level FFI bindings to [`libmnl`]. A minimalistic user-space library oriented to Netlink
developers. See [`mnl`] for a higher level safe abstraction.

These bindings were generated with bindgen. See the `generate_bindings.sh` script in the
repository.

## Linking to libmnl

By default this crate uses pkg-config to find and link to [`libmnl`]. To manually configure
where to look for the library, set the environment variable `LIBMNL_LIB_DIR` to point to the
directory where `libmnl.so` or `libmnl.a` resides.

[`libmnl`]: https://netfilter.org/projects/libmnl/
[`mnl`]: https://crates.io/crates/mnl

License: MIT/Apache-2.0
