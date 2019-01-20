extern crate kodict;

use kodict::crawler;
use kodict::fs;
use std::path::Path;

fn main() {
    let words = crawler::get_standard_dictionary_words(16);
    fs::write_as_tsv(Path::new("./dictionary.tsv"), &words);
    println!("Complete: {:?} words", words.len());
}
