//! Persistent memory maps

use ::std::mem;
use ::std::io;
use ::std::ffi::CString;
use ::std::path::Path;

use ::libc::{c_void, c_int};
use ::libc::{size_t, mode_t};

use pmem_sys as ffi;
use ptr;
use cell::PmemCell;

/// Persistent memory region
///
/// The region will be automatically unmapped when the variable is dropped
pub struct PersistentMap {
    is_pmem: bool,
    buf: *mut c_void,
    len: usize,
}

impl PersistentMap {
    /// Creates a new read/write mapping for the named file
    ///
    /// It will map the file using mmap(2), but it also takes extra steps to make large page mappings more likely.
    ///
    /// On success, returns a pointer to mapped area.
    /// If mapped_lenp is not NULL, the length of the mapping is also stored at the address it points to.
    /// The is_pmemp argument, if non-NULL, points to a flag that pmem_is_pmem() sets to say
    /// if the mapped file is actual pmem, or if msync() must be used to flush writes for the mapped range.
    /// On error, NULL is returned, errno is set appropriately, and mapped_lenp and is_pmemp are left untouched.
    fn map_file<P: AsRef<Path>>(path: P,
                                len: usize,
                                flags: CreationFlags,
                                mode: u16)
                                -> Result<Self, io::Error> {
        let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        let mut mapped_len = 0;
        let mut is_pmem = 0;
        let r = unsafe {
            ffi::pmem_map_file(path.as_ptr(),
                               len as size_t,
                               flags.bits as c_int,
                               mode as mode_t,
                               &mut mapped_len as *mut _ as *mut size_t,
                               &mut is_pmem as *mut _)
        };
        if r.is_null() {
            Err(io::Error::last_os_error())
        } else {
            let is_pmem = is_pmem > 0;
            Ok(PersistentMap { is_pmem: is_pmem, buf: r, len: mapped_len })
        }
    }

    pub fn create<P: AsRef<Path>>(path: P, len: usize, sparse: bool, mode: u16) -> Result<Self, io::Error> {
        let mut flags = FILE_CREATE | FILE_EXCL;
        if sparse {
            flags = flags | FILE_SPARSE;
        }
        PersistentMap::map_file(path, len, flags, mode)
    }

    pub fn create_tmp<D: AsRef<Path>>(dir: D,
                                      len: usize,
                                      sparse: bool,
                                      mode: u16)
                                      -> Result<Self, io::Error> {
        let mut flags = FILE_TMPFILE | FILE_EXCL;
        if sparse {
            flags = flags | FILE_SPARSE;
        }
        PersistentMap::map_file(dir, len, flags, mode)
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        PersistentMap::map_file(path, 0, CreationFlags::empty(), 0)
    }

    pub fn open_or_create<P: AsRef<Path>>(path: P,
                                          len: usize,
                                          sparse: bool,
                                          mode: u16)
                                          -> Result<Self, io::Error> {
        let mut flags = FILE_CREATE;
        if sparse {
            flags = flags | FILE_SPARSE;
        }
        PersistentMap::map_file(path, len, flags, mode)
    }

    pub fn is_pmem(&self) -> bool { self.is_pmem }

    pub fn len(&self) -> usize { self.len }

    pub unsafe fn uninitialized<T>(&self, offset: isize) -> PmemCell<T> {
        let t_p = self.buf.offset(offset) as *mut u8 as *mut T;
        PmemCell::new(t_p)
    }

    pub unsafe fn zeroed<T>(&self, offset: isize) -> PmemCell<T> {
        let t_p = self.buf.offset(offset) as *mut u8 as *mut T;
        ptr::write_bytes(t_p, 0, 1);
        PmemCell::new(t_p)
    }

    pub unsafe fn write<T>(&self, offset: isize, val: T) -> PmemCell<T> {
        let t_p = self.buf.offset(offset) as *mut u8 as *mut T;
        ptr::write(t_p, val);
        PmemCell::new(t_p)
    }
}

impl ::std::ops::Deref for PersistentMap {
    type Target = [u8];
    fn deref(&self) -> &[u8] { unsafe { ::std::slice::from_raw_parts(self.buf as *mut _, self.len) } }
}

impl ::std::ops::DerefMut for PersistentMap {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { ::std::slice::from_raw_parts_mut(self.buf as *mut _, self.len) }
    }
}

impl Drop for PersistentMap {
    fn drop(&mut self) {
        let len = mem::size_of_val(self);
        let _r = unsafe { ffi::pmem_unmap(self.buf, len) };
        // XXX: What if unmap fails?
    }
}

bitflags! {
    flags CreationFlags: i32 {
/// Create the named file if it does not exist
///
/// `len` must be non-zero and specifies the size of the file to be created.
/// `mode` has the same meaning as for `open()` and specifies the mode to use in case a new file is created.
/// If neither `FILE_CREATE` nor `FILE_TMPFILE` is specified, then `mode` is ignored.
        const FILE_CREATE  = 0b00000001,
/// Ensure that this call creates the file
///
/// If this flag is specified in conjunction with `FILE_CREATE`, and `path` already exists,
/// then `map_file()` will fail.
        const FILE_EXCL    = 0b00000010,
/// When creating a file, create a sparse (holey) file instead of calling `posix_fallocate()`
///
/// Valid only if specified in conjunction with `FILE_CREATE` or `FILE_TMPFILE`, otherwise ignored.
        const FILE_SPARSE  = 0b00000100,
/// Create a mapping for an unnamed temporary file
///
/// `FILE_CREATE` and `len` must be specified and `path` must be an existing directory name.
        const FILE_TMPFILE = 0b00001000,
    }
}
