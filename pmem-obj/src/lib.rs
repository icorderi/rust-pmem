//! # FFI bindings to **libpmemobj**
//!
//! The **pmem-obj** library provides a transactional object store, providing memory allocation,
//! transactions, and general facilities for persistent memory programming.
//!
//! Developers new to persistent memory probably want to start with this library.
//!
//! > This is **not** an official port of the NVM Library.
//! >
//! > The official **libpmemobj** documentation can be found at: [http://pmem.io/nvml/libpmemobj/](http://pmem.io/nvml/libpmemobj/)

extern crate pmemobj_sys;
extern crate libc;

pub mod objpool;

pub use objpool::ObjPool;
