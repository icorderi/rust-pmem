//! # FFI bindings to **libpmem**
//!
//! The **libpmem** library provides low level persistent memory support.
//! The libraries above are implemented using **libpmem**.
//! Developers wishing to roll their own persistent memory algorithms will find this library useful,
//! but most developers will likely use **libpmemobj** and let that library call **libpmem** for them.
//!
//! > This is **not** an official port of the NVM Library.
//! >
//! > The official **libpmem** documentation can be found at: [http://pmem.io/nvml/libpmem/](http://pmem.io/nvml/libpmem/)

extern crate libc;

use ::libc::{size_t, mode_t};
use ::libc::{c_void, c_char, c_int};

#[allow(dead_code)]
#[link(name = "pmem")]
extern "C" {
    // Most commonly used functions:
    pub fn pmem_is_pmem(addr: *const c_void, len: size_t) -> c_int;
    pub fn pmem_persist(addr: *const c_void, len: size_t) -> c_void;
    pub fn pmem_msync(addr: *const c_void, len: size_t) -> c_int;
    pub fn pmem_map_file(path: *const c_char,
                         len: size_t,
                         flags: c_int,
                         mode: mode_t,
                         mapped_lenp: *mut size_t,
                         is_pmemp: *mut c_int);
    pub fn pmem_unmap(addr: *mut c_void, len: size_t) -> c_int;
}
