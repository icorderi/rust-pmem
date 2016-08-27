//! The functions in this section provide optimized copying to persistent memory
//!
//! The `copy`, `copy_nooverlapping`, and `write_bytes`, provide the same memory copying as
//! `memmove(3)`, `memcpy(3)`, and `memset(3)`, and ensure that the result has been flushed to persistence before returning.
//!
//! > **Warning:** Using these functions where `is_pmem(1)` returns false may not do anything useful.
//! >              Use the normal libc functions in that case.

use ::std::mem;

use ::libc::{c_void, c_int};
use ::libc::size_t;
use ::pmem_sys as ffi;

/// Copies `count * size_of<T>` bytes from `src` to `pmemdest`. The source and destination may overlap.
///
/// `copy` is semantically equivalent to C's `memmove` and is optimized for persitent memory.
///
/// Ensures that the result has been flushed to persistence before returning.
///
/// # Safety
///
/// Care must be taken with the ownership of `src` and `pmemdest`.
/// This method semantically moves the values of `src` into `pmemdest`.
/// However it does not drop the contents of `pmemdest`, or prevent the contents of `src` from being dropped or used.
pub unsafe fn copy<T>(src: *const T, pmemdest: *mut T, count: usize) {
    let len = mem::size_of::<T>() * count;
    ffi::pmem_memmove_persist(pmemdest as *mut c_void, src as *const c_void, len as size_t);
}

/// Copies `count * size_of<T>` bytes from `src` to `pmemdest`. The source and destination may _not_ overlap.
///
/// `copy_nooverlapping` is semantically equivalent to C's `memcpy` and is optimized for persitent memory.
///
/// Ensures that the result has been flushed to persistence before returning.
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
    ffi::pmem_memcpy_persist(pmemdest as *mut c_void, src as *const c_void, len as size_t);
}

/// Invokes memset on the specified pointer, setting `count * size_of::<T>()` bytes of memory starting at `pmemdest` to `val`.
///
/// Ensures that the result has been flushed to persistence before returning.
pub unsafe fn write_bytes<T>(pmemdest: *mut T, val: u8, count: usize) {
    let len = mem::size_of::<T>() * count;
    ffi::pmem_memset_persist(pmemdest as *mut c_void, val as c_int, len as size_t);
}

/// Overwrites a memory location with the given value without reading or dropping the old value.
///
/// Ensures that the result has been flushed to persistence before returning.
///
/// # Safety
///
/// This operation is marked unsafe because it accepts a raw pointer.
///
/// It does not drop the contents of dst. This is safe, but it could leak allocations or resources,
/// so care must be taken not to overwrite an object that should be dropped.
///
/// This is appropriate for initializing uninitialized memory, or overwriting memory that has previously been read from.
pub unsafe fn write<T>(pmemdest: *mut T, val: T) {
    copy_nooverlapping(&val as *const _, pmemdest, 1)
}
