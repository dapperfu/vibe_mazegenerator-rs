use crate::algorithms::MazeGenerator;
use crate::maze::Maze;

pub struct RecursiveBacktrackingBraided;

impl MazeGenerator for RecursiveBacktrackingBraided {
    fn generate(&self, width: u32, height: u32, complexity: f64, seed: Option<u64>) -> Maze {
        // First generate using recursive backtracking
        let generator = crate::algorithms::RecursiveBacktracking;
        let mut maze = generator.generate(width, height, complexity, seed);

        // Then apply braiding (remove dead ends)
        crate::algorithms::braid_postprocess::braid_maze(&mut maze, complexity);

        maze
    }
}

