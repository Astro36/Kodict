use std::fs;
use std::path::Path;
use {Dictionary, DictionaryItem};

pub fn read_as_tsv(path: &Path) -> Dictionary {
  Dictionary {
    items: fs::read_to_string(path)
      .expect("Unable to read file!")
      .split("\n")
      .map(|element| {
        let props = element.split("\t").collect::<Vec<&str>>();
        DictionaryItem {
          word: props[0].to_string(),
          meaning: props[1].to_string(),
          pos: props[2].split(",").map(|value| value.to_string()).collect(),
          category: props[3].split(",").map(|value| value.to_string()).collect(),
        }
      })
      .collect(),
  }
}

pub fn write_as_tsv(path: &Path, dictionary: &Dictionary) {
  let items = &dictionary.items;
  fs::write(
    &path,
    items
      .into_iter()
      .map(|item| {
        format!(
          "{}\t{}\t{}\t{}",
          item.word,
          item.meaning,
          item.pos.join(","),
          item.category.join(",")
        )
      })
      .collect::<Vec<String>>()
      .join("\n")
      .as_bytes(),
  )
  .expect("Unable to write file!");
}
