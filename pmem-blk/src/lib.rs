//! Arrays of pmem-resident blocks, all the same size, that are atomically updated.
//! For example, a program keeping a cache of fixed-size objects in pmem might find this library useful.
//!
//! > This is **not** an official port of the NVM Library.
//! >
//! > The official **libpmemblk** documentation can be found at: [http://pmem.io/nvml/libpmemblk/](http://pmem.io/nvml/libpmemblk/)

extern crate pmemblk_sys;
extern crate libc;

// Modules

pub mod blkpool;

// Re-exports

pub use blkpool::BlkPool;

// module - lib

use ::std::ffi::CStr;

use ::libc::c_uint;
use ::pmemblk_sys::{self as ffi};

/// Checks the version of the **libpmemblk** library
pub fn check_version(major_required: usize, minor_required: usize) -> Result<(), String> {
    unsafe {
        let reason_p = ffi::pmemblk_check_version(major_required as c_uint, minor_required as c_uint);
        if !reason_p.is_null() {
            let reason = CStr::from_ptr(reason_p).to_owned().into_string().unwrap();
            Err(reason)
        } else {
            Ok(())
        }
    }
}
