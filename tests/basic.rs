extern crate pmem;

use ::std::fs;
use ::std::path::Path;

use pmem::pmap::PersistentMap;

#[test]
fn create() {
    let path = Path::new("/tmp/test-create.pmemobj");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }
    let _p = PersistentMap::create(path, 10 * 1024 * 1024, false, 0o666).unwrap();
}
