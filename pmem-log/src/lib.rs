//! A pmem-resident log file.
//! This is useful for programs like databases that append frequently to a log file.
//!
//! > This is **not** an official port of the NVM Library.
//! >
//! > The official **libpmemlog** documentation can be found at: [http://pmem.io/nvml/libpmemlog/](http://pmem.io/nvml/libpmemlog/)

extern crate pmemlog_sys;
extern crate libc;

pub mod log;

pub use log::Log;
