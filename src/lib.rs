#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate reqwest;

use std::collections::BTreeMap;

pub mod crawler;
pub mod fs;

#[derive(Debug)]
pub struct Word {
    pub entry: String,
    pub meaning: String,
    pub pos: Vec<String>,
    pub category: Vec<String>,
}

pub struct Dictionary {
    words: BTreeMap<String, Vec<Word>>,
}

impl Dictionary {
    pub fn new(words: Vec<Word>) -> Dictionary {
        let mut map = BTreeMap::new();
        for word in words {
            let entry = word.entry.to_string();
            if map.contains_key(&entry) {
                let v: &mut Vec<Word> = map.get_mut(&entry).unwrap();
                v.push(word);
            } else {
                map.insert(entry, vec![word]);
            }
        }
        Dictionary { words: map }
    }

    pub fn find(&self, entry: &str) -> Option<&Vec<Word>> {
        self.words.get(entry)
    }

    pub fn has(&self, entry: &str) -> bool {
        self.words.contains_key(entry)
    }

    pub fn size(&self) -> usize {
        self.words.len()
    }
}
