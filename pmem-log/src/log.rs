use ::std::ffi::CString;
use ::std::path::Path;
use ::std::io;

use ::libc::iovec;
use ::libc::{size_t, mode_t};
use ::libc::{c_void, c_int};

use pmemlog_sys::{self as ffi, PMEMlogpool};

pub struct Log {
    inner: *mut PMEMlogpool,
}

extern "C" fn visit_log<F>(buf: *const c_void, len: size_t, arg: *mut c_void) -> c_int
    where F: Fn(&[u8]) -> Option<()>
{
    let callback = arg as *const F;
    let buf = buf as *mut u8;
    unsafe {
        let item = ::std::slice::from_raw_parts(buf, len);
        match (*callback)(item) {
            Some(()) => 1,
            None => 0,
        }
    }
}

impl Log {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        let objpool = unsafe { ffi::pmemlog_open(path.as_ptr()) };

        if objpool.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(Log { inner: objpool })
        }
    }

    pub fn create<P: AsRef<Path>>(path: P, size: usize) -> Result<Self, io::Error> {
        let path = path.as_ref().to_str().unwrap();
        let path = CString::new(path).unwrap();

        let mode = 0o666;

        let objpool = unsafe { ffi::pmemlog_create(path.as_ptr(), size as size_t, mode as mode_t) };

        if objpool.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(Log { inner: objpool })
        }
    }

    pub fn append<T: AsRef<[u8]>>(&mut self, entry: T) -> Result<(), io::Error> {
        let buf = entry.as_ref();
        let len = buf.len();

        let r = unsafe {
            let buf = buf.as_ptr() as *mut c_void;
            ffi::pmemlog_append(self.inner, buf, len as size_t)
        };

        if r == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    pub fn append_many<T: AsRef<[u8]>>(&mut self, entries: &[T]) -> Result<(), io::Error> {
        let count = entries.len();
        let mut io_vecs = Vec::with_capacity(count);
        for entry in entries {
            let buf = entry.as_ref();
            io_vecs.push(iovec { iov_base: buf.as_ptr() as *mut c_void,
                                 iov_len: buf.len() as size_t, });
        }

        let r = unsafe {
            let io_vecs = io_vecs.as_slice().as_ptr() as *const iovec;
            ffi::pmemlog_appendv(self.inner, io_vecs, count as c_int)
        };

        if r == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    pub fn len(&self) -> usize {
        unsafe { ffi::pmemlog_tell(self.inner) as usize }
    }

    pub fn capacity(&self) -> usize { unsafe { ffi::pmemlog_nbyte(self.inner) as usize } }

    pub fn walk<F>(&self, chunk_size: usize, callback: F)
        where F: Fn(&[u8]) -> Option<()>
    {
        unsafe {
            let arg = &callback as *const _ as *mut c_void;
            ffi::pmemlog_walk(self.inner, chunk_size as size_t, visit_log::<F>, arg)
        };
    }
}

impl Drop for Log {
    fn drop(&mut self) {
        unsafe {
            ffi::pmemlog_close(self.inner);
        }
    }
}
