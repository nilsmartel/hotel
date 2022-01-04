# Hotel

Simple collection datastructure to associate data with unique keys.

## Properties and Advantages
The key is of type `usize`, and can therefore cheaply be passed around in favour of the actual data.

The advantage is that `keys` can be cloned unbeatable cheaply and no hashing of keys has to ever take place. This also means no collisions can ever happen.

`put`, `remove`, `get`, `take` are all O(1) operations.

The `Hotel` is backed by a `Vec` and is very _memory efficient_. Thanks to this it's very much efficient to use on modern CPUs by virtue of inheriting the `Vec`s cache-friendliness.


## Use cases
Here are examples

- Often used together with Maps in order to avoid hashing often and cloning keys.
- In some cases can be used as an high-performance `HashMap` replacement.
- Implementing graphs! Graphs can take many forms, in some way they can be found in almost every programm. Trees are simple, everything else is not so obvious to implement in rust, thanks to it's ownership rules. A `Hotel` is the perfect place to store ownership of the Nodes and manage edges as keys.

## Example

here's a simple Graph.
    A -> B -> C
```rust
let mut nodes: Hotel<Node>,
let mut edges: Vec<(usize, usize)>

let key1 = nodes.put(Node::new(a));
let key2 = nodes.put(Node::new(b));
let key3 = nodes.put(Node::new(c));
edges.push( (key1, key2) );
edges.push( (key2, key3) );
```
