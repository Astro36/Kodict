extern crate kodict;

use kodict::fs;
use kodict::Dictionary;
use std::path::Path;

#[test]
fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/dictionary.tsv");
    let dictionary = Dictionary::new(fs::read_as_tsv(&path));
    println!("find: {:?}", dictionary.find("사과"));
    assert_eq!(dictionary.has("사과"), true);
    assert_eq!(dictionary.has("배추"), false);
    // assert_eq!(dictionary.size(), 7);
}
