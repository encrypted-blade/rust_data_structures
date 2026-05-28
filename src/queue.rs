use std::ptr;

struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

/// A FIFO (First-In-First-Out) queue implemented with a singly-linked list.
///
/// This provides O(1) enqueue and dequeue operations.
///
/// # Examples
///
/// ```
/// use rust_data_structures::Queue;
///
/// let mut queue = Queue::new();
/// queue.enqueue(1);
/// queue.enqueue(2);
/// queue.enqueue(3);
/// assert_eq!(queue.peek(), Some(&1));
/// assert_eq!(queue.dequeue(), Some(1));
/// assert_eq!(queue.dequeue(), Some(2));
/// assert_eq!(queue.len(), 1);
/// ```
pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    len: usize,
}

/// An owning iterator over a [`Queue`].
pub struct IntoIter<T>(Queue<T>);

impl<T> Queue<T> {
    /// Creates an empty queue.
    pub fn new() -> Self {
        Self {
            head: None,
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    /// Adds an element to the back of the queue.
    pub fn enqueue(&mut self, elem: T) {
        let mut new_tail = Box::new(Node { elem, next: None });
        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
        self.len += 1;
    }

    /// Removes and returns the front element, or `None` if empty.
    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            self.len -= 1;
            head.elem
        })
    }

    /// Returns a reference to the front element without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    /// Returns a mutable reference to the front element.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    /// Returns `true` if the queue contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of elements in the queue.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Clears the queue, removing all values.
    pub fn clear(&mut self) {
        while self.dequeue().is_some() {}
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Queue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        let mut current = self.head.as_deref();
        while let Some(node) = current {
            list.entry(&node.elem);
            current = node.next.as_deref();
        }
        list.finish()
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.dequeue()
    }
}
