extern crate pmem_obj;

use ::std::fs;
use ::std::path::Path;

use ::pmem_obj::ObjPool;


// #[test]
// Getting "invalid memory reference" on close
fn create() {
    let path = Path::new("/tmp/test-create.pmemobj");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }
    let _p = ObjPool::create(path, "", 10 * 1024 * 1024).unwrap();
}

// #[test]
// Getting "invalid memory reference" on close
fn open() {
    let path = Path::new("/tmp/test-open.pmemobj");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    {
        let _p = ObjPool::create(path, "", 10 * 1024 * 1024).unwrap();
    }

    let _p = ObjPool::open(path, "").unwrap();
}
