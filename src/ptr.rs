//! The functions in this section provide optimized copying to persistent memory
//!
//! The `copy`, `copy_nonoverlapping`, and `write_bytes`, provide the same memory copying as
//! `memmove(3)`, `memcpy(3)`, and `memset(3)`, and ensure that the result has been flushed to persistence before returning.
//!
//! > **Warning:** Using these functions where `is_pmem(1)` returns false may not do anything useful.
//! >              Use the normal libc functions in that case.

use ::std::mem;
use ::std::io;
use ::std::marker::PhantomData;

use ::libc::{c_void, c_int};
use ::libc::size_t;
use ::pmem_sys as ffi;

use pmap::PersistentMap;

/// Persistent memory virtual pointer
///
/// This pointer is safe to store.
#[derive(Copy, Clone)]
pub struct PmemConstVirtualPtr<T: ?Sized> {
    poolid: usize,
    offset: usize,
    _t: PhantomData<T>,
}

impl<T: ?Sized> PmemConstVirtualPtr<T> {
    pub unsafe fn new(poolid: usize, offset: usize) -> Self {
        assert!(poolid != 0, "Poolid 0 is reserved for null pointers");
        PmemConstVirtualPtr { poolid: poolid, offset: offset, _t: PhantomData }
    }

    pub fn null() -> Self { PmemConstVirtualPtr { poolid: 0, offset: 0, _t: PhantomData } }

    pub fn is_null(&self) -> bool { self.poolid == 0 }

    pub unsafe fn as_type<U>(&self) -> PmemConstVirtualPtr<U> {
        PmemConstVirtualPtr { poolid: self.poolid, offset: self.offset, _t: PhantomData }
    }

    pub unsafe fn as_mut(&self) -> PmemMutVirtualPtr<T> {
        PmemMutVirtualPtr { poolid: self.poolid, offset: self.offset, _t: PhantomData }
    }
}

impl<T> PmemConstVirtualPtr<T> {
    pub unsafe fn offset(&self, count: isize) -> Self {
        if self.is_null() {
            PmemConstVirtualPtr::null()
        } else {
            let new_offset = self.offset as isize + mem::size_of::<T>() as isize * count;
            PmemConstVirtualPtr { poolid: self.poolid, offset: new_offset as usize, _t: self._t }
        }
    }

    pub unsafe fn link(&self, pool: &PersistentMap) -> PmemConstPtr<T> {
        let new_virt = PmemConstVirtualPtr { poolid: self.poolid, offset: self.offset, _t: self._t };
        PmemConstPtr { virt: new_virt, pool: pool.as_ptr() }
    }
}

impl<T> ::std::fmt::Pointer for PmemConstVirtualPtr<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:#x}:{:#x}",self.poolid, self.offset)
    }
}

impl<T> ::std::fmt::Debug for PmemConstVirtualPtr<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "PmemVirt *const {{ pool: {:#x}, offset: {:#x} }}", self.poolid, self.offset)
    }
}

/// Persistent memory virtual mutable pointer
///
/// This pointer is safe to store.
#[derive(Copy, Clone)]
pub struct PmemMutVirtualPtr<T: ?Sized> {
    poolid: usize,
    offset: usize,
    _t: PhantomData<T>,
}

impl<T: ?Sized> PmemMutVirtualPtr<T> {
    pub unsafe fn new(poolid: usize, offset: usize) -> Self {
        assert!(poolid != 0, "Poolid 0 is reserved for null pointers");
        PmemMutVirtualPtr { poolid: poolid, offset: offset, _t: PhantomData }
    }

    pub fn null() -> Self { PmemMutVirtualPtr { poolid: 0, offset: 0, _t: PhantomData } }

    pub fn is_null(&self) -> bool { self.poolid == 0 }

    pub unsafe fn as_type<U>(&self) -> PmemMutVirtualPtr<U> {
        PmemMutVirtualPtr { poolid: self.poolid, offset: self.offset, _t: PhantomData }
    }

    pub unsafe fn as_const(&self) -> PmemConstVirtualPtr<T> {
        PmemConstVirtualPtr { poolid: self.poolid, offset: self.offset, _t: PhantomData }
    }
}

impl<T> ::std::fmt::Pointer for PmemMutVirtualPtr<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:#x}:{:#x}",self.poolid, self.offset)
    }
}

impl<T> ::std::fmt::Debug for PmemMutVirtualPtr<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "PmemVirt *mut {{ pool: {:#x}, offset: {:#x} }}", self.poolid, self.offset)
    }
}

impl<T> PmemMutVirtualPtr<T> {
    pub unsafe fn offset(&self, count: isize) -> Self {
        if self.is_null() {
            PmemMutVirtualPtr::null()
        } else {
            let new_offset = self.offset as isize + mem::size_of::<T>() as isize * count;
            PmemMutVirtualPtr { poolid: self.poolid, offset: new_offset as usize, _t: self._t }
        }
    }

    pub unsafe fn link(&self, pool: &mut PersistentMap) -> PmemMutPtr<T> {
        let new_virt = PmemMutVirtualPtr { poolid: self.poolid, offset: self.offset, _t: self._t };
        PmemMutPtr { virt: new_virt, pool: pool.as_mut_ptr() }
    }
}

/// Direct `*const T` pointer to a pmem location
///
/// # Safety
///
/// This pointer is **not** safe to store directly on pmem.
/// You can get a _safe_ virtual pointer using `as_virtual()`.
#[derive(Copy, Clone)]
pub struct PmemConstPtr<T: ?Sized> {
    virt: PmemConstVirtualPtr<T>,
    pool: *const T,
}

impl<T: ?Sized> PmemConstPtr<T> {
    pub fn is_null(&self) -> bool { self.virt.is_null() }

    pub unsafe fn as_mut(&self) -> PmemMutPtr<T> {
        PmemMutPtr { virt: self.virt.as_mut(), pool: self.pool as *mut T }
    }
}

impl<T> PmemConstPtr<T> {
    pub unsafe fn direct(&self) -> *const T {
        if self.is_null() {
            ::std::ptr::null()
        } else {
            self.pool.offset(self.virt.offset as isize) as *const T
        }
    }

    pub fn as_virtual(self) -> PmemConstVirtualPtr<T> {
        self.virt
    }

    pub unsafe fn offset(&self, count: isize) -> Self {
        let new_virt = self.virt.offset(count);
        PmemConstPtr { virt: new_virt, pool: self.pool }
    }

    pub unsafe fn as_type<U>(&self) -> PmemConstPtr<U> {
        let new_virt = self.virt.as_type();
        PmemConstPtr { virt: new_virt, pool: self.pool as *mut U }
    }
}

impl<T> ::std::fmt::Pointer for PmemConstPtr<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:p} -> {:p}",self.virt, unsafe { self.direct() })
    }
}

impl<T> ::std::fmt::Debug for PmemConstPtr<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Pmem *const {{ pool: {:#x}, offset: {:#x} }}", self.virt.poolid, self.virt.offset)
    }
}

/// Direct `*mut T` pointer to a pmem location
///
/// # Safety
///
/// This pointer is **not** safe to store directly on pmem.
/// You can get a _safe_ virtual pointer using `as_virtual()`.
#[derive(Copy, Clone)]
pub struct PmemMutPtr<T: ?Sized> {
    virt: PmemMutVirtualPtr<T>,
    pool: *mut T,
}

impl<T: ?Sized> PmemMutPtr<T> {
    pub fn is_null(&self) -> bool { self.virt.is_null() }

    pub unsafe fn as_const(&self) -> PmemConstPtr<T> {
        PmemConstPtr { virt: self.virt.as_const(), pool: self.pool as *const T }
    }
}

impl<T> PmemMutPtr<T> {
    pub unsafe fn direct(&self) -> *mut T {
        if self.is_null() {
            ::std::ptr::null_mut()
        } else {
            self.pool.offset(self.virt.offset as isize) as *mut T
        }
    }

    pub fn as_virtual(self) -> PmemMutVirtualPtr<T> {
        self.virt
    }

    pub unsafe fn offset(&self, count: isize) -> Self {
        let new_virt = self.virt.offset(count);
        PmemMutPtr { virt: new_virt, pool: self.pool }
    }

    pub unsafe fn as_type<U>(&self) -> PmemMutPtr<U> {
        let new_virt = self.virt.as_type();
        PmemMutPtr { virt: new_virt, pool: self.pool as *mut U }
    }
}

impl<T> ::std::fmt::Pointer for PmemMutPtr<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:p} -> {:p}",self.virt, unsafe { self.direct() })
    }
}

impl<T> ::std::fmt::Debug for PmemMutPtr<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Pmem *mut {{ pool: {:#x}, offset: {:#x} }}", self.virt.poolid, self.virt.offset)
    }
}

pub fn null<T>() -> PmemConstVirtualPtr<T> {
    PmemConstVirtualPtr::null()
}

pub fn null_mut<T>() -> PmemMutVirtualPtr<T> {
    PmemMutVirtualPtr::null()
}


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
/// `copy_nonoverlapping` is semantically equivalent to C's `memcpy` and is optimized for persitent memory.
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
pub unsafe fn copy_nonoverlapping<T>(src: *const T, pmemdest: *mut T, count: usize) {
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
pub unsafe fn write<T>(pmemdest: *mut T, val: T) { copy_nonoverlapping(&val as *const _, pmemdest, 1) }

pub unsafe fn msync<T>(pmemdest: *const T, count: usize) -> Result<(), io::Error> {
    let len = count * mem::size_of::<T>();
    let r = ffi::pmem_msync(pmemdest as *const c_void, len as size_t);
    if r == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
