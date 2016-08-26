use ::std::ffi::{CString, CStr};
use ::std::path::Path;
use ::std::io;

use ::libc::{size_t, mode_t};

use pmemobj_sys::{self as ffi, PMEMobjpool};


fn errormsg() -> Option<String> {
    unsafe {
        let reason_p = ffi::pmemobj_errormsg();
        if !reason_p.is_null() {
            CStr::from_ptr(reason_p).to_owned().into_string().ok()
        } else {
            None
        }
    }
}

pub struct ObjPool {
    inner: *mut PMEMobjpool,
}


impl ObjPool {
    pub fn open<P: AsRef<Path>, S: Into<String>>(path: P, layout: S) -> Result<Self, io::Error> {
        let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        let layout = CString::new(layout.into()).unwrap();

        let objpool = unsafe { ffi::pmemobj_open(path.as_ptr(), layout.as_ptr()) };

        if objpool.is_null() {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::InvalidInput {
                if let Some(msg) = errormsg() {
                    return Err(io::Error::new(io::ErrorKind::Other, msg));
                }
            }
            Err(err)
        } else {
            Ok(ObjPool { inner: objpool })
        }
    }

    pub fn create<P: AsRef<Path>, S: Into<String>>(path: P,
                                                   layout: S,
                                                   size: usize)
                                                   -> Result<Self, io::Error> {
        let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        let layout = CString::new(layout.into()).unwrap();

        let mode = 0o666;

        let objpool =
            unsafe { ffi::pmemobj_create(path.as_ptr(), layout.as_ptr(), size as size_t, mode as mode_t) };

        if objpool.is_null() {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::InvalidInput {
                if let Some(msg) = errormsg() {
                    return Err(io::Error::new(io::ErrorKind::Other, msg));
                }
            }
            Err(err)
        } else {
            Ok(ObjPool { inner: objpool })
        }
    }
}


impl Drop for ObjPool {
    fn drop(&mut self) {
        unsafe {
            ffi::pmemobj_close(self.inner);
        }
    }
}
