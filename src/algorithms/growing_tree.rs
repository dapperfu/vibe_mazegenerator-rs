use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

pub struct GrowingTree;

impl MazeGenerator for GrowingTree {
    fn generate(&self, width: u32, height: u32, complexity: f64, seed: Option<u64>) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => {
                let seed = rand::thread_rng().gen();
                ChaCha8Rng::seed_from_u64(seed)
            }
        };
        let mut active = Vec::new();
        let mut visited = HashSet::new();

        // Start at a random cell
        let start_x = rng.gen_range(0..width);
        let start_y = rng.gen_range(0..height);
        active.push((start_x, start_y));
        visited.insert((start_x, start_y));

        while !active.is_empty() {
            // Select active cell based on policy (determined by complexity)
            let current_idx = if complexity < 0.25 {
                // Newest (stack-like, DFS-like): always take last
                active.len() - 1
            } else if complexity < 0.5 {
                // Random: pick random active cell
                rng.gen_range(0..active.len())
            } else if complexity < 0.75 {
                // Oldest (queue-like, BFS-like): always take first
                0
            } else {
                // Mixed: weighted random between newest and oldest
                if rng.gen::<f64>() < 0.5 {
                    active.len() - 1 // newest
                } else {
                    0 // oldest
                }
            };

            let (x, y) = active[current_idx];

            // Get unvisited neighbors
            let neighbors: Vec<(u32, u32)> = maze
                .get_neighbors(x, y)
                .into_iter()
                .filter(|&(nx, ny)| !visited.contains(&(nx, ny)))
                .collect();

            if !neighbors.is_empty() {
                // Choose neighbor (random selection, complexity affects this)
                let next_idx = if neighbors.len() == 1 {
                    0
                } else if complexity < 0.1 {
                    // Very low complexity: prefer first neighbor
                    0
                } else {
                    rng.gen_range(0..neighbors.len())
                };
                let (nx, ny) = neighbors[next_idx];

                // Remove wall between current and chosen neighbor
                maze.remove_wall(x, y, nx, ny);

                // Mark neighbor as visited and add to active set
                visited.insert((nx, ny));
                active.push((nx, ny));
            } else {
                // No unvisited neighbors, remove from active set
                active.remove(current_idx);
            }
        }

        maze
    }
}

