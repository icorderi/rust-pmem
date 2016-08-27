//! # FFI bindings to **libpmem**
//!
//! Rust bindings for the NVM Library [http://pmem.io](http://pmem.io)
//!
//! The **pmem** library provides low level persistent memory support.
//! Developers wishing to roll their own persistent memory algorithms will find this library useful,
//! but most developers will likely use **pmem-obj** and let that library call **pmem** for them.
//!
//! The interfaces in `pmem` are **non-transactional**. Crates like `pmem-obj` provide transactional
//! interfaces by building on these `pmem` functions.
//!
//! > This is **not** an official port of the NVM Library.
//! >
//! > The official **libpmem** documentation can be found at: [http://pmem.io/nvml/libpmem/](http://pmem.io/nvml/libpmem/)

#[macro_use]
extern crate bitflags;
extern crate pmem_sys;
extern crate libc;

// Modules

pub mod pmap;
pub mod ptr;
pub mod cell;
pub mod nodrain;

// lib module

use ::std::mem;
use ::std::io;
use ::std::ffi::CStr;

use ::libc::{c_void, c_uint};
use ::libc::size_t;
use ::pmem_sys as ffi;

/// Description of the last error
///
/// The error message is thread-local; errors encountered in one thread do not affect its value in other threads.
/// Its content is significant only when the return value of the immediately preceding
/// call to a function indicated an error.
pub fn errormsg() -> Option<String> {
    unsafe {
        let reason_p = ffi::pmem_errormsg();
        if !reason_p.is_null() {
            CStr::from_ptr(reason_p).to_owned().into_string().ok()
        } else {
            None
        }
    }
}

/// Checks if an entire object consists of persistent memory
/// If `true` then it is safe to use `persist(1)` and other related functions to make changes durable for that memory range.
///
/// The implementation of `is_pmem(1)` requires a non-trivial amount of work to determine if the given range is
/// entirely persistent memory. For this reason, it is better to call `is_pmem(1)` once when a range of memory is first encountered,
/// save the result, and use the saved result to determine whether `persist()` or `msync(1)` is appropriate for flushing changes to persistence.
/// Calling `is_pmem(1)` each time changes are flushed to persistence will not perform well.
///
/// > **Warning:** Using `persist(1)` on a range where `is_pmem(1)` returns `false` may not do anything useful -- use `msync(1)` instead.
pub fn is_pmem<T>(x: &T) -> bool {
    let len = mem::size_of_val(x);
    let r = unsafe { ffi::pmem_is_pmem(x as *const _ as *const c_void, len as size_t) };
    r > 0
}

/// Force an object to be stored durably in persistent memory.
///
/// This is equivalent to calling `msync(1)` but may be more optimal
/// and will avoid calling into the kernel if possible.
/// There are no alignment restrictions on the range the object is in,
/// but `persist(1)` may expand the range as necessary to meet platform alignment requirements.
///
/// > **Warning:** Like `msync()`, there is nothing atomic or transactional about this call.
/// > Any unwritten stores in the given range will be written, but some stores may have already been written
/// > by virtue of normal cache eviction/replacement policies. Correctly written code must not depend on stores
/// > waiting until `persist(1)` is called to become persistent -- they can become persistent at
/// > any time before `persist(1)` is called.
///
/// To create your own variations of `persist(1)`, see `flush(1)` and `drain()`. One can think of `persist(1)` as:
///
/// ```no_run
/// fn persist<T>(x: &T) {
///     // flush the processor caches
///     pmem::flush(x);
///
///     // wait for any pmem stores to drain from HW buffers
///     pmem::drain();
/// }
/// ```
pub fn persist<T>(x: &T) {
    let len = mem::size_of_val(x);
    unsafe { ffi::pmem_persist(x as *const _ as *const c_void, len as size_t) };
}

/// Forces any changes in an object to be stored durably.
///
/// This function works on either persistent memory or a memory mapped file on traditional storage.
/// `msync()` takes steps to ensure the alignment of addresses and lengths passed down meet the requirements of that system call.
/// It calls msync() with the MS_SYNC flag as described in msync(2).
/// Typically the application only checks for the existence of persistent memory once,
/// and then uses that result throughout the program, for example:
///
/// # Example
///
/// ```no_run
/// fn some_method<T>(x: &T) {
///     // do this call once, after the pmem is memory mapped
///     let is_pmem = pmem::is_pmem(x);
///
///     // ...make some changes to x
///
///     // make the changes durable
///     if is_pmem {
///         pmem::persist(&x);
///     } else{
///         pmem::msync(&x).unwrap();
///     }
/// }
/// ```
pub fn msync<T>(x: &T) -> Result<(), io::Error> {
    let len = mem::size_of::<T>();
    let r = unsafe { ffi::pmem_msync(x as *const _ as *const c_void, len as size_t) };
    if r == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn msync_unsized<T: ?Sized>(x: &T) -> Result<(), io::Error> {
    let len = mem::size_of_val(x);
    let r = unsafe { ffi::pmem_msync(x as *const _ as *const c_void, len as size_t) };
    if r == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

/// Flushes the processor caches
///
/// This is the first of two steps in flushing to persistence,
/// the second stpe is `drain()`.
///
/// These steps are performed together when `persist()` is called.
///
/// > Note that either of these steps may be unnecessary on a given platform, and
/// > the library knows how to check for that and do the right thing.
/// > For example, on Intel platforms, pmem_drain() is an empty function.
pub fn flush<T>(x: &T) {
    let len = mem::size_of_val(x);
    unsafe { ffi::pmem_flush(x as *const _ as *const c_void, len as size_t) };
}

/// Waits for any pmem stores to drain from HW buffers
///
/// This is the second of two steps in flushing to persistence,
/// the first step is `flush(1)`.
///
/// These steps are performed together when `persist(1)` is called.
///
/// > Note that either of these steps may be unnecessary on a given platform, and
/// > the library knows how to check for that and do the right thing.
/// > For example, on Intel platforms, pmem_drain() is an empty function.
pub fn drain() { unsafe { ffi::pmem_drain() }; }

/// Wether or not the machine supports an explicit hardware drain instruction for persistent memory.
///
/// On Intel processors with persistent memory, stores to persistent memory are considered persistent
/// once they are flushed from the CPU caches, so this function always returns false.
///
/// Despite that, programs using `flush(1)` to flush ranges of memory should still follow up by calling `drain()`
// once to ensure the flushes are complete. As mentioned above, `persist(1)` handles calling both `flush(1)` and `drain()`.
pub fn has_hw_drain() -> bool {
    let r = unsafe { ffi::pmem_has_hw_drain() };
    r > 0
}

/// Checks the version of the **libpmem** library
pub fn check_version(major_required: usize, minor_required: usize) -> Result<(), String> {
    unsafe {
        let reason_p = ffi::pmem_check_version(major_required as c_uint, minor_required as c_uint);
        if !reason_p.is_null() {
            let reason = CStr::from_ptr(reason_p).to_owned().into_string().unwrap();
            Err(reason)
        } else {
            Ok(())
        }
    }
}
