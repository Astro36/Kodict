extern crate kodict;

use kodict::fs;
use std::path::Path;

#[test]
fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/dictionary.tsv");
    let dictionary = fs::read_as_tsv(&path);
    println!("find: {:?}", dictionary.find("사과").ok().unwrap());
    println!("find_all: {:?}", dictionary.find_all("사과").ok().unwrap());
    assert_eq!(dictionary.has("사과"), true);
    assert_eq!(dictionary.has("배추"), false);
    assert_eq!(dictionary.size(), 10);
}
