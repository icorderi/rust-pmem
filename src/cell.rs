pub struct PmemRef<T> {
    ptr: *const T,
}

impl<T> PmemRef<T> {
    pub unsafe fn new(ptr: *const T) -> Self {
        PmemRef { ptr: ptr }
    }
}

impl<T> ::std::ops::Deref for PmemRef<T> {
    type Target = T;
    fn deref(&self) -> &T { unsafe { &*self.ptr } }
}

impl<T> ::std::fmt::Debug for PmemRef<T> where T: ::std::fmt::Debug {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", (self as &T))
    }
}

impl<T> ::std::fmt::Display for PmemRef<T> where T: ::std::fmt::Display {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", (self as &T))
    }
}

pub struct PmemMutRef<T> {
    ptr: *mut T,
}

impl<T> PmemMutRef<T> {
    pub unsafe fn new(ptr: *mut T) -> Self {
        PmemMutRef { ptr: ptr }
    }
}

impl<T> ::std::ops::Deref for PmemMutRef<T> {
    type Target = T;
    fn deref(&self) -> &T { unsafe { &*self.ptr } }
}

impl<T> ::std::ops::DerefMut for PmemMutRef<T> {
    fn deref_mut(&mut self) -> &mut T { unsafe { &mut *self.ptr } }
}

impl<T> ::std::fmt::Debug for PmemMutRef<T> where T: ::std::fmt::Debug {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", (self as &T))
    }
}

impl<T> ::std::fmt::Display for PmemMutRef<T> where T: ::std::fmt::Display {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", (self as &T))
    }
}
