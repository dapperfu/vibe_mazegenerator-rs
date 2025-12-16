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
            // Select an edge from frontier based on complexity
            // Lower complexity = prefer earlier edges (more deterministic growth)
            // Higher complexity = more random selection
            let edge_idx = if frontier.len() == 1 {
                0
            } else if complexity < 0.1 {
                // Very low complexity: always choose first edge (deterministic tree growth)
                0
            } else {
                // Higher complexity: bias toward random selection
                let bias = (1.0 - complexity).max(0.0);
                if rng.gen::<f64>() < bias {
                    // Bias toward first edge when complexity is low
                    if rng.gen::<f64>() < 0.7 {
                        0
                    } else {
                        rng.gen_range(1..frontier.len())
                    }
                } else {
                    rng.gen_range(0..frontier.len())
                }
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

