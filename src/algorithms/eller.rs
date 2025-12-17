use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;

pub struct Eller;

impl MazeGenerator for Eller {
    fn generate(&self, width: u32, height: u32, complexity: f64, seed: Option<u64>) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => {
                let seed = rand::thread_rng().gen();
                ChaCha8Rng::seed_from_u64(seed)
            }
        };

        // Union-Find for tracking sets
        struct UnionFind {
            parent: Vec<usize>,
        }

        impl UnionFind {
            fn new(size: usize) -> Self {
                UnionFind {
                    parent: (0..size).collect(),
                }
            }

            fn find(&mut self, x: usize) -> usize {
                if self.parent[x] != x {
                    self.parent[x] = self.find(self.parent[x]);
                }
                self.parent[x]
            }

            fn union(&mut self, x: usize, y: usize) {
                let root_x = self.find(x);
                let root_y = self.find(y);
                if root_x != root_y {
                    self.parent[root_y] = root_x;
                }
            }
        }

        let mut uf = UnionFind::new((width * height) as usize);
        let mut row_sets: Vec<usize> = (0..width).map(|x| x as usize).collect();

        // Process each row
        for y in 0..height {
            // Merge cells within the row (horizontal connections)
            for x in 0..(width - 1) {
                let idx1 = (y * width + x) as usize;
                let idx2 = (y * width + x + 1) as usize;
                let set1 = uf.find(idx1);
                let set2 = uf.find(idx2);

                // Decide whether to merge based on complexity
                // At complexity 0.0: merge deterministically (every other)
                // At complexity 1.0: merge randomly
                let should_merge = if complexity < 0.1 {
                    x % 2 == 0 // Deterministic pattern
                } else {
                    // Random merge with probability based on complexity
                    let merge_prob = 0.3 + complexity * 0.4; // Range: 0.3 to 0.7
                    rng.gen::<f64>() < merge_prob
                };

                if should_merge && set1 != set2 {
                    maze.remove_wall(x, y, x + 1, y);
                    uf.union(idx1, idx2);
                    row_sets[x as usize] = uf.find(idx1);
                    row_sets[(x + 1) as usize] = row_sets[x as usize];
                }
            }

            // Create vertical connections to next row (if not last row)
            if y < height - 1 {
                // Group cells by their set
                let mut set_to_cells: HashMap<usize, Vec<u32>> = HashMap::new();
                for x in 0..width {
                    let idx = (y * width + x) as usize;
                    let set = uf.find(idx);
                    set_to_cells.entry(set).or_insert_with(Vec::new).push(x);
                }

                // For each set, create at least one vertical connection
                for (_set, cells) in &set_to_cells {
                    // Number of vertical connections based on complexity
                    // At complexity 0.0: exactly 1 connection per set
                    // At complexity 1.0: more connections (up to all cells in set)
                    let num_connections = if complexity < 0.1 {
                        1
                    } else {
                        let min_conn = 1;
                        let max_conn = cells.len();
                        let extra_prob = complexity;
                        let mut count = min_conn;
                        for _ in 1..max_conn {
                            if rng.gen::<f64>() < extra_prob {
                                count += 1;
                            }
                        }
                        count.min(max_conn)
                    };

                    // Shuffle cells for random selection
                    let mut cells_to_connect = cells.clone();
                    if complexity > 0.0 {
                        for i in 0..cells_to_connect.len() {
                            let j = rng.gen_range(i..cells_to_connect.len());
                            cells_to_connect.swap(i, j);
                        }
                    }

                    // Create vertical connections
                    for i in 0..num_connections.min(cells_to_connect.len()) {
                        let x = cells_to_connect[i];
                        let idx1 = (y * width + x) as usize;
                        let idx2 = ((y + 1) * width + x) as usize;
                        maze.remove_wall(x, y, x, y + 1);
                        uf.union(idx1, idx2);
                    }
                }

                // Update row_sets for next row
                for x in 0..width {
                    let idx = ((y + 1) * width + x) as usize;
                    row_sets[x as usize] = uf.find(idx);
                }
            }
        }

        maze
    }
}

