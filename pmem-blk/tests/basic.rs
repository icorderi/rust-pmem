extern crate pmem_blk;

use ::std::fs;
use ::std::path::Path;

use ::pmem_blk::BlkPool;


#[test]
fn create() {
    let path = Path::new("/tmp/test-create.pmemblk");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    let _p: BlkPool = BlkPool::create(path, 4 * 1024, 20 * 1024 * 1024).unwrap();
}

#[test]
fn open() {
    let path = Path::new("/tmp/test-open.pmemblk");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    {
        let _p = BlkPool::create(path, 4 * 1024, 20 * 1024 * 1024).unwrap();
    }

    let _p = BlkPool::open(path).unwrap();
}

#[test]
fn version() {
    BlkPool::check_version(1,0).unwrap();
}
