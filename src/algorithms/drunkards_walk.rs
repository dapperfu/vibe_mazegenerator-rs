use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

pub struct DrunkardsWalk;

impl MazeGenerator for DrunkardsWalk {
    fn generate(&self, width: u32, height: u32, complexity: f64, seed: Option<u64>) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => {
                let seed = rand::thread_rng().gen();
                ChaCha8Rng::seed_from_u64(seed)
            }
        };
        let mut carved = HashSet::new();
        let total_cells = width * height;

        // Density threshold based on complexity
        // Lower complexity = lower density (more open space needed)
        // Higher complexity = higher density (less open space needed)
        let density_threshold = 0.4 + complexity * 0.3; // Range: 0.4 to 0.7
        let target_carved = (total_cells as f64 * density_threshold) as u32;

        // Start at a random cell
        let mut current_x = rng.gen_range(0..width);
        let mut current_y = rng.gen_range(0..height);
        carved.insert((current_x, current_y));

        // Random walk until enough area is carved
        let mut steps = 0;
        let max_steps = total_cells * 10; // Prevent infinite loops

        while (carved.len() as u32) < target_carved && steps < max_steps {
            let neighbors = maze.get_neighbors(current_x, current_y);
            if neighbors.is_empty() {
                break;
            }

            // Choose neighbor based on complexity
            let next_idx = if neighbors.len() == 1 {
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
            let (nx, ny) = neighbors[next_idx];

            // Remove wall and mark as carved
            maze.remove_wall(current_x, current_y, nx, ny);
            carved.insert((nx, ny));
            current_x = nx;
            current_y = ny;
            steps += 1;
        }

        // Ensure connectivity: connect any isolated carved regions
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for &(x, y) in &carved {
            if !visited.contains(&(x, y)) {
                let mut component = Vec::new();
                let mut stack = vec![(x, y)];
                visited.insert((x, y));

                while let Some((cx, cy)) = stack.pop() {
                    component.push((cx, cy));
                    for neighbor in maze.get_accessible_neighbors(cx, cy) {
                        if carved.contains(&neighbor) && !visited.contains(&neighbor) {
                            visited.insert(neighbor);
                            stack.push(neighbor);
                        }
                    }
                }
                components.push(component);
            }
        }

        // Connect components
        if components.len() > 1 {
            for i in 0..(components.len() - 1) {
                let comp1 = &components[i];
                let comp2 = &components[i + 1];
                
                // Find closest pair
                let mut min_dist = u32::MAX;
                let mut best_pair = None;
                for &(x1, y1) in comp1 {
                    for &(x2, y2) in comp2 {
                        let dist = (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs();
                        if dist < min_dist as i32 {
                            min_dist = dist as u32;
                            best_pair = Some((x1, y1, x2, y2));
                        }
                    }
                }

                // Carve path
                if let Some((x1, y1, x2, y2)) = best_pair {
                    let mut cx = x1;
                    let mut cy = y1;
                    while cx != x2 {
                        let next_x = if cx < x2 { cx + 1 } else { cx - 1 };
                        maze.remove_wall(cx, cy, next_x, cy);
                        carved.insert((cx, cy));
                        carved.insert((next_x, cy));
                        cx = next_x;
                    }
                    while cy != y2 {
                        let next_y = if cy < y2 { cy + 1 } else { cy - 1 };
                        maze.remove_wall(cx, cy, cx, next_y);
                        carved.insert((cx, cy));
                        carved.insert((cx, next_y));
                        cy = next_y;
                    }
                }
            }
        }

        // Ensure connectivity from start to end
        maze.ensure_connectivity();

        maze
    }
}

