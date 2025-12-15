use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use std::collections::HashSet;

pub struct AldousBroder;

impl MazeGenerator for AldousBroder {
    fn generate(&self, width: u32, height: u32, complexity: f64) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = rand::thread_rng();
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

            // Choose a random neighbor
            let next_idx = rng.gen_range(0..neighbors.len());
            let (next_x, next_y) = neighbors[next_idx];

            // If the neighbor hasn't been visited, remove wall and mark as visited
            if !visited.contains(&(next_x, next_y)) {
                maze.remove_wall(current_x, current_y, next_x, next_y);
                visited.insert((next_x, next_y));
                visited_count += 1;
            } else {
                // Even if visited, sometimes remove wall based on complexity
                // Higher complexity = more random wall removals (creates loops)
                if complexity > 0.0 && rng.gen::<f64>() < complexity * 0.1 {
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

