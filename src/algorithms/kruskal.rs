use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;

/// Union-Find data structure for tracking connected components
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }

        true
    }
}

pub struct Kruskal;

impl MazeGenerator for Kruskal {
    fn generate(&self, width: u32, height: u32, complexity: f64) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = rand::thread_rng();
        let mut uf = UnionFind::new((width * height) as usize);

        // Create list of all edges (walls between adjacent cells)
        let mut edges = Vec::new();
        for y in 0..height {
            for x in 0..width {
                if x < width - 1 {
                    edges.push((x, y, x + 1, y));
                }
                if y < height - 1 {
                    edges.push((x, y, x, y + 1));
                }
            }
        }

        // Shuffle edges based on complexity
        // Higher complexity = more randomness in edge selection
        if complexity > 0.0 {
            for i in 0..edges.len() {
                let j = rng.gen_range(0..edges.len());
                edges.swap(i, j);
            }
        }

        // Process edges
        for (x1, y1, x2, y2) in edges {
            let idx1 = (y1 * width + x1) as usize;
            let idx2 = (y2 * width + x2) as usize;

            // If cells are in different sets, remove wall and union them
            if uf.find(idx1) != uf.find(idx2) {
                // Apply complexity-based filtering: higher complexity allows more edges
                if rng.gen::<f64>() < (1.0 - complexity * 0.3) || complexity < 0.1 {
                    maze.remove_wall(x1, y1, x2, y2);
                    uf.union(idx1, idx2);
                }
            }
        }

        maze
    }
}

