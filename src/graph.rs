use std::collections::{HashMap, HashSet, VecDeque};

/// An adjacency-list graph supporting both directed and undirected edges.
///
/// Nodes must implement [`Eq`], [`Hash`], and [`Clone`].
/// Edge weights default to `()` for unweighted graphs.
///
/// # Examples
///
/// ```
/// use rust_data_structures::Graph;
///
/// let mut graph = Graph::new(false); // undirected
/// graph.add_edge("A", "B", 1);
/// graph.add_edge("B", "C", 2);
/// let dfs = graph.dfs(&"A");
/// assert!(dfs.contains(&&"A"));
/// assert!(dfs.contains(&&"B"));
/// assert!(dfs.contains(&&"C"));
/// ```
#[derive(Debug, Clone)]
pub struct Graph<T, E = ()> {
    adjacency_list: HashMap<T, Vec<(T, E)>>,
    directed: bool,
}

impl<T: Eq + std::hash::Hash + Clone, E: Clone> Graph<T, E> {
    /// Creates a new graph.
    ///
    /// * `directed` — `true` for a directed graph, `false` for undirected.
    pub fn new(directed: bool) -> Self {
        Self {
            adjacency_list: HashMap::new(),
            directed,
        }
    }

    /// Adds a node to the graph. Has no effect if the node already exists.
    pub fn add_node(&mut self, node: T) {
        self.adjacency_list.entry(node).or_default();
    }

    /// Adds an edge between two nodes, creating the nodes if they do not exist.
    pub fn add_edge(&mut self, from: T, to: T, weight: E) {
        self.adjacency_list
            .entry(from.clone())
            .or_default()
            .push((to.clone(), weight.clone()));
        if !self.directed {
            self.adjacency_list
                .entry(to)
                .or_default()
                .push((from, weight));
        }
    }

    /// Returns `true` if the graph contains the given node.
    pub fn contains_node(&self, node: &T) -> bool {
        self.adjacency_list.contains_key(node)
    }

    /// Returns `true` if an edge exists from `from` to `to`.
    pub fn contains_edge(&self, from: &T, to: &T) -> bool {
        self.adjacency_list
            .get(from)
            .is_some_and(|edges| edges.iter().any(|(n, _)| n == to))
    }

    /// Removes a node and all incident edges. Returns `true` if it existed.
    pub fn remove_node(&mut self, node: &T) -> bool {
        if self.adjacency_list.remove(node).is_none() {
            return false;
        }
        for edges in self.adjacency_list.values_mut() {
            edges.retain(|(n, _)| n != node);
        }
        true
    }

    /// Removes an edge from `from` to `to`. Returns `true` if it existed.
    pub fn remove_edge(&mut self, from: &T, to: &T) -> bool {
        let mut removed = false;
        if let Some(edges) = self.adjacency_list.get_mut(from) {
            let original_len = edges.len();
            edges.retain(|(n, _)| n != to);
            removed = edges.len() < original_len;
        }
        if !self.directed {
            if let Some(edges) = self.adjacency_list.get_mut(to) {
                edges.retain(|(n, _)| n != from);
            }
        }
        removed
    }

    /// Returns the neighbors (adjacent nodes and edge weights) of a node.
    pub fn neighbors(&self, node: &T) -> Option<&Vec<(T, E)>> {
        self.adjacency_list.get(node)
    }

    /// Returns a vector of references to all nodes in the graph.
    pub fn nodes(&self) -> Vec<&T> {
        self.adjacency_list.keys().collect()
    }

    /// Returns `true` if the graph contains no nodes.
    pub fn is_empty(&self) -> bool {
        self.adjacency_list.is_empty()
    }

    /// Returns the number of nodes.
    pub fn node_count(&self) -> usize {
        self.adjacency_list.len()
    }

    /// Returns the number of edges.
    pub fn edge_count(&self) -> usize {
        let count: usize = self.adjacency_list.values().map(|v| v.len()).sum();
        if self.directed {
            count
        } else {
            count / 2
        }
    }

    /// Performs a depth-first search (DFS) starting from `start`.
    /// Returns a vector of visited nodes in discovery order.
    pub fn dfs<'a>(&'a self, start: &'a T) -> Vec<&'a T> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut stack = vec![start];

        while let Some(node) = stack.pop() {
            if visited.insert(node) {
                result.push(node);
                if let Some(neighbors) = self.adjacency_list.get(node) {
                    for (neighbor, _) in neighbors {
                        if !visited.contains(neighbor) {
                            stack.push(neighbor);
                        }
                    }
                }
            }
        }
        result
    }

    /// Performs a breadth-first search (BFS) starting from `start`.
    /// Returns a vector of visited nodes in level order.
    pub fn bfs<'a>(&'a self, start: &'a T) -> Vec<&'a T> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            if visited.insert(node) {
                result.push(node);
                if let Some(neighbors) = self.adjacency_list.get(node) {
                    for (neighbor, _) in neighbors {
                        if !visited.contains(neighbor) {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }
        result
    }
}
