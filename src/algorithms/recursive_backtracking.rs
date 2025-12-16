use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub struct RecursiveBacktracking;

impl MazeGenerator for RecursiveBacktracking {
    fn generate(&self, width: u32, height: u32, complexity: f64, seed: Option<u64>) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => {
                let seed = rand::thread_rng().gen();
                ChaCha8Rng::seed_from_u64(seed)
            }
        };
        let mut stack = Vec::new();
        let mut visited = vec![vec![false; width as usize]; height as usize];

        // Start at a random cell
        let start_x = rng.gen_range(0..width);
        let start_y = rng.gen_range(0..height);
        stack.push((start_x, start_y));
        visited[start_y as usize][start_x as usize] = true;

        while let Some((x, y)) = stack.pop() {
            // Get unvisited neighbors
            let neighbors: Vec<(u32, u32)> = maze
                .get_neighbors(x, y)
                .into_iter()
                .filter(|&(nx, ny)| !visited[ny as usize][nx as usize])
                .collect();

            if !neighbors.is_empty() {
                // Push current cell back onto stack
                stack.push((x, y));

                // Choose a random unvisited neighbor
                let next_idx = rng.gen_range(0..neighbors.len());
                let (nx, ny) = neighbors[next_idx];

                // Remove wall between current and chosen neighbor
                maze.remove_wall(x, y, nx, ny);

                // Mark neighbor as visited and push onto stack
                visited[ny as usize][nx as usize] = true;
                stack.push((nx, ny));
            } else {
                // No unvisited neighbors - backtrack based on complexity
                // Higher complexity means more backtracking (more branches)
                if complexity > 0.0 && rng.gen::<f64>() < complexity * 0.3 {
                    // Occasionally backtrack to create more branches
                    if let Some(&(bx, by)) = stack.last() {
                        if let Some(backtrack_neighbor) = maze
                            .get_neighbors(bx, by)
                            .into_iter()
                            .find(|&(nx, ny)| !visited[ny as usize][nx as usize])
                        {
                            let (nx, ny) = backtrack_neighbor;
                            maze.remove_wall(bx, by, nx, ny);
                            visited[ny as usize][nx as usize] = true;
                            stack.push((nx, ny));
                        }
                    }
                }
            }
        }

        maze
    }
}

