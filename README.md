# pmem

Rust abstractions over the NVM Library [http://pmem.io](http://pmem.io)

## Dashboard

| Linux CI | Test Coverage | Crate | Documentation |
|:--------:|:-------------:|:-----:|:-------------:|
| [![Build Status](https://travis-ci.org/icorderi/rust-pmem.svg?branch=master)](https://travis-ci.org/icorderi/rust-pmem) | [![Coverage Status](https://coveralls.io/repos/icorderi/rust-pmem/badge.svg?branch=master)](https://coveralls.io/r/icorderi/rust-pmem?branch=master) | [![Crate](http://meritbadge.herokuapp.com/pmem)](https://crates.io/crates/pmem) | [![Docs](https://img.shields.io/badge/docs-up--to--date-blue.svg)](https://icorderi.github.io/rust-pmem/index.html)

## What's in this project

This repository currently houses a number of pmem-related crates, all with associated documentation:

- **pmem** (you are here) - low level persistent memory support ([doc](https://icorderi.github.io/rust-pmem/index.html))
- [pmem-obj] - transactional object store, providing memory allocation, transactions, and general facilities for persistent memory programming
- [pmem-log] - a pmem-resident log file
- [pmem-blk] - arrays of pmem-resident blocks, all the same size, that are atomically updated

[pmem-obj]: pmem-obj/README.md
[pmem-log]: pmem-log/README.md
[pmem-blk]: pmem-blk/README.md

You can all find the `*-sys` crates with no abstractions:

- [pmem-sys](sys/pmem-sys/README.md)
- [pmemobj-sys](sys/pmemobj-sys/README.md)
- [pmemlog-sys](sys/pmemlog-sys/README.md)
- [pmemblk-sys](sys/pmemblk-sys/README.md)

## Usage

**TODO:** write...

Don't forget to check out the [examples](./examples)

## License

Licensed under:

- Apache License, Version 2.0 - [LICENSE-APACHE](LICENSE-APACHE) ([source](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license - ([LICENSE-MIT](LICENSE-MIT) ([source](http://opensource.org/licenses/MIT))

This library links with the [NVML](https://github.com/pmem/nvml), you can see the NVML license [here](https://github.com/pmem/nvml/blob/master/LICENSE).

> This is **not** an official port of the NVM Library.
>
> The official **libpmem** documentation can be found at: [http://pmem.io/nvml/libpmem/](http://pmem.io/nvml/libpmem/)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
