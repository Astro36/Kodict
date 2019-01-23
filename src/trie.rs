use std::collections::HashMap;
use std::str::Chars;

pub struct TrieMap<T> {
  root: TrieNode<T>,
  length: usize,
}

impl<T> TrieMap<T> {
  pub fn new() -> TrieMap<T> {
    TrieMap {
      root: TrieNode::new(),
      length: 0,
    }
  }

  pub fn clear(&mut self) {
    self.root = TrieNode::new();
    self.length = 0;
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

  pub fn get_mut(&mut self, key: String) -> Option<&mut Vec<T>> {
    match self.root.find_child_mut(key.chars()) {
      Some(node) => Some(&mut node.values),
      None => None,
    }
  }

  pub fn insert(&mut self, key: String, value: T) {
    self.root.append_child(key.chars(), value);
    self.length += 1;
  }

  pub fn is_empty(&self) -> bool {
    self.length == 0
  }

  pub fn len(&self) -> usize {
    self.length
  }

  pub fn values(&self) -> Vec<&T> {
    self
      .root
      .traverse()
      .into_iter()
      .flat_map(|node| &node.values)
      .collect()
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

  pub fn find_child_mut(&mut self, mut key: Chars) -> Option<&mut TrieNode<T>> {
    match key.next() {
      Some(first_char) => match self.children.get_mut(&first_char) {
        Some(child) => child.find_child_mut(key),
        None => None,
      },
      None => Some(self),
    }
  }

  pub fn traverse(&self) -> Vec<&TrieNode<T>> {
    let mut children: Vec<&TrieNode<T>> = self
      .children
      .iter()
      .flat_map(|(_, node)| node.traverse())
      .collect();
    children.push(self);
    children
  }
}
