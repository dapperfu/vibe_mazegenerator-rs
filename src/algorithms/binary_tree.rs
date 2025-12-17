use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub struct BinaryTree;

impl MazeGenerator for BinaryTree {
    fn generate(&self, width: u32, height: u32, complexity: f64, seed: Option<u64>) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => {
                let seed = rand::thread_rng().gen();
                ChaCha8Rng::seed_from_u64(seed)
            }
        };

        // For each cell, carve either north or east
        // Complexity controls direction bias: 0.0 = always first option, 1.0 = random
        for y in 0..height {
            for x in 0..width {
                let mut options = Vec::new();
                
                // Can carve north if not on top row
                if y > 0 {
                    options.push((x, y - 1)); // north
                }
                
                // Can carve east if not on rightmost column
                if x < width - 1 {
                    options.push((x + 1, y)); // east
                }

                if !options.is_empty() {
                    let target = if options.len() == 1 {
                        options[0]
                    } else if complexity < 0.1 {
                        // Very low complexity: always choose first (north if available, else east)
                        options[0]
                    } else {
                        // Higher complexity: more randomness
                        let bias = (1.0 - complexity).max(0.0);
                        if rng.gen::<f64>() < bias {
                            // Bias toward first option when complexity is low
                            options[0]
                        } else {
                            options[rng.gen_range(0..options.len())]
                        }
                    };
                    
                    maze.remove_wall(x, y, target.0, target.1);
                }
            }
        }

        maze
    }
}

