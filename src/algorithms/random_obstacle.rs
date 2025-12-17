use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

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

pub struct RandomObstacle;

impl MazeGenerator for RandomObstacle {
    fn generate(&self, width: u32, height: u32, complexity: f64, seed: Option<u64>) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => {
                let seed = rand::thread_rng().gen();
                ChaCha8Rng::seed_from_u64(seed)
            }
        };

        // Start with all walls removed (open space)
        for y in 0..height {
            for x in 0..width {
                if x < width - 1 {
                    maze.remove_wall(x, y, x + 1, y);
                }
                if y < height - 1 {
                    maze.remove_wall(x, y, x, y + 1);
                }
            }
        }

        // Place random walls (obstacles) based on complexity
        // Higher complexity = more walls
        let obstacle_density = complexity * 0.4; // Range: 0.0 to 0.4
        let total_edges = (width - 1) * height + width * (height - 1);
        let num_obstacles = (total_edges as f64 * obstacle_density) as u32;

        // Create list of all possible walls to add back
        let mut walls = Vec::new();
        for y in 0..height {
            for x in 0..width {
                if x < width - 1 {
                    walls.push((x, y, x + 1, y));
                }
                if y < height - 1 {
                    walls.push((x, y, x, y + 1));
                }
            }
        }

        // Shuffle and select walls to add back
        if complexity > 0.0 {
            for i in 0..walls.len() {
                let j = rng.gen_range(i..walls.len());
                walls.swap(i, j);
            }
        }

        // Add back selected walls
        for i in 0..num_obstacles.min(walls.len() as u32) as usize {
            let (x1, y1, x2, y2) = walls[i];
            // Add wall back by setting cell walls directly
            // Get cells separately to avoid borrow checker issues
            {
                if let Some(mut_c1) = maze.get_cell_mut(x1, y1) {
                    if x2 == x1 + 1 {
                        // Vertical wall (east-west)
                        mut_c1.east = true;
                    } else if y2 == y1 + 1 {
                        // Horizontal wall (north-south)
                        mut_c1.south = true;
                    }
                }
            }
            {
                if let Some(mut_c2) = maze.get_cell_mut(x2, y2) {
                    if x2 == x1 + 1 {
                        // Vertical wall (east-west)
                        mut_c2.west = true;
                    } else if y2 == y1 + 1 {
                        // Horizontal wall (north-south)
                        mut_c2.north = true;
                    }
                }
            }
        }

        // Ensure connectivity using union-find
        let mut uf = UnionFind::new((width * height) as usize);

        // First, find all connected components
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize;
                for neighbor in maze.get_accessible_neighbors(x, y) {
                    let nidx = (neighbor.1 * width + neighbor.0) as usize;
                    uf.union(idx, nidx);
                }
            }
        }

        // Find all components
        let mut components: std::collections::HashMap<usize, Vec<(u32, u32)>> =
            std::collections::HashMap::new();
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize;
                let root = uf.find(idx);
                components.entry(root).or_insert_with(Vec::new).push((x, y));
            }
        }

        // Connect components by carving additional paths
        let component_list: Vec<Vec<(u32, u32)>> = components.into_values().collect();
        if component_list.len() > 1 {
            for i in 0..(component_list.len() - 1) {
                let comp1 = &component_list[i];
                let comp2 = &component_list[i + 1];
                
                // Find closest pair
                let mut min_dist = u32::MAX;
                let mut best_pair = None;
                for &(x1, y1) in comp1 {
                    for &(x2, y2) in comp2 {
                        let dist = (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs();
                        if dist < min_dist as i32 {
                            min_dist = dist as u32;
                            best_pair = Some((x1, y1, x2, y2));
                        }
                    }
                }

                // Carve path
                if let Some((x1, y1, x2, y2)) = best_pair {
                    let mut cx = x1;
                    let mut cy = y1;
                    while cx != x2 {
                        let next_x = if cx < x2 { cx + 1 } else { cx - 1 };
                        maze.remove_wall(cx, cy, next_x, cy);
                        cx = next_x;
                    }
                    while cy != y2 {
                        let next_y = if cy < y2 { cy + 1 } else { cy - 1 };
                        maze.remove_wall(cx, cy, cx, next_y);
                        cy = next_y;
                    }
                }
            }
        }

        // Ensure connectivity from start to end
        maze.ensure_connectivity();

        maze
    }
}

