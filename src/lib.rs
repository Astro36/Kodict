#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate reqwest;

pub mod fs;

pub struct Dictionary {
    items: Vec<DictionaryItem>,
}

impl Dictionary {
    pub fn new(items: Vec<DictionaryItem>) -> Dictionary {
        Dictionary {
            items: items,
        }
    }

    pub fn find(&self, word: &str) -> Result<&DictionaryItem, ()> {
        for item in &self.items {
            if item.word == word {
                return Ok(item);
            }
        }
        Err(())
    }

    pub fn find_all(&self, word: &str) -> Result<Vec<&DictionaryItem>, ()> {
        let mut items = vec![];
        for item in &self.items {
            if item.word == word {
                items.push(item);
            }
        }
        if items.len() > 0 {
            return Ok(items);
        }
        Err(())
    }

    pub fn has(&self, word: &str) -> bool {
        for item in &self.items {
            if item.word == word {
                return true;
            }
        }
        return false;
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

#[derive(Debug)]
pub struct DictionaryItem {
    pub word: String,
    pub meaning: String,
    pub pos: Vec<String>,
    pub category: Vec<String>,
}
