extern crate kodict;

use kodict::Dictionary;
use std::path::Path;

fn main() {
    let dict = Dictionary::create_from_web(16);
    println!("Complete: {:?} words", dict.items.len());
    dict.save_as_tsv(Path::new("./out.tsv"));
}
