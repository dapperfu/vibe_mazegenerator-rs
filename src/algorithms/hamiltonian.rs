use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub struct Hamiltonian;

impl MazeGenerator for Hamiltonian {
    fn generate(&self, width: u32, height: u32, complexity: f64, seed: Option<u64>) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => {
                let seed = rand::thread_rng().gen();
                ChaCha8Rng::seed_from_u64(seed)
            }
        };

        // Generate a Hamiltonian cycle/path using backtracking
        let mut path = Vec::new();
        let mut visited = vec![vec![false; width as usize]; height as usize];
        let total_cells = width * height;

        // Start at (0, 0)
        let start_x = 0;
        let start_y = 0;

        // Try to find a Hamiltonian path using backtracking
        fn find_hamiltonian_path(
            maze: &Maze,
            rng: &mut ChaCha8Rng,
            x: u32,
            y: u32,
            path: &mut Vec<(u32, u32)>,
            visited: &mut Vec<Vec<bool>>,
            width: u32,
            height: u32,
            complexity: f64,
        ) -> bool {
            path.push((x, y));
            visited[y as usize][x as usize] = true;

            if path.len() == (width * height) as usize {
                return true; // Found Hamiltonian path
            }

            // Get unvisited neighbors
            let mut neighbors: Vec<(u32, u32)> = maze
                .get_neighbors(x, y)
                .into_iter()
                .filter(|&(nx, ny)| !visited[ny as usize][nx as usize])
                .collect();

            // Shuffle neighbors based on complexity
            if complexity > 0.0 && neighbors.len() > 1 {
                for i in 0..neighbors.len() {
                    let j = rng.gen_range(i..neighbors.len());
                    neighbors.swap(i, j);
                }
            }

            // Try each neighbor
            for (nx, ny) in neighbors {
                if find_hamiltonian_path(maze, rng, nx, ny, path, visited, width, height, complexity)
                {
                    return true;
                }
            }

            // Backtrack
            path.pop();
            visited[y as usize][x as usize] = false;
            false
        }

        // Try to find Hamiltonian path (may fail for large mazes, so we'll have a fallback)
        let found = find_hamiltonian_path(
            &maze,
            &mut rng,
            start_x,
            start_y,
            &mut path,
            &mut visited,
            width,
            height,
            complexity,
        );

        if !found || path.len() < (total_cells * 3 / 4) as usize {
            // Fallback: create a spanning tree and extend it to cover most cells
            path.clear();
            visited = vec![vec![false; width as usize]; height as usize];
            let mut stack = vec![(start_x, start_y)];
            visited[start_y as usize][start_x as usize] = true;

            while let Some((x, y)) = stack.pop() {
                path.push((x, y));
                let mut neighbors: Vec<(u32, u32)> = maze
                    .get_neighbors(x, y)
                    .into_iter()
                    .filter(|&(nx, ny)| !visited[ny as usize][nx as usize])
                    .collect();

                if complexity > 0.0 && neighbors.len() > 1 {
                    for i in 0..neighbors.len() {
                        let j = rng.gen_range(i..neighbors.len());
                        neighbors.swap(i, j);
                    }
                }

                for (nx, ny) in neighbors {
                    visited[ny as usize][nx as usize] = true;
                    stack.push((nx, ny));
                }
            }
        }

        // Build maze from path: connect consecutive cells in path
        for i in 0..(path.len() - 1) {
            let (x1, y1) = path[i];
            let (x2, y2) = path[i + 1];
            maze.remove_wall(x1, y1, x2, y2);
        }

        // Optionally break some connections based on complexity
        // Higher complexity = break more connections (create more loops)
        if complexity > 0.3 {
            let break_probability = (complexity - 0.3) * 0.5; // Range: 0.0 to 0.35
            for i in 0..(path.len() - 1) {
                if rng.gen::<f64>() < break_probability {
                    // Break this connection by adding a wall
                    let (x1, y1) = path[i];
                    let (x2, y2) = path[i + 1];
                    // Get cells separately to avoid borrow checker issues
                    {
                        if let Some(mut_c1) = maze.get_cell_mut(x1, y1) {
                            if x2 == x1 + 1 {
                                mut_c1.east = true;
                            } else if x2 + 1 == x1 {
                                mut_c1.west = true;
                            } else if y2 == y1 + 1 {
                                mut_c1.south = true;
                            } else if y2 + 1 == y1 {
                                mut_c1.north = true;
                            }
                        }
                    }
                    {
                        if let Some(mut_c2) = maze.get_cell_mut(x2, y2) {
                            if x2 == x1 + 1 {
                                mut_c2.west = true;
                            } else if x2 + 1 == x1 {
                                mut_c2.east = true;
                            } else if y2 == y1 + 1 {
                                mut_c2.north = true;
                            } else if y2 + 1 == y1 {
                                mut_c2.south = true;
                            }
                        }
                    }
                }
            }
        }

        // Ensure connectivity from start to end (in case breaking connections disconnected them)
        maze.ensure_connectivity();

        maze
    }
}

