use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub struct Sidewinder;

impl MazeGenerator for Sidewinder {
    fn generate(&self, width: u32, height: u32, complexity: f64, seed: Option<u64>) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => {
                let seed = rand::thread_rng().gen();
                ChaCha8Rng::seed_from_u64(seed)
            }
        };

        // Process row by row, top to bottom
        for y in 0..height {
            let mut run_start = 0;
            
            for x in 0..width {
                // Decide whether to end the current run
                // Complexity controls run length: lower = longer runs, higher = shorter runs
                let should_end_run = if x == width - 1 {
                    // Always end at the right edge
                    true
                } else {
                    // Probability of ending run increases with complexity
                    // At complexity 0.0: very long runs (low probability to end)
                    // At complexity 1.0: very short runs (high probability to end)
                    let end_probability = 0.3 + complexity * 0.5; // Range: 0.3 to 0.8
                    rng.gen::<f64>() < end_probability
                };

                if should_end_run {
                    // End the run: carve east if not at right edge
                    if x < width - 1 {
                        maze.remove_wall(x, y, x + 1, y);
                    }
                    
                    // Link upward to previous row (if not on top row)
                    if y > 0 {
                        // Choose a random cell from the run to link upward
                        let link_x = if run_start == x {
                            x
                        } else {
                            rng.gen_range(run_start..=x)
                        };
                        maze.remove_wall(link_x, y, link_x, y - 1);
                    }
                    
                    run_start = x + 1;
                } else {
                    // Continue the run: carve east
                    if x < width - 1 {
                        maze.remove_wall(x, y, x + 1, y);
                    }
                }
            }
        }

        maze
    }
}

