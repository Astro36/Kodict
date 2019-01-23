extern crate calamine;
#[macro_use]
extern crate lazy_static;
extern crate rayon;
extern crate regex;
extern crate reqwest;

pub mod crawler;
pub mod fs;
pub mod parser;
pub mod trie;

use trie::Map;

#[derive(Debug)]
pub struct Word {
    pub entry: String,
    pub meaning: String,
    pub pos: Vec<String>,
    pub category: Vec<String>,
}

pub type Words = Vec<Word>;

pub struct Dictionary {
    pub words: Map<Word>,
}

impl Dictionary {
    pub fn new(words: Words) -> Dictionary {
        let mut map = Map::new();
        for word in words {
            map.insert(word.entry.to_string(), word);
        }
        Dictionary { words: map }
    }

    pub fn find(&self, entry: &str) -> Option<&Words> {
        self.words.get(entry.to_string())
    }

    pub fn find_mut(&mut self, entry: &str) -> Option<&mut Words> {
        self.words.get_mut(entry.to_string())
    }

    pub fn get_all(&self) -> Vec<&Word> {
        self.words.values()
    }

    pub fn has(&self, entry: &str) -> bool {
        self.words.contains_key(entry.to_string())
    }

    pub fn size(&self) -> usize {
        self.words.len()
    }
}
