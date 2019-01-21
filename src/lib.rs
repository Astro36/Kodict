#[macro_use]
extern crate lazy_static;
extern crate rayon;
extern crate regex;
extern crate reqwest;

pub mod crawler;
pub mod fs;
pub mod trie;

use trie::TrieMap;

#[derive(Debug)]
pub struct Word {
    pub entry: String,
    pub meaning: String,
    pub pos: Vec<String>,
    pub category: Vec<String>,
}

pub struct Dictionary {
    words: trie::TrieMap<Word>,
}

impl Dictionary {
    pub fn new(words: Vec<Word>) -> Dictionary {
        let mut trie = TrieMap::new();
        for word in words {
            trie.insert(word.entry.to_string(), word);
        }
        Dictionary { words: trie }
    }

    pub fn find(&self, entry: &str) -> Option<&Vec<Word>> {
        self.words.get(entry.to_string())
    }

    pub fn has(&self, entry: &str) -> bool {
        self.words.contains_key(entry.to_string())
    }
}
