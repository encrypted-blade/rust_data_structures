use std::collections::HashMap;

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

/// A prefix tree (trie) for efficient storage and retrieval of strings.
///
/// # Examples
///
/// ```
/// use rust_data_structures::Trie;
///
/// let mut trie = Trie::new();
/// trie.insert("hello");
/// trie.insert("world");
/// assert!(trie.search("hello"));
/// assert!(!trie.search("hell"));
/// assert!(trie.starts_with("hell"));
/// ```
#[derive(Debug, Default)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    /// Creates an empty trie.
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    /// Inserts a word into the trie.
    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for ch in word.chars() {
            current = current.children.entry(ch).or_default();
        }
        current.is_end_of_word = true;
    }

    /// Returns `true` if the exact word is present in the trie.
    pub fn search(&self, word: &str) -> bool {
        self.find_node(word).is_some_and(|node| node.is_end_of_word)
    }

    /// Returns `true` if any word in the trie starts with the given prefix.
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.find_node(prefix).is_some()
    }

    /// Deletes a word from the trie. Returns `true` if the word existed.
    pub fn delete(&mut self, word: &str) -> bool {
        let chars: Vec<char> = word.chars().collect();
        Self::delete_recursive(&mut self.root, &chars, 0)
    }

    /// Returns `true` if the trie contains no words.
    pub fn is_empty(&self) -> bool {
        self.root.children.is_empty()
    }

    fn find_node(&self, word: &str) -> Option<&TrieNode> {
        let mut current = &self.root;
        for ch in word.chars() {
            current = current.children.get(&ch)?;
        }
        Some(current)
    }

    fn delete_recursive(node: &mut TrieNode, chars: &[char], index: usize) -> bool {
        if index == chars.len() {
            if !node.is_end_of_word {
                return false;
            }
            node.is_end_of_word = false;
            return node.children.is_empty();
        }

        let ch = chars[index];
        let child = match node.children.get_mut(&ch) {
            Some(child) => child,
            None => return false,
        };

        let should_delete_child = Self::delete_recursive(child, chars, index + 1);

        if should_delete_child {
            node.children.remove(&ch);
        }

        node.children.is_empty() && !node.is_end_of_word
    }
}
