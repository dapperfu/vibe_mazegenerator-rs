use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::VecDeque;

pub struct CellularAutomata;

impl MazeGenerator for CellularAutomata {
    fn generate(&self, width: u32, height: u32, complexity: f64, seed: Option<u64>) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => {
                let seed = rand::thread_rng().gen();
                ChaCha8Rng::seed_from_u64(seed)
            }
        };

        // Initial fill density based on complexity
        // Lower complexity = more open space initially
        // Higher complexity = more walls initially
        let initial_density = 0.3 + complexity * 0.4; // Range: 0.3 to 0.7

        // Random fill: mark cells as walls (true) or open (false)
        let mut cells: Vec<Vec<bool>> = (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| rng.gen::<f64>() < initial_density)
                    .collect()
            })
            .collect();

        // Number of CA iterations based on complexity
        let iterations = (3.0 + complexity * 5.0) as usize; // Range: 3 to 8

        // Apply CA smoothing rules
        for _ in 0..iterations {
            let mut new_cells = cells.clone();
            for y in 0..height {
                for x in 0..width {
                    // Count wall neighbors (including diagonals)
                    let mut wall_count = 0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            if nx >= 0
                                && nx < width as i32
                                && ny >= 0
                                && ny < height as i32
                            {
                                if cells[ny as usize][nx as usize] {
                                    wall_count += 1;
                                }
                            } else {
                                // Count edges as walls
                                wall_count += 1;
                            }
                        }
                    }

                    // CA rule: if 5 or more neighbors are walls, become wall; otherwise become open
                    new_cells[y as usize][x as usize] = wall_count >= 5;
                }
            }
            cells = new_cells;
        }

        // Convert CA cells to maze walls
        // If cell is wall (true), add walls around it; if open (false), remove walls
        for y in 0..height {
            for x in 0..width {
                let is_wall = cells[y as usize][x as usize];
                if !is_wall {
                    // Open cell: remove walls to neighbors if they're also open
                    if x < width - 1 && !cells[y as usize][(x + 1) as usize] {
                        maze.remove_wall(x, y, x + 1, y);
                    }
                    if y < height - 1 && !cells[(y + 1) as usize][x as usize] {
                        maze.remove_wall(x, y, x, y + 1);
                    }
                }
            }
        }

        // Ensure connectivity using flood fill
        // Find all connected components and connect them
        let mut visited = vec![vec![false; width as usize]; height as usize];
        let mut components = Vec::new();

        for y in 0..height {
            for x in 0..width {
                if !cells[y as usize][x as usize] && !visited[y as usize][x as usize] {
                    // Found a new component
                    let mut component = Vec::new();
                    let mut queue = VecDeque::new();
                    queue.push_back((x, y));
                    visited[y as usize][x as usize] = true;

                    while let Some((cx, cy)) = queue.pop_front() {
                        component.push((cx, cy));
                        for neighbor in maze.get_accessible_neighbors(cx, cy) {
                            let (nx, ny) = neighbor;
                            if !cells[ny as usize][nx as usize]
                                && !visited[ny as usize][nx as usize]
                            {
                                visited[ny as usize][nx as usize] = true;
                                queue.push_back((nx, ny));
                            }
                        }
                    }
                    components.push(component);
                }
            }
        }

        // Connect components by carving paths
        if components.len() > 1 {
            for i in 0..(components.len() - 1) {
                let comp1 = &components[i];
                let comp2 = &components[i + 1];
                
                // Find closest pair of cells between components
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

                // Carve path between components
                if let Some((x1, y1, x2, y2)) = best_pair {
                    // Simple path: move horizontally then vertically
                    let mut cx = x1;
                    let mut cy = y1;
                    while cx != x2 {
                        let next_x = if cx < x2 { cx + 1 } else { cx - 1 };
                        maze.remove_wall(cx, cy, next_x, cy);
                        cx = next_x;
                    }
                    while cy != y2 {
                        let next_y = if cy < y2 { cy + 1 } else { cy - 1 };
                        maze.remove_wall(cx, cy, cx, next_y);
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

