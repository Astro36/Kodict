use super::Word;
use std::fs;
use std::path::Path;

pub fn read_as_tsv(path: &Path) -> Vec<Word> {
  fs::read_to_string(path)
    .expect("Unable to read file!")
    .split("\n")
    .map(|element| {
      let props = element.split("\t").collect::<Vec<&str>>();
      Word {
        entry: props[0].to_string(),
        meaning: props[1].to_string(),
        pos: props[2].split(",").map(|value| value.to_string()).collect(),
        category: props[3].split(",").map(|value| value.to_string()).collect(),
      }
    })
    .collect()
}

pub fn write_as_tsv(path: &Path, words: &Vec<Word>) {
  fs::write(
    &path,
    words
      .into_iter()
      .map(|word| {
        format!(
          "{}\t{}\t{}\t{}",
          word.entry,
          word.meaning,
          word.pos.join(","),
          word.category.join(",")
        )
      })
      .collect::<Vec<String>>()
      .join("\n")
      .as_bytes(),
  )
  .expect("Unable to write file!");
}
