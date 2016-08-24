//! libpmem bindings

extern crate libc;

use ::libc::{size_t, mode_t};
use ::libc::{c_void, c_char, c_int};

#[allow(dead_code)]
#[link(name = "pmem")]
extern "C" {
    // Most commonly used functions:
    fn pmem_is_pmem(addr: *const c_void, len: size_t) -> c_int;
    // void pmem_persist(const void *addr, size_t len);
    // int pmem_msync(const void *addr, size_t len);
    fn pmem_map_file(path: *const c_char,
                     len: size_t,
                     flags: c_int,
                     mode: mode_t,
                     mapped_lenp: *mut size_t,
                     is_pmemp: *mut c_int);
    fn pmem_unmap(addr: *mut c_void, len: size_t) -> c_int;
}
