//! Raw bindings generated from the pinned libghostty-vt C headers.
//!
//! Consumers must uphold the ownership, lifetime, thread-safety, and buffer
//! contracts documented in `vendor/ghostty/include/ghostty/vt`. This crate
//! provides no safe wrapper. C-only macros and static inline helpers are not
//! exported as Rust functions.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
