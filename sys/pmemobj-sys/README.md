# FFI bindings to **libpmemobj**

Following the `*-sys` package conventions,
the **pmemobj-sys** crate does not define higher-level abstractions over the native **libpmemobj** library functions.

## Dashboard

| Linux CI | Test Coverage | Crate | Documentation |
|:--------:|:-------------:|:-----:|:-------------:|
| [![Build Status](https://travis-ci.org/icorderi/rust-pmem.svg?branch=master)](https://travis-ci.org/icorderi/rust-pmem) | [![Coverage Status](https://coveralls.io/repos/icorderi/rust-pmem/badge.svg?branch=master)](https://coveralls.io/r/icorderi/rust-pmem?branch=master) | [![Crate](http://meritbadge.herokuapp.com/pmemobj-sys)](https://crates.io/crates/pmemobj-sys) | [![Docs](https://img.shields.io/badge/docs-up--to--date-blue.svg)](https://icorderi.github.io/rust-pmem/pmemobj_sys/)

## Usage

TODO

## License

Licensed under:

- Apache License, Version 2.0 - [LICENSE-APACHE](../LICENSE-APACHE) ([source](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license - ([LICENSE-MIT](../LICENSE-MIT) ([source](http://opensource.org/licenses/MIT))

This library links with the [NVML](https://github.com/pmem/nvml), you can see the NVML license [here](https://github.com/pmem/nvml/blob/master/LICENSE).

> This is **not** an official port of the NVM Library.
>
> The official **libpmemobj** documentation can be found at: [http://pmem.io/nvml/libpmemobj/](http://pmem.io/nvml/libpmemobj/)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
