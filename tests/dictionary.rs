extern crate kodict;

use kodict::fs;
use kodict::Dictionary;
use std::path::Path;

#[test]
fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/dictionary.tsv");
    let dictionary = Dictionary::new(fs::read_as_tsv(&path));
    dbg!(dictionary.find("사과").unwrap());
    assert_eq!(dictionary.has("사과"), true);
    assert_eq!(dictionary.has("배추"), false);
}
