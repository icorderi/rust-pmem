//! # FFI bindings to **libpmemblk**
//!
//! The **libpmemblk** library supports arrays of pmem-resident blocks, all the same size, that are atomically updated.
//! For example, a program keeping a cache of fixed-size objects in pmem might find this library useful.
//!
//! > This is **not** an official port of the NVM Library.
//! >
//! > The official **libpmemblk** documentation can be found at: [http://pmem.io/nvml/libpmemblk/](http://pmem.io/nvml/libpmemblk/)

extern crate libc;

use ::libc::{size_t, mode_t};
use ::libc::{c_void, c_char, c_int, c_longlong, c_uint};

pub enum PMEMblkpool {}

#[allow(dead_code)]
#[link(name = "pmemblk")]
extern "C" {
    // Most commonly used functions:

    pub fn pmemblk_open(path: *const c_char, bsize: size_t) -> *mut PMEMblkpool;
    pub fn pmemblk_create(path: *const c_char, bsize: size_t, poolsize: size_t, mode: mode_t) -> *mut PMEMblkpool;
    pub fn pmemblk_close(pbp: *mut PMEMblkpool);
    pub fn pmemblk_bsize(pbp: *mut PMEMblkpool) -> size_t;
    pub fn pmemblk_nblock(pbp: *mut PMEMblkpool) -> size_t;
    pub fn pmemblk_read(pbp: *mut PMEMblkpool, buf: *mut c_void, blockno: c_longlong) -> c_int;
    pub fn pmemblk_write(pbp: *mut PMEMblkpool, buf: *const c_void, blockno: c_longlong) -> c_int;
    pub fn pmemblk_set_zero(pbp: *mut PMEMblkpool, blockno: c_longlong) -> c_int;
    pub fn pmemblk_set_error(pbp: *mut PMEMblkpool, blockno: c_longlong) -> c_int;

    // Library API versioning:

    pub fn pmemblk_check_version(major_required: c_uint, minor_required: c_uint) -> *const c_char;

    // Managing library behavior:

    pub fn pmemblk_check(path: *const c_char, bsize: size_t) -> c_int;

    // Error handling:

    pub fn pmemblk_errormsg() -> *const c_char;

    // Globals

    pub static PMEMBLK_MIN_POOL: size_t;
}
