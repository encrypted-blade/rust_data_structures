# rust_data_structures

A Rust library providing common data structures not included in the standard library.

## Included Structures

| Structure | File | Description |
|-----------|------|-------------|
| `Stack<T>` | `src/stack.rs` | LIFO stack backed by `Vec<T>` |
| `Queue<T>` | `src/queue.rs` | FIFO queue with a custom linked list |
| `BinarySearchTree<T>` | `src/bst.rs` | Ordered binary search tree |
| `Trie` | `src/trie.rs` | Prefix tree for string keys |
| `Graph<T, E>` | `src/graph.rs` | Adjacency-list graph with DFS/BFS |
| `UnionFind` | `src/union_find.rs` | Disjoint-set with path compression |

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
rust_data_structures = { path = "../path/to/rust_data_structures" }
```

### Example

```rust
use rust_data_structures::{Stack, Queue, BinarySearchTree, Trie, Graph, UnionFind};

fn main() {
    // Stack
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.pop(), Some(2));

    // Queue
    let mut queue = Queue::new();
    queue.enqueue("a");
    queue.enqueue("b");
    assert_eq!(queue.dequeue(), Some("a"));

    // Binary Search Tree
    let mut bst = BinarySearchTree::new();
    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    assert!(bst.contains(&3));
    assert_eq!(bst.iter().collect::<Vec<_>>(), vec![&3, &5, &7]);

    // Trie
    let mut trie = Trie::new();
    trie.insert("hello");
    assert!(trie.search("hello"));
    assert!(trie.starts_with("hell"));

    // Graph
    let mut graph = Graph::new(false);
    graph.add_edge("A", "B", 1);
    graph.add_edge("B", "C", 2);
    let dfs = graph.dfs(&"A");
    println!("DFS: {:?}", dfs);

    // Union-Find
    let mut uf = UnionFind::new(5);
    uf.union(0, 1);
    uf.union(1, 2);
    assert!(uf.connected(0, 2));
    assert_eq!(uf.count(), 3);
}
```

## Testing

```bash
cd rust_data_structures
cargo test
```

## License

MIT OR Apache-2.0
