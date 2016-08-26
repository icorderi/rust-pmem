use ::std::ffi::{CString, CStr};
use ::std::path::Path;
use ::std::io;

use ::libc::{size_t, mode_t};
use ::pmemblk_sys::{self as ffi, PMEMblkpool};

pub use ::pmemblk_sys::PMEMBLK_MIN_POOL as MIN_POOLSIZE;

fn errormsg() -> Option<String> {
    unsafe {
        let reason_p = ffi::pmemblk_errormsg();
        if !reason_p.is_null() {
            CStr::from_ptr(reason_p).to_owned().into_string().ok()
        } else {
            None
        }
    }
}

pub struct BlkPool {
    inner: *mut PMEMblkpool,
}


impl BlkPool {
    /// Opens an existent memory pool with an _unknown_ block size
    ///
    /// Use `block_size()` to query the block size of the opened memory pool.
    pub fn open_no_size<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> { BlkPool::open(path, 0) }

    /// Opens an existent memory pool
    ///
    /// If the `blksize` provided is non-zero, we will verify the given block size matches
    /// the block size used when the pool was created.
    pub fn open<P: AsRef<Path>>(path: P, blksize: usize) -> Result<Self, io::Error> {
        let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        let objpool = unsafe { ffi::pmemblk_open(path.as_ptr(), blksize as size_t) };

        if objpool.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(BlkPool { inner: objpool })
        }
    }

    /// Creates a block memory pool with the given total `poolsize` divided up into as many elements of size `blksize` as will fit in the pool
    ///
    /// Since the transactional nature of a block memory pool requires some space overhead in the memory pool,
    /// the resulting number of available blocks is less than `poolsize` / `blksize`.
    ///
    /// You can query the total number of blocks with `capacity()`.
    ///
    /// Given the specifics of the implementation, the number of available blocks for the user cannot be less than 256.
    /// This translates to at least 512 internal blocks.
    pub fn create<P: AsRef<Path>>(path: P, blksize: usize, poolsize: usize) -> Result<Self, io::Error> {
        let path = path.as_ref().to_str().unwrap();
        let path = CString::new(path).unwrap();

        let mode = 0o666;

        let objpool = unsafe {
            ffi::pmemblk_create(path.as_ptr(), blksize as size_t, poolsize as size_t, mode as mode_t)
        };

        if objpool.is_null() {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::InvalidInput {
                if let Some(msg) = errormsg() {
                    return Err(io::Error::new(io::ErrorKind::Other, msg));
                }
            }
            Err(err)
        } else {
            Ok(BlkPool { inner: objpool })
        }
    }

    /// The block size for this pool
    pub fn block_size(&self) -> usize { unsafe { ffi::pmemblk_bsize(self.inner) as usize } }

    /// The capacity of the pool in number of blocks
    pub fn capacity(&self) -> usize { unsafe { ffi::pmemblk_nblock(self.inner) as usize } }

    /// Reads block number `blockno` from the memory pool into `buf`
    ///
    /// Reading a block that has never been written will return a block of zeroes.
    pub fn read(&self, buf: &mut [u8], blockno: i64) -> Result<(),io::Error> {
        let r = unsafe {
            ffi::pmemblk_read(self.inner, buf.as_ptr() as *mut _, blockno)
        };
        if r == 0 {
            Ok(())
        } else {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::InvalidInput {
                if let Some(msg) = errormsg() {
                    return Err(io::Error::new(io::ErrorKind::Other, msg));
                }
            }
            Err(err)
        }
    }

    /// Writes a block from `buf` to block number `blockno` in the memory pool
    ///
    /// The write is **atomic** with respect to other reads and writes.
    /// In addition, the write cannot be torn by program failure or system crash;
    /// on recovery the block is guaranteed to contain either the old data or the new data
    /// never a mixture of both.
    pub fn write(&self, buf: &[u8], blockno: i64) -> Result<(),io::Error> {
        let r = unsafe {
            ffi::pmemblk_write(self.inner, buf.as_ptr() as *const _, blockno)
        };
        if r == 0 {
            Ok(())
        } else {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::InvalidInput {
                if let Some(msg) = errormsg() {
                    return Err(io::Error::new(io::ErrorKind::Other, msg));
                }
            }
            Err(err)
        }
    }

    /// Check consistency of the memory pool
    pub fn check<P: AsRef<Path>>(path: P, blksize: usize) -> Result<bool, io::Error> {
        let path = path.as_ref().to_str().unwrap();
        let path = CString::new(path).unwrap();

        let r = unsafe { ffi::pmemblk_check(path.as_ptr(), blksize as size_t) };
        match r {
            1 => Ok(true),
            0 => Ok(false),
            -1 => {
                let err = io::Error::last_os_error();
                if err.kind() == io::ErrorKind::InvalidInput {
                    if let Some(msg) = errormsg() {
                        return Err(io::Error::new(io::ErrorKind::Other, msg));
                    }
                }
                Err(err)
            }
            r => {
                Err(io::Error::new(io::ErrorKind::Other,
                                   format!("Invalid return value, expected 1, 0 or -1 but received {}", r)))
            }
        }
    }
}

impl Drop for BlkPool {
    fn drop(&mut self) {
        unsafe {
            ffi::pmemblk_close(self.inner);
        }
    }
}
