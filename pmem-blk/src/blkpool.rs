use ::std::ffi::{CString, CStr};
use ::std::path::Path;
use ::std::io;

use ::libc::c_uint;
use ::libc::{size_t, mode_t};
use ::pmemblk_sys::{self as ffi, PMEMblkpool};

fn errormsg() -> Option<String> {
    unsafe {
        let reason_p = ffi::pmemblk_errormsg();
        if ! reason_p.is_null() {
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
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        BlkPool::open_with_blksize(path, 0)
    }

    pub fn open_with_blksize<P: AsRef<Path>>(path: P, blksize: usize) -> Result<Self, io::Error> {
        let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        let objpool = unsafe { ffi::pmemblk_open(path.as_ptr(), blksize as size_t) };

        if objpool.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(BlkPool { inner: objpool })
        }
    }

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

    pub fn check_version(major_required: usize, minor_required: usize) -> Result<(),String> {
        unsafe {
            let reason_p = ffi::pmemblk_check_version(major_required as c_uint, minor_required as c_uint);
            if ! reason_p.is_null() {
                let reason = CStr::from_ptr(reason_p).to_owned().into_string().unwrap();
                Err(reason)
            } else {
                Ok(())
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
