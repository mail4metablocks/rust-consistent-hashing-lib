# Consistent Hashing

This is a simple implementation of consistent hashing in Rust. It allows you to add nodes to a hash and look up the node for a given key.
Usage

To use the consistent hashing library, add the following to your Cargo.toml file:

```
[dependencies]
consistent_hash = "0.1.0"

Then, in your Rust code, you can use the ConsistentHash struct to create a new hash and add nodes to it:

extern crate consistent_hash;

use consistent_hash::ConsistentHash;

fn main() {
    let mut hash = ConsistentHash::new();
    hash.add_node(consistent_hash::Node::new(0, "Node 1"));
    hash.add_node(consistent_hash::Node::new(1, "Node 2"));
    hash.add_node(consistent_hash::Node::new(2, "Node 3"));
}

You can then use the get_node method to look up the node for a given key:

let node = hash.get_node("key1");
```

## Examples

See the examples folder for example programs that demonstrate how to use the consistent hashing library.

### License

This library is distributed under the MIT license. See the LICENSE file for details.
