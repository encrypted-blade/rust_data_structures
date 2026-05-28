use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

/// A generic binary search tree (BST).
///
/// Elements must implement [`Ord`]. Duplicate values are inserted into the
/// right subtree.
///
/// # Examples
///
/// ```
/// use rust_data_structures::BinarySearchTree;
///
/// let mut tree = BinarySearchTree::new();
/// tree.insert(5);
/// tree.insert(3);
/// tree.insert(7);
/// assert!(tree.contains(&3));
/// assert_eq!(tree.min(), Some(&3));
/// assert_eq!(tree.max(), Some(&7));
/// ```
#[derive(Debug, Clone)]
pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
    len: usize,
}

/// An in-order iterator over references to the elements of a [`BinarySearchTree`].
pub struct Iter<'a, T> {
    stack: Vec<&'a Node<T>>,
}

impl<T> BinarySearchTree<T> {
    /// Creates an empty binary search tree.
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    /// Returns `true` if the tree contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of elements in the tree.
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Ord> BinarySearchTree<T> {
    /// Inserts a value into the tree.
    pub fn insert(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            left: None,
            right: None,
        });
        match self.root {
            None => self.root = Some(new_node),
            Some(ref mut node) => Self::insert_recursive(node, new_node),
        }
        self.len += 1;
    }

    fn insert_recursive(current: &mut Node<T>, new_node: Box<Node<T>>) {
        if new_node.value < current.value {
            match current.left {
                None => current.left = Some(new_node),
                Some(ref mut left) => Self::insert_recursive(left, new_node),
            }
        } else {
            match current.right {
                None => current.right = Some(new_node),
                Some(ref mut right) => Self::insert_recursive(right, new_node),
            }
        }
    }

    /// Returns `true` if the tree contains the given value.
    pub fn contains(&self, value: &T) -> bool {
        Self::contains_recursive(&self.root, value)
    }

    fn contains_recursive(node: &Option<Box<Node<T>>>, value: &T) -> bool {
        match node {
            None => false,
            Some(n) => match value.cmp(&n.value) {
                Ordering::Equal => true,
                Ordering::Less => Self::contains_recursive(&n.left, value),
                Ordering::Greater => Self::contains_recursive(&n.right, value),
            },
        }
    }

    /// Returns a reference to the smallest element, or `None` if empty.
    pub fn min(&self) -> Option<&T> {
        self.root.as_ref().map(|node| {
            let mut current = node;
            while let Some(ref left) = current.left {
                current = left;
            }
            &current.value
        })
    }

    /// Returns a reference to the largest element, or `None` if empty.
    pub fn max(&self) -> Option<&T> {
        self.root.as_ref().map(|node| {
            let mut current = node;
            while let Some(ref right) = current.right {
                current = right;
            }
            &current.value
        })
    }

    /// Removes a value from the tree. Returns `true` if the value was present.
    pub fn remove(&mut self, value: &T) -> bool {
        let (new_root, removed) = Self::remove_recursive(self.root.take(), value);
        self.root = new_root;
        if removed {
            self.len -= 1;
        }
        removed
    }

    fn remove_recursive(node: Option<Box<Node<T>>>, value: &T) -> (Option<Box<Node<T>>>, bool) {
        match node {
            None => (None, false),
            Some(mut n) => match value.cmp(&n.value) {
                Ordering::Less => {
                    let (new_left, removed) = Self::remove_recursive(n.left.take(), value);
                    n.left = new_left;
                    (Some(n), removed)
                }
                Ordering::Greater => {
                    let (new_right, removed) = Self::remove_recursive(n.right.take(), value);
                    n.right = new_right;
                    (Some(n), removed)
                }
                Ordering::Equal => match (n.left.take(), n.right.take()) {
                    (None, None) => (None, true),
                    (Some(left), None) => (Some(left), true),
                    (None, Some(right)) => (Some(right), true),
                    (Some(left), Some(right)) => {
                        let (new_right, min_val) = Self::remove_min(right);
                        n.value = min_val;
                        n.left = Some(left);
                        n.right = new_right;
                        (Some(n), true)
                    }
                },
            },
        }
    }

    /// Removes and returns the minimum value from a subtree.
    fn remove_min(mut node: Box<Node<T>>) -> (Option<Box<Node<T>>>, T) {
        if let Some(left) = node.left.take() {
            let (new_left, min_val) = Self::remove_min(left);
            node.left = new_left;
            (Some(node), min_val)
        } else {
            let val = node.value;
            (node.right.take(), val)
        }
    }

    /// Returns an iterator over the tree in ascending (in-order) order.
    pub fn iter(&self) -> Iter<'_, T> {
        let mut stack = Vec::new();
        let mut current = self.root.as_deref();
        while let Some(node) = current {
            stack.push(node);
            current = node.left.as_deref();
        }
        Iter { stack }
    }
}

impl<T> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|node| {
            let mut current = node.right.as_deref();
            while let Some(n) = current {
                self.stack.push(n);
                current = n.left.as_deref();
            }
            &node.value
        })
    }
}

impl<'a, T: Ord> IntoIterator for &'a BinarySearchTree<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
