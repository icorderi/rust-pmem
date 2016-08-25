use ::std::ffi::CString;
use ::std::path::Path;
use ::std::io;

use ::libc::{size_t, mode_t};

use pmemobj_sys::{self as ffi, PMEMobjpool};


pub struct ObjPool {
    inner: *mut PMEMobjpool,
}


impl ObjPool {
    pub fn open<P: AsRef<Path>, S: Into<String>>(path: P, layout: S) -> Self {
        let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        let layout = CString::new(layout.into()).unwrap();

        let objpool = unsafe { ffi::pmemobj_open(path.as_ptr(), layout.as_ptr()) };

        ObjPool { inner: objpool }
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
            Err(io::Error::last_os_error())
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
