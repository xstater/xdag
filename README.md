# XDag
A simple DAG (Directed Acyclic Graph) library
# Note
This lib just provides a data-structure to store DAG with checking.
It doesn't contain any algorithm about DAG.
# Docs
[docs.rs](https://docs.rs/xdag/)
# Examples
```Rust
// Create a new DAG
let mut dag = Dag::new();
// insert 3 nodes with data '()'
dag.insert_node(2, ());
dag.insert_node(4, ());
dag.insert_node(3, ());
// insert 2 edges with data '()'
dag.insert_edge(2, 3, ()).unwrap();
dag.insert_edge(2, 4, ()).unwrap();
// Get all roots and leaves in DAG
let roots = dag.roots().map(|(id, _)| id).collect::<Vec<_>>();
let leaves = dag.leaves().map(|(id, _)| id).collect::<Vec<_>>();

assert_eq!(&roots, &[2]);
for id in [3, 4].iter() {
    assert!(leaves.contains(id))
}
```
