use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::{HashSet, VecDeque};

pub struct Bfs;

impl MazeGenerator for Bfs {
    fn generate(&self, width: u32, height: u32, complexity: f64, seed: Option<u64>) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => {
                let seed = rand::thread_rng().gen();
                ChaCha8Rng::seed_from_u64(seed)
            }
        };
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        // Start at a random cell
        let start_x = rng.gen_range(0..width);
        let start_y = rng.gen_range(0..height);
        queue.push_back((start_x, start_y));
        visited.insert((start_x, start_y));

        while let Some((x, y)) = queue.pop_front() {
            // Get unvisited neighbors
            let neighbors: Vec<(u32, u32)> = maze
                .get_neighbors(x, y)
                .into_iter()
                .filter(|&(nx, ny)| !visited.contains(&(nx, ny)))
                .collect();

            if !neighbors.is_empty() {
                // Select neighbors to process based on complexity
                // Lower complexity = process all neighbors (more bushy)
                // Higher complexity = process fewer neighbors randomly (less bushy)
                let num_to_process = if complexity < 0.1 {
                    neighbors.len() // Process all neighbors
                } else {
                    // Process 1 to all neighbors, biased by complexity
                    let min_process = 1;
                    let max_process = neighbors.len();
                    let process_probability = 1.0 - (complexity * 0.5); // At complexity 1.0, process ~50% on average
                    let mut count = 0;
                    for _ in &neighbors {
                        if rng.gen::<f64>() < process_probability {
                            count += 1;
                        }
                    }
                    count.max(min_process).min(max_process)
                };

                // Shuffle neighbors if complexity > 0
                let mut neighbors_to_process = neighbors;
                if complexity > 0.0 {
                    for i in 0..neighbors_to_process.len() {
                        let j = rng.gen_range(i..neighbors_to_process.len());
                        neighbors_to_process.swap(i, j);
                    }
                }

                // Process selected neighbors
                for i in 0..num_to_process.min(neighbors_to_process.len()) {
                    let (nx, ny) = neighbors_to_process[i];
                    
                    // Remove wall between current and neighbor
                    maze.remove_wall(x, y, nx, ny);
                    
                    // Mark as visited and add to queue
                    visited.insert((nx, ny));
                    queue.push_back((nx, ny));
                }
            }
        }

        maze
    }
}

