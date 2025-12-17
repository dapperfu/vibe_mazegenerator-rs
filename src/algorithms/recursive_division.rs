use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub struct RecursiveDivision;

impl MazeGenerator for RecursiveDivision {
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
        // Remove all walls initially
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

        // Recursively divide the space
        fn divide(
            maze: &mut Maze,
            rng: &mut ChaCha8Rng,
            x1: u32,
            y1: u32,
            x2: u32,
            y2: u32,
            complexity: f64,
        ) {
            let width = x2 - x1;
            let height = y2 - y1;

            // Base case: region too small to divide
            if width < 2 || height < 2 {
                return;
            }

            // Choose orientation based on dimensions and complexity
            let horizontal = if width < height {
                true
            } else if height < width {
                false
            } else {
                // Equal dimensions: use complexity to decide
                if complexity < 0.1 {
                    true // Deterministic: prefer horizontal
                } else {
                    rng.gen_bool(0.5) // Random
                }
            };

            if horizontal {
                // Draw horizontal wall
                let wall_y = if complexity < 0.1 {
                    y1 + height / 2 // Deterministic: middle
                } else {
                    // Random position with bias toward middle
                    let min_y = y1 + 1;
                    let max_y = y2 - 1;
                    let center = (min_y + max_y) / 2;
                    let range = ((max_y - min_y) as f64 * complexity).max(1.0) as u32;
                    let offset = rng.gen_range(0..=range);
                    if rng.gen_bool(0.5) {
                        center.saturating_sub(offset).max(min_y).min(max_y)
                    } else {
                        (center + offset).min(max_y).max(min_y)
                    }
                };

                // Add wall (except for a gap)
                let gap_x = if complexity < 0.1 {
                    x1 + width / 2 // Deterministic: middle gap
                } else {
                    // Random gap position
                    rng.gen_range(x1..x2)
                };

                for x in x1..x2 {
                    if x != gap_x && wall_y > 0 && wall_y < maze.height() {
                        // Add wall back by setting cell walls directly
                        // We need to get cells separately to avoid borrow checker issues
                        {
                            if let Some(mut_c1) = maze.get_cell_mut(x, wall_y - 1) {
                                mut_c1.south = true;
                            }
                        }
                        {
                            if let Some(mut_c2) = maze.get_cell_mut(x, wall_y) {
                                mut_c2.north = true;
                            }
                        }
                    }
                }

                // Recursively divide the two regions
                divide(maze, rng, x1, y1, x2, wall_y, complexity);
                divide(maze, rng, x1, wall_y, x2, y2, complexity);
            } else {
                // Draw vertical wall
                let wall_x = if complexity < 0.1 {
                    x1 + width / 2 // Deterministic: middle
                } else {
                    // Random position with bias toward middle
                    let min_x = x1 + 1;
                    let max_x = x2 - 1;
                    let center = (min_x + max_x) / 2;
                    let range = ((max_x - min_x) as f64 * complexity).max(1.0) as u32;
                    let offset = rng.gen_range(0..=range);
                    if rng.gen_bool(0.5) {
                        center.saturating_sub(offset).max(min_x).min(max_x)
                    } else {
                        (center + offset).min(max_x).max(min_x)
                    }
                };

                // Add wall (except for a gap)
                let gap_y = if complexity < 0.1 {
                    y1 + height / 2 // Deterministic: middle gap
                } else {
                    // Random gap position
                    rng.gen_range(y1..y2)
                };

                for y in y1..y2 {
                    if y != gap_y && wall_x > 0 && wall_x < maze.width() {
                        // Add wall back
                        // We need to get cells separately to avoid borrow checker issues
                        {
                            if let Some(mut_c1) = maze.get_cell_mut(wall_x - 1, y) {
                                mut_c1.east = true;
                            }
                        }
                        {
                            if let Some(mut_c2) = maze.get_cell_mut(wall_x, y) {
                                mut_c2.west = true;
                            }
                        }
                    }
                }

                // Recursively divide the two regions
                divide(maze, rng, x1, y1, wall_x, y2, complexity);
                divide(maze, rng, wall_x, y1, x2, y2, complexity);
            }
        }

        divide(&mut maze, &mut rng, 0, 0, width, height, complexity);

        maze
    }
}

