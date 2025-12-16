use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

pub struct Prim;

impl MazeGenerator for Prim {
    fn generate(&self, width: u32, height: u32, complexity: f64, seed: Option<u64>) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => {
                let seed = rand::thread_rng().gen();
                ChaCha8Rng::seed_from_u64(seed)
            }
        };
        let mut in_tree = HashSet::new();
        let mut frontier = Vec::new();

        // Start at a random cell
        let start_x = rng.gen_range(0..width);
        let start_y = rng.gen_range(0..height);
        in_tree.insert((start_x, start_y));
        
        // Add neighbors to frontier
        for neighbor in maze.get_neighbors(start_x, start_y) {
            frontier.push((start_x, start_y, neighbor.0, neighbor.1));
        }

        while !frontier.is_empty() {
            // Select a random edge from frontier
            let edge_idx = if complexity > 0.0 {
                // Higher complexity = more randomness in selection
                let range = (frontier.len() as f64 * (1.0 - complexity * 0.5)).max(1.0) as usize;
                rng.gen_range(0..range.min(frontier.len()))
            } else {
                rng.gen_range(0..frontier.len())
            };

            let (x1, y1, x2, y2) = frontier.remove(edge_idx);

            // If the destination cell is not in the tree, add it
            if !in_tree.contains(&(x2, y2)) {
                // Remove wall between cells
                maze.remove_wall(x1, y1, x2, y2);
                
                // Add destination to tree
                in_tree.insert((x2, y2));

                // Add new edges to frontier
                for neighbor in maze.get_neighbors(x2, y2) {
                    if !in_tree.contains(&neighbor) {
                        frontier.push((x2, y2, neighbor.0, neighbor.1));
                    }
                }
            }
        }

        maze
    }
}

