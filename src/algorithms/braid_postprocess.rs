use crate::maze::Maze;
use rand::Rng;

/// Post-processing utility to remove dead ends from a maze (braiding)
/// 
/// This function identifies dead ends (cells with exactly 3 walls) and removes
/// one of the walls to eliminate the dead end, creating loops in the maze.
/// 
/// # Arguments
/// * `maze` - The maze to braid (modified in place)
/// * `complexity` - Controls how many dead ends to remove (0.0 = none, 1.0 = all)
pub fn braid_maze(maze: &mut Maze, complexity: f64) {
    let width = maze.width();
    let height = maze.height();
    let mut rng = rand::thread_rng();

    // Collect all dead ends
    let mut dead_ends = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if let Some(cell) = maze.get_cell(x, y) {
                let mut wall_count = 0;
                if cell.north {
                    wall_count += 1;
                }
                if cell.south {
                    wall_count += 1;
                }
                if cell.east {
                    wall_count += 1;
                }
                if cell.west {
                    wall_count += 1;
                }

                // Dead end: exactly 3 walls
                if wall_count == 3 {
                    dead_ends.push((x, y));
                }
            }
        }
    }

    // Shuffle dead ends for randomness
    for i in 0..dead_ends.len() {
        let j = rng.gen_range(i..dead_ends.len());
        dead_ends.swap(i, j);
    }

    // Remove dead ends based on complexity
    // Higher complexity = remove more dead ends
    let remove_count = (dead_ends.len() as f64 * complexity) as usize;

    for i in 0..remove_count.min(dead_ends.len()) {
        let (x, y) = dead_ends[i];
        if let Some(cell) = maze.get_cell(x, y) {
            // Find which wall to remove
            let mut candidates = Vec::new();

            // Check each direction
            if cell.north && y > 0 {
                candidates.push((x, y - 1));
            }
            if cell.south && y < height - 1 {
                candidates.push((x, y + 1));
            }
            if cell.east && x < width - 1 {
                candidates.push((x + 1, y));
            }
            if cell.west && x > 0 {
                candidates.push((x - 1, y));
            }

            // Remove wall to a random candidate
            if !candidates.is_empty() {
                let target = candidates[rng.gen_range(0..candidates.len())];
                maze.remove_wall(x, y, target.0, target.1);
            }
        }
    }
}

