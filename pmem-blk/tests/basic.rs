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

    let _p = BlkPool::open_no_size(path).unwrap();
}

#[test]
fn block_size() {
    let path = Path::new("/tmp/test-block_size.pmemblk");
    let bsize = 4 * 1024;
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    {
        let _p = BlkPool::create(path, bsize, 20 * 1024 * 1024).unwrap();
    }

    let p = BlkPool::open_no_size(path).unwrap();
    assert_eq!(bsize, p.block_size());
}

#[test]
fn capacity() {
    let path = Path::new("/tmp/test-capacity.pmemblk");

    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    let poolsize = 20 * 1024 * 1024;
    let bsize = 4 * 1024;
    let p = BlkPool::create(path, bsize, poolsize).unwrap();
    assert!(p.capacity() > 0);
    assert!(p.capacity() < poolsize / bsize);
}

#[test]
fn check() {
    let path = Path::new("/tmp/test-check.pmemblk");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    let bsize = 4 * 1024;
    {
        let _p = BlkPool::create(path, bsize, 20 * 1024 * 1024).unwrap();
    }

    let pass = BlkPool::check(path, bsize).unwrap();
    assert!(pass);
}

#[test]
fn check_no_bsize() {
    let path = Path::new("/tmp/test-check_no_bsize.pmemblk");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    {
        let _p = BlkPool::create(path, 4 * 1024, 20 * 1024 * 1024).unwrap();
    }

    let pass = BlkPool::check(path, 0).unwrap();
    assert!(pass);
}

#[test]
#[should_panic]
fn check_bad_size() {
    let path = Path::new("/tmp/test-check_bad_size.pmemblk");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    {
        let _p = BlkPool::create(path, 4 * 1024, 20 * 1024 * 1024).unwrap();
    }

    BlkPool::check(path, 2 * 1024).unwrap();
}

#[test]
fn write() {
    let path = Path::new("/tmp/test-write.pmemblk");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    let p: BlkPool = BlkPool::create(path, 4 * 1024, 20 * 1024 * 1024).unwrap();
    let buf = [1; 4 * 1024];
    p.write(&buf, 1).unwrap();
}

#[test]
fn read() {
    let path = Path::new("/tmp/test-read.pmemblk");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    {
        let p: BlkPool = BlkPool::create(path, 4 * 1024, 20 * 1024 * 1024).unwrap();
        let buf = [1; 4 * 1024];
        p.write(&buf, 1).unwrap();
    }

    let p = BlkPool::open_no_size(path).unwrap();
    let mut buf = [0; 4 * 1024];
    p.read(&mut buf, 1).unwrap();
    assert_eq!(buf[0], 1);
    assert_eq!(buf[1024], 1);
}

#[test]
fn version() { pmem_blk::check_version(1, 0).unwrap(); }
