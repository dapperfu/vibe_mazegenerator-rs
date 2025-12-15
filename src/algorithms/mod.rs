use crate::maze::Maze;

/// Trait for maze generation algorithms
pub trait MazeGenerator {
    /// Generate a maze with the given dimensions and complexity
    /// 
    /// # Arguments
    /// * `width` - Width of the maze in cells
    /// * `height` - Height of the maze in cells
    /// * `complexity` - Complexity parameter (0.0 to 1.0), affects algorithm behavior
    /// 
    /// # Returns
    /// A generated maze that is guaranteed to be solvable
    fn generate(&self, width: u32, height: u32, complexity: f64) -> Maze;
}

pub mod recursive_backtracking;
pub mod kruskal;
pub mod prim;
pub mod aldous_broder;

pub use recursive_backtracking::RecursiveBacktracking;
pub use kruskal::Kruskal;
pub use prim::Prim;
pub use aldous_broder::AldousBroder;

