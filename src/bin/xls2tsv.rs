use kodict::{fs, parser};
use std::env;
use std::ffi::OsStr;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let output = &args[2];
    let target = Path::new(input);
    println!("Input: {}", input);
    let words = if target.is_dir() {
        std::fs::read_dir(target)?
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().unwrap_or(OsStr::new("")) == "xls")
            .map(|path| parser::parse_open_dictionary_xls(&path))
            .flatten()
            .collect()
    } else {
        parser::parse_open_dictionary_xls(target)
    };
    fs::write_as_tsv(Path::new(output), &words);
    println!("Output: {}", output);
    Ok(())
}
