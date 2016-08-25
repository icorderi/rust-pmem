//! Arrays of pmem-resident blocks, all the same size, that are atomically updated.
//! For example, a program keeping a cache of fixed-size objects in pmem might find this library useful.
//!
//! > This is **not** an official port of the NVM Library.
//! >
//! > The official **libpmemblk** documentation can be found at: [http://pmem.io/nvml/libpmemblk/](http://pmem.io/nvml/libpmemblk/)

extern crate pmemblk_sys;
extern crate libc;

pub mod blkpool;

pub use blkpool::BlkPool;
