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
use ::libc::{c_void, c_char, c_int, c_uint};

#[allow(dead_code)]
#[link(name = "pmem")]
extern "C" {
    // Most commonly used functions:

    pub fn pmem_is_pmem(addr: *const c_void, len: size_t) -> c_int;
    pub fn pmem_persist(addr: *const c_void, len: size_t);
    pub fn pmem_msync(addr: *const c_void, len: size_t) -> c_int;
    pub fn pmem_map_file(path: *const c_char,
                         len: size_t,
                         flags: c_int,
                         mode: mode_t,
                         mapped_lenp: *mut size_t,
                         is_pmemp: *mut c_int) -> *mut c_void;
    pub fn pmem_unmap(addr: *mut c_void, len: size_t) -> c_int;

    // Partial flushing operations:

    pub fn pmem_flush(addr: *const c_void, len: size_t);
    pub fn pmem_drain();
    pub fn pmem_has_hw_drain() -> c_int;

    // Copying to persistent memory:

    pub fn pmem_memmove_persist(pmemdest: *mut c_void, src: *const c_void, len: size_t) -> *mut c_void;
    pub fn pmem_memcpy_persist(pmemdest: *mut c_void, src: *const c_void, len: size_t) -> *mut c_void;
    pub fn pmem_memset_persist(pmemdest: *mut c_void, c: c_int, len: size_t) -> *mut c_void;
    pub fn pmem_memmove_nodrain(pmemdest: *mut c_void, src: *const c_void, len: size_t) -> *mut c_void;
    pub fn pmem_memcpy_nodrain(pmemdest: *mut c_void, src: *const c_void, len: size_t) -> *mut c_void;
    pub fn pmem_memset_nodrain(pmemdest: *mut c_void, c: c_int, len: size_t) -> *mut c_void;

    // Library API versioning:

    pub fn pmem_check_version(major_required: c_uint, minor_required: c_uint) -> *const c_char;
}
