//! A collection of common data structures not present in Rust's standard library.
//!
//! # Structures
//!
//! | Structure | Module | Description |
//! |-----------|--------|-------------|
//! | `Stack` | [`stack`] | LIFO wrapper with `Vec`-backed storage |
//! | `Queue` | [`queue`] | FIFO queue with a custom singly-linked list |
//! | `BinarySearchTree` | [`bst`] | Generic ordered binary search tree |
//! | `Trie` | [`trie`] | Prefix tree for efficient string storage |
//! | `Graph` | [`graph`] | Adjacency-list graph with DFS/BFS |
//! | `UnionFind` | [`union_find`] | Disjoint-set with path compression |
//!
//! # Example
//!
//! ```
//! use rust_data_structures::{Stack, Queue, BinarySearchTree, Trie, Graph, UnionFind};
//!
//! let mut stack = Stack::new();
//! stack.push(1);
//! stack.push(2);
//! assert_eq!(stack.pop(), Some(2));
//! ```

pub mod bst;
pub mod graph;
pub mod queue;
pub mod stack;
pub mod trie;
pub mod union_find;

pub use bst::BinarySearchTree;
pub use graph::Graph;
pub use queue::Queue;
pub use stack::Stack;
pub use trie::Trie;
pub use union_find::UnionFind;
