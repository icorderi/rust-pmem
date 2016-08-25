extern crate pmem_blk;

use ::std::fs;
use ::std::path::Path;

use ::pmem_blk::BlkPool;


// #[test] - ignore for now
fn create() {
    let path = Path::new("/tmp/test-create.pmemblk");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }
    let _p: BlkPool = BlkPool::create(path, 64 * 1024, 10 * 1024 * 1024).unwrap();
}

// #[test] - ignore for now
fn open() {
    let path = Path::new("/tmp/test-open.pmemblk");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    {
        let _p = BlkPool::create(path, 64 * 1024, 10 * 1024 * 1024).unwrap();
    }

    let _p = BlkPool::open(path).unwrap();
}

// #[test] - ignore for now
fn version() {
    BlkPool::check_version(1,1).unwrap();
}
