//! libpmemobj bindings
//!
//! Official documentation at: http://pmem.io/nvml/libpmemobj/libpmemobj.3.html

extern crate pmem;
extern crate libc;

mod ffi;
pub mod blkpool;

pub use blkpool::BlkPool;
