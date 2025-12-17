use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::{HashSet, VecDeque};

pub struct Wilsons;

impl MazeGenerator for Wilsons {
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

        // Start with one random cell in the tree
        let start_x = rng.gen_range(0..width);
        let start_y = rng.gen_range(0..height);
        in_tree.insert((start_x, start_y));

        // List of unvisited cells
        let mut unvisited: Vec<(u32, u32)> = (0..height)
            .flat_map(|y| (0..width).map(move |x| (x, y)))
            .filter(|&(x, y)| !in_tree.contains(&(x, y)))
            .collect();

        // Shuffle unvisited cells for randomness
        if complexity > 0.0 {
            for i in 0..unvisited.len() {
                let j = rng.gen_range(i..unvisited.len());
                unvisited.swap(i, j);
            }
        }

        // For each unvisited cell, perform loop-erased random walk
        while !unvisited.is_empty() {
            // Pick an unvisited cell to start the walk
            let start_cell = unvisited[0];
            let mut current = start_cell;
            let mut path = VecDeque::new();
            path.push_back(current);
            let mut visited_in_walk = HashSet::new();
            visited_in_walk.insert(current);

            // Random walk until we hit the tree
            while !in_tree.contains(&current) {
                let neighbors = maze.get_neighbors(current.0, current.1);
                
                // Choose neighbor based on complexity
                let next_idx = if neighbors.is_empty() {
                    break;
                } else if neighbors.len() == 1 {
                    0
                } else if complexity < 0.1 {
                    // Very low complexity: prefer first neighbor
                    if rng.gen::<f64>() < 0.8 {
                        0
                    } else {
                        rng.gen_range(1..neighbors.len())
                    }
                } else {
                    // Higher complexity: fully random
                    rng.gen_range(0..neighbors.len())
                };
                
                let next = neighbors[next_idx];

                // If we've visited this cell in the current walk, erase the loop
                if visited_in_walk.contains(&next) {
                    // Remove all cells after the first occurrence of 'next' in the path
                    while let Some(back) = path.back() {
                        if *back == next {
                            break;
                        }
                        visited_in_walk.remove(back);
                        path.pop_back();
                    }
                    current = next;
                } else {
                    path.push_back(next);
                    visited_in_walk.insert(next);
                    current = next;
                }
            }

            // Add the path to the tree
            let mut prev = path.pop_front().unwrap();
            while let Some(current) = path.pop_front() {
                maze.remove_wall(prev.0, prev.1, current.0, current.1);
                in_tree.insert(prev);
                prev = current;
            }
            in_tree.insert(prev);

            // Remove visited cells from unvisited list
            unvisited.retain(|&cell| !in_tree.contains(&cell));
        }

        maze
    }
}

