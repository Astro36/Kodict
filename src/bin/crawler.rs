extern crate kodict;

use kodict::Dictionary;
use std::path::Path;

fn main() {
    let dictionary = Dictionary::create_from_web(16);
    println!("Complete: {:?} words", dictionary.size());
    dictionary.save_as_tsv(Path::new("./dictionary.tsv"));
}
