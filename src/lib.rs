//! # FFI bindings to **libpmem**
//!
//! Rust bindings for the NVM Library [http://pmem.io](http://pmem.io)
//!
//! The **pmem** library provides low level persistent memory support.
//! The libraries above are implemented using **pmem**.
//! Developers wishing to roll their own persistent memory algorithms will find this library useful,
//! but most developers will likely use **pmem-obj** and let that library call **pmem** for them.
//!
//! > This is **not** an official port of the NVM Library.
//! >
//! > The official **libpmem** documentation can be found at: [http://pmem.io/nvml/libpmem/](http://pmem.io/nvml/libpmem/)

extern crate pmem_sys;
