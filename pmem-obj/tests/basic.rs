extern crate pmem_obj;

use ::std::fs;
use ::std::path::Path;
use ::std::fs::File;

use ::pmem_obj::ObjPool;

//#[test] - ignore for now
fn create() {
    let path = Path::new("/tmp/test-create.pmemobj");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }
    File::create(&path).unwrap();
    let _p = ObjPool::create(path, "", 2*1024*1024).unwrap();
}
