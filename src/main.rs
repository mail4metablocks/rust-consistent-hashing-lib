extern crate md5;

use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

struct Node<T> {
    key: u64,
    value: T,
}

impl<T> Node<T> {
    fn new(key: u64, value: T) -> Self {
        Self { key, value }
    }
}

struct ConsistentHash<T> {
    nodes: BTreeMap<u64, T>,
}

impl<T> ConsistentHash<T> {
    fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
        }
    }

    fn add_node(&mut self, node: Node<T>) {
        self.nodes.insert(node.key, node.value);
    }

    fn get_node(&self, key: &str) -> Option<&T> {
        let mut hasher = md5::Md5::new();
        hasher.write(key.as_bytes());
        let hash = hasher.finish();
        let key = hash.iter().fold(0, |acc, &b| (acc << 8) | u64::from(b));
        self.nodes.range(..=key).next_back().map(|(_, value)| value)
    }
}

fn main() {
    let mut hash = ConsistentHash::new();
    hash.add_node(Node::new(0, "Node 1"));
    hash.add_node(Node::new(1, "Node 2"));
    hash.add_node(Node::new(2, "Node 3"));

    println!("{:?}", hash.get_node("key1"));
    println!("{:?}", hash.get_node("key2"));
    println!("{:?}", hash.get_node("key3"));
}
