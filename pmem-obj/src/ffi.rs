//! #include <libpmemobj.h>
//!
//! cc -std=gnu99 ... -lpmemobj -lpmem

use ::libc::{size_t, mode_t};
use ::libc::{c_void, c_char, c_int};


pub enum PMEMobjpool {}


#[allow(dead_code)]
#[link(name = "pmemobj")]
extern "C" {
    pub fn pmemobj_open(path: *const c_char, layout: *const c_char) -> *mut PMEMobjpool;
    pub fn pmemobj_create(path: *const c_char,
                          layout: *const c_char,
                          poolsize: size_t,
                          mode: mode_t)
                          -> *mut PMEMobjpool;
    pub fn pmemobj_close(pop: *mut PMEMobjpool);
    pub fn pmemobj_memcpy_persist(pop: *mut PMEMobjpool,
                                  dest: *mut c_void,
                                  src: *const c_void,
                                  len: size_t);
    pub fn pmemobj_memset_persist(pop: *mut PMEMobjpool, dest: *mut c_void, c: c_int, len: size_t);
    pub fn pmemobj_persist(pop: *mut PMEMobjpool, addr: *const c_void, len: size_t);
    pub fn pmemobj_flush(pop: *mut PMEMobjpool, addr: *const c_void, len: size_t);
    pub fn pmemobj_drain(pop: *mut PMEMobjpool);
}
