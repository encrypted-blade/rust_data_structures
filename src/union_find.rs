/// A disjoint-set (Union-Find) data structure with path compression
/// and union by rank.
///
/// Elements are identified by `usize` indices in the range `0..size`.
///
/// # Examples
///
/// ```
/// use rust_data_structures::UnionFind;
///
/// let mut uf = UnionFind::new(5);
/// uf.union(0, 1);
/// uf.union(1, 2);
/// assert!(uf.connected(0, 2));
/// assert!(!uf.connected(0, 3));
/// assert_eq!(uf.count(), 3); // 3 disjoint sets remain
/// ```
#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    /// Creates a new `UnionFind` with `size` elements, each in its own set.
    pub fn new(size: usize) -> Self {
        let parent: Vec<usize> = (0..size).collect();
        let rank = vec![0; size];
        Self {
            parent,
            rank,
            count: size,
        }
    }

    /// Finds the representative (root) of the set containing `x`.
    /// Applies path compression for future efficiency.
    ///
    /// # Panics
    ///
    /// Panics if `x >= size`.
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Merges the sets containing `x` and `y`. Returns `true` if they were separate.
    ///
    /// # Panics
    ///
    /// Panics if `x >= size` or `y >= size`.
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }

        self.count -= 1;
        true
    }

    /// Returns `true` if `x` and `y` belong to the same set.
    ///
    /// # Panics
    ///
    /// Panics if `x >= size` or `y >= size`.
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Returns the current number of disjoint sets.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns the size of the set containing `x`.
    ///
    /// # Panics
    ///
    /// Panics if `x >= size`.
    pub fn set_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        let mut size = 0;
        for i in 0..self.parent.len() {
            if self.find(i) == root {
                size += 1;
            }
        }
        size
    }
}
