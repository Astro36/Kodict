use std::collections::HashMap;
use std::str::Chars;

pub struct Map<T> {
    root: Node<T>,
    length: usize,
}

impl<T> Map<T> {
    pub fn new() -> Map<T> {
        Map {
            root: Node::new(),
            length: 0,
        }
    }

    pub fn clear(&mut self) {
        self.root = Node::new();
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

    pub fn range(&self, key: String) -> Option<Vec<&T>> {
        match self.root.find_child(key.chars()) {
            Some(node) => Some(node.traverse()
                .into_iter()
                .flat_map(|node| &node.values)
                .collect()),
            None => None,
        }
    }

    pub fn values(&self) -> Vec<&T> {
        self.root
            .traverse()
            .into_iter()
            .flat_map(|node| &node.values)
            .collect()
    }
}

pub struct Node<T> {
    children: HashMap<char, Node<T>>,
    pub values: Vec<T>,
}

impl<T> Node<T> {
    pub fn new() -> Node<T> {
        Node {
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
                    let mut child = Node::new();
                    child.append_child(key, value);
                    self.children.insert(first_char, child);
                }
            },
            None => {
                self.values.push(value);
            }
        }
    }

    pub fn find_child(&self, mut key: Chars) -> Option<&Node<T>> {
        match key.next() {
            Some(first_char) => match self.children.get(&first_char) {
                Some(child) => child.find_child(key),
                None => None,
            },
            None => Some(self),
        }
    }

    pub fn find_child_mut(&mut self, mut key: Chars) -> Option<&mut Node<T>> {
        match key.next() {
            Some(first_char) => match self.children.get_mut(&first_char) {
                Some(child) => child.find_child_mut(key),
                None => None,
            },
            None => Some(self),
        }
    }

    pub fn traverse(&self) -> Vec<&Node<T>> {
        let mut children: Vec<&Node<T>> = self
            .children
            .iter()
            .flat_map(|(_, node)| node.traverse())
            .collect();
        children.push(self);
        children
    }
}
