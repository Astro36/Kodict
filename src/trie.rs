use std::collections::HashMap;
use std::str::Chars;

pub struct TrieMap<T> {
  root: TrieNode<T>,
}

impl<T> TrieMap<T> {
  pub fn new() -> TrieMap<T> {
    TrieMap {
      root: TrieNode::new(),
    }
  }

  pub fn contains_key(&self, key: String) -> bool {
    self.get(key).is_some()
  }

  pub fn get(&self, key: String) -> Option<&Vec<T>> {
    match self.root.find_child(key.chars()) {
      Some(node) => Some(&node.values),
      None => None,
    }
  }

  pub fn insert(&mut self, key: String, value: T) {
    self.root.append_child(key.chars(), value);
  }
}

pub struct TrieNode<T> {
  children: HashMap<char, TrieNode<T>>,
  pub values: Vec<T>,
}

impl<T> TrieNode<T> {
  pub fn new() -> TrieNode<T> {
    TrieNode {
      children: HashMap::new(),
      values: vec![],
    }
  }

  pub fn append_child(&mut self, mut key: Chars, value: T) {
    match key.next() {
      Some(first_char) => match self.children.get_mut(&first_char) {
        Some(child) => {
          child.append_child(key, value);
        }
        None => {
          let mut child = TrieNode::new();
          child.append_child(key, value);
          self.children.insert(first_char, child);
        }
      },
      None => {
        self.values.push(value);
      }
    }
  }

  pub fn find_child(&self, mut key: Chars) -> Option<&TrieNode<T>> {
    match key.next() {
      Some(first_char) => match self.children.get(&first_char) {
        Some(child) => child.find_child(key),
        None => None,
      },
      None => Some(self),
    }
  }
}
