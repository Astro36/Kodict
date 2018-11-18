extern crate kodict;

use kodict::Dictionary;
use std::path::Path;

#[test]
fn main() {
    let dictionary = Dictionary::create_from_file(&Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/dictionary.tsv"));
    println!("find: {:?}", dictionary.find("사과").ok().unwrap());
    println!("find_all: {:?}", dictionary.find_all("사과").ok().unwrap());
    assert_eq!(dictionary.has("사과"), true);
    assert_eq!(dictionary.has("배추"), false);
    assert_eq!(dictionary.size(), 10);
}
