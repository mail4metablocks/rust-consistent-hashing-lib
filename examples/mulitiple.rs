extern crate consistent_hash;

use consistent_hash::ConsistentHash;

fn main() {
    let mut hash = ConsistentHash::new();
    hash.add_node(consistent_hash::Node::new(0, "Node 1"));
    hash.add_node(consistent_hash::Node::new(1, "Node 2"));
    hash.add_node(consistent_hash::Node::new(2, "Node 3"));

    let keys = vec!["key1", "key2", "key3", "key4", "key5", "key6"];
    for key in &keys {
        println!("{}: {:?}", key, hash.get_node(key));
    }
}
