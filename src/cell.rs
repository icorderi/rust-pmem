pub struct PmemCell<T> {
    ptr: *mut T,
}

impl<T> PmemCell<T> {
    pub unsafe fn new(ptr: *mut T) -> Self {
        PmemCell { ptr: ptr }
    }
}

impl<T> ::std::ops::Deref for PmemCell<T> {
    type Target = T;
    fn deref(&self) -> &T { unsafe { &*self.ptr } }
}

impl<T> ::std::ops::DerefMut for PmemCell<T> {
    fn deref_mut(&mut self) -> &mut T { unsafe { &mut *self.ptr } }
}
