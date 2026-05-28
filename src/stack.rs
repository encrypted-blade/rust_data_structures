use std::iter::FromIterator;

/// A LIFO (Last-In-First-Out) stack backed by a `Vec<T>`.
///
/// # Examples
///
/// ```
/// use rust_data_structures::Stack;
///
/// let mut stack = Stack::new();
/// stack.push(10);
/// stack.push(20);
/// assert_eq!(stack.peek(), Some(&20));
/// assert_eq!(stack.pop(), Some(20));
/// assert_eq!(stack.pop(), Some(10));
/// assert_eq!(stack.pop(), None);
/// ```
#[derive(Clone, Default)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates an empty stack.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Creates an empty stack with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    /// Pushes an element onto the stack.
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Removes and returns the top element, or `None` if empty.
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Returns a reference to the top element without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    /// Returns a mutable reference to the top element.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.items.last_mut()
    }

    /// Returns `true` if the stack contains no elements.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns the number of elements in the stack.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns the number of elements the stack can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    /// Clears the stack, removing all values.
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stack").field("items", &self.items).finish()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Stack<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            items: Vec::from_iter(iter),
        }
    }
}
