use crate::maze::Maze;

/// Trait for maze generation algorithms
pub trait MazeGenerator {
    /// Generate a maze with the given dimensions and complexity
    /// 
    /// # Arguments
    /// * `width` - Width of the maze in cells
    /// * `height` - Height of the maze in cells
    /// * `complexity` - Complexity parameter (0.0 to 1.0), affects algorithm behavior
    /// * `seed` - Optional seed for reproducible generation. If None, uses random seed.
    /// 
    /// # Returns
    /// A generated maze that is guaranteed to be solvable
    fn generate(&self, width: u32, height: u32, complexity: f64, seed: Option<u64>) -> Maze;
}

pub mod recursive_backtracking;
pub mod kruskal;
pub mod prim;
pub mod aldous_broder;
pub mod wilsons;
pub mod recursive_division;
pub mod growing_tree;
pub mod hunt_and_kill;
pub mod binary_tree;
pub mod sidewinder;
pub mod eller;
pub mod dfs_iterative;
pub mod bfs;
pub mod recursive_backtracking_braided;
pub mod cellular_automata;
pub mod drunkards_walk;
pub mod random_obstacle;
pub mod hamiltonian;
pub mod voronoi;
pub mod braid_postprocess;

pub use recursive_backtracking::RecursiveBacktracking;
pub use kruskal::Kruskal;
pub use prim::Prim;
pub use aldous_broder::AldousBroder;
pub use wilsons::Wilsons;
pub use recursive_division::RecursiveDivision;
pub use growing_tree::GrowingTree;
pub use hunt_and_kill::HuntAndKill;
pub use binary_tree::BinaryTree;
pub use sidewinder::Sidewinder;
pub use eller::Eller;
pub use dfs_iterative::DfsIterative;
pub use bfs::Bfs;
pub use recursive_backtracking_braided::RecursiveBacktrackingBraided;
pub use cellular_automata::CellularAutomata;
pub use drunkards_walk::DrunkardsWalk;
pub use random_obstacle::RandomObstacle;
pub use hamiltonian::Hamiltonian;
pub use voronoi::Voronoi;

