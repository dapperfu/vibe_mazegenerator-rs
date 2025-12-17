use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

pub struct HuntAndKill;

impl MazeGenerator for HuntAndKill {
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

        loop {
            // Random walk carving until stuck
            let mut stuck = false;
            while !stuck {
                let neighbors = maze.get_neighbors(current_x, current_y);
                let unvisited_neighbors: Vec<(u32, u32)> = neighbors
                    .into_iter()
                    .filter(|&(nx, ny)| !visited.contains(&(nx, ny)))
                    .collect();

                if unvisited_neighbors.is_empty() {
                    stuck = true;
                } else {
                    // Choose neighbor based on complexity
                    let next_idx = if unvisited_neighbors.len() == 1 {
                        0
                    } else if complexity < 0.1 {
                        // Very low complexity: prefer first neighbor
                        if rng.gen::<f64>() < 0.8 {
                            0
                        } else {
                            rng.gen_range(1..unvisited_neighbors.len())
                        }
                    } else {
                        // Higher complexity: fully random
                        rng.gen_range(0..unvisited_neighbors.len())
                    };
                    let (nx, ny) = unvisited_neighbors[next_idx];

                    // Remove wall and mark as visited
                    maze.remove_wall(current_x, current_y, nx, ny);
                    visited.insert((nx, ny));
                    current_x = nx;
                    current_y = ny;
                }
            }

            // Hunt for an unvisited cell adjacent to visited cells
            let mut found = false;
            for y in 0..height {
                for x in 0..width {
                    if !visited.contains(&(x, y)) {
                        // Check if this cell has a visited neighbor
                        let neighbors = maze.get_neighbors(x, y);
                        let visited_neighbors: Vec<(u32, u32)> = neighbors
                            .into_iter()
                            .filter(|&(nx, ny)| visited.contains(&(nx, ny)))
                            .collect();

                        if !visited_neighbors.is_empty() {
                            // Choose a visited neighbor to connect to
                            let target_idx = if visited_neighbors.len() == 1 {
                                0
                            } else {
                                rng.gen_range(0..visited_neighbors.len())
                            };
                            let (tx, ty) = visited_neighbors[target_idx];

                            // Remove wall and mark as visited
                            maze.remove_wall(x, y, tx, ty);
                            visited.insert((x, y));
                            current_x = x;
                            current_y = y;
                            found = true;
                            break;
                        }
                    }
                }
                if found {
                    break;
                }
            }

            // If no unvisited cell found, we're done
            if !found {
                break;
            }
        }

        maze
    }
}

