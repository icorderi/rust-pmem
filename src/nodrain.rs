//! The functions in this section provide optimized copying to persistent memory _without_ draining
//!
//! The `copy`, `copy_nooverlapping`, and `write_bytes` are similar to the ones on the `persistent` module,
//! except they skip the final `pmem::drain()` step.
//! This allows applications to optimize cases where several ranges are being copied to persistent memory,
//! followed by a single call to `pmem::drain()`.
//!
//! > **Warning:** Using the functions in this module on a destination where `pmem::is_pmem()` returns false may not do anything useful.

use ::std::mem;

use ::libc::{c_void, c_int};
use ::libc::size_t;
use ::pmem_sys as ffi;

/// Copies `count * size_of<T>` bytes from `src` to `pmemdest`. The source and destination may overlap.
///
/// `copy` is semantically equivalent to C's `memmove` and is optimized for persitent memory.
///
/// # Safety
///
/// Care must be taken with the ownership of `src` and `pmemdest`.
/// This method semantically moves the values of `src` into `pmemdest`.
/// However it does not drop the contents of `pmemdest`, or prevent the contents of `src` from being dropped or used.
pub unsafe fn copy<T>(src: *const T, pmemdest: *mut T, count: usize) {
    let len = mem::size_of::<T>() * count;
    ffi::pmem_memmove_nodrain(pmemdest as *mut c_void, src as *const c_void, len as size_t);
}

/// Copies `count * size_of<T>` bytes from `src` to `pmemdest`. The source and destination may _not_ overlap.
///
/// `copy_nooverlapping` is semantically equivalent to C's `memcpy` and is optimized for persitent memory.
///
/// # Safety
///
/// Beyond requiring that the program must be allowed to access both regions of memory,
/// it is _Undefined Behavior_ for source and destination to overlap.
/// Care must also be taken with the ownership of `src` and `pmemdest`.
/// This method semantically moves the values of `src` into `pmemdest`.
/// However it does not drop the contents of `pmemdest`, or prevent the contents of `src` from being dropped or used.
pub unsafe fn copy_nooverlapping<T>(src: *const T, pmemdest: *mut T, count: usize) {
    let len = mem::size_of::<T>() * count;
    ffi::pmem_memcpy_nodrain(pmemdest as *mut c_void, src as *const c_void, len as size_t);
}

/// Invokes memset on the specified pointer, setting `count * size_of::<T>()` bytes of memory starting at `pmemdest` to `val`.
pub unsafe fn write_bytes<T>(pmemdest: *mut T, val: u8, count: usize) {
    let len = mem::size_of::<T>() * count;
    ffi::pmem_memset_nodrain(pmemdest as *mut c_void, val as c_int, len as size_t);
}
