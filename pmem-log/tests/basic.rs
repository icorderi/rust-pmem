extern crate pmem_log;

use ::std::fs;
use ::std::path::Path;

use ::pmem_log::Log;


#[test]
fn create() {
    let path = Path::new("/tmp/test-create.pmemlog");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }
    let _p: Log = Log::create(path, 2 * 1024 * 1024).unwrap();
}

#[test]
fn append() {
    let path = Path::new("/tmp/test-append.pmemlog");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }
    let mut p = Log::create(path, 2 * 1024 * 1024).unwrap();
    p.append("Hello world").unwrap();
}

#[test]
fn open() {
    let path = Path::new("/tmp/test-open.pmemlog");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    {
        let mut p = Log::create(path, 2 * 1024 * 1024).unwrap();
        p.append("Hello world").unwrap();
    }

    let mut p = Log::open(path).unwrap();
    p.append("Welcome back").unwrap();
}

#[test]
fn walk() {
    let path = Path::new("/tmp/test-walk.pmemlog");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    let mut p = Log::create(path, 2 * 1024 * 1024).unwrap();
    p.append("dez").unwrap();
    p.append("foo").unwrap();
    p.append("bar").unwrap();

    p.walk(3, |t| {
        println!("Found: {}", String::from_utf8_lossy(t));
        Some(())
    });
}

#[test]
fn walk_after_open() {
    let path = Path::new("/tmp/test-walk-after-open.pmemlog");
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    {
        let mut p = Log::create(path, 2 * 1024 * 1024).unwrap();
        p.append("Hello world").unwrap();
        p.append("foo").unwrap();
        p.append("bar").unwrap();
    }

    let mut p = Log::open(path).unwrap();
    p.append("after-load").unwrap();

    p.walk(3, |t| {
        println!("Found: {}", String::from_utf8_lossy(t));
        Some(())
    });
}
