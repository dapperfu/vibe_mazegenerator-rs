use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

pub struct AldousBroder;

impl MazeGenerator for AldousBroder {
    fn generate(&self, width: u32, height: u32, complexity: f64, seed: Option<u64>) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => {
                let seed = rand::thread_rng().gen();
                ChaCha8Rng::seed_from_u64(seed)
            }
        };
        let mut visited = HashSet::new();

        // Start at a random cell
        let mut current_x = rng.gen_range(0..width);
        let mut current_y = rng.gen_range(0..height);
        visited.insert((current_x, current_y));

        let total_cells = width * height;
        let mut visited_count = 1;

        // Random walk until all cells are visited
        while visited_count < total_cells {
            let neighbors = maze.get_neighbors(current_x, current_y);
            if neighbors.is_empty() {
                break;
            }

            // Choose neighbor based on complexity
            // Lower complexity = prefer first neighbor (more deterministic walk)
            // Higher complexity = more random selection
            let next_idx = if neighbors.len() == 1 {
                0
            } else if complexity < 0.1 {
                // Very low complexity: prefer first neighbor (more deterministic)
                if rng.gen::<f64>() < 0.8 {
                    0
                } else {
                    rng.gen_range(1..neighbors.len())
                }
            } else {
                // Higher complexity: fully random
                rng.gen_range(0..neighbors.len())
            };
            let (next_x, next_y) = neighbors[next_idx];

            // If the neighbor hasn't been visited, remove wall and mark as visited
            if !visited.contains(&(next_x, next_y)) {
                maze.remove_wall(current_x, current_y, next_x, next_y);
                visited.insert((next_x, next_y));
                visited_count += 1;
            } else {
                // Even if visited, sometimes remove wall based on complexity
                // Higher complexity = more random wall removals (creates loops)
                // At complexity 0.0: no loops (perfect maze)
                // At complexity 1.0: many loops (complex maze)
                if complexity > 0.0 && rng.gen::<f64>() < complexity * 0.5 {
                    maze.remove_wall(current_x, current_y, next_x, next_y);
                }
            }

            // Move to the neighbor
            current_x = next_x;
            current_y = next_y;
        }

        maze
    }
}

