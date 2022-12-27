extern crate consistent_hash;

use consistent_hash::ConsistentHash;

fn main() {
    let mut hash = ConsistentHash::new();
    hash.add_node(consistent_hash::Node::new(0, "Node 1"));
    hash.add_node(consistent_hash::Node::new(1, "Node 2"));
    hash.add_node(consistent_hash::Node::new(2, "Node 3"));

    println!("{:?}", hash.get_node("key1"));
    println!("{:?}", hash.get_node("key2"));
    println!("{:?}", hash.get_node("key3"));
}
