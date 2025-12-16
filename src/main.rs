pub mod algorithms;
mod config;
pub mod maze;
pub mod render;

use clap::Parser;
use config::{Algorithm, Config};
use render::{save_maze, save_maze_with_solution};

#[derive(Parser, Debug)]
#[command(name = "maze_generator")]
#[command(about = "Generate mazes using various algorithms", long_about = None)]
struct Args {
    /// Width of the maze in cells
    #[arg(long)]
    width: Option<u32>,

    /// Height of the maze in cells
    #[arg(long)]
    height: Option<u32>,

    /// Algorithm to use: recursive_backtracking, kruskal, prim, aldous_broder, wilsons, recursive_division, growing_tree, hunt_and_kill, binary_tree, sidewinder, eller, dfs_iterative, bfs, recursive_backtracking_braided, cellular_automata, drunkards_walk, random_obstacle, hamiltonian, voronoi
    #[arg(long)]
    algorithm: Option<String>,

    /// Complexity parameter (0.0 to 1.0)
    #[arg(long)]
    complexity: Option<f64>,

    /// Output file path
    #[arg(long)]
    output: Option<String>,

    /// Configuration file path
    #[arg(long, default_value = "config.toml")]
    config: Option<String>,

    /// Seed for reproducible maze generation
    #[arg(long)]
    seed: Option<u64>,

    /// Solution line color (hex code, e.g., "#ff0000" or "ff0000")
    #[arg(long)]
    line_color: Option<String>,

    /// Solution line thickness (overrides auto-calculation)
    #[arg(long)]
    line_thickness: Option<f32>,
}

fn main() {
    let args = Args::parse();

    // Load configuration
    let config = Config::load(args.config.as_deref()).with_cli_overrides(
        args.width,
        args.height,
        args.algorithm.as_deref(),
        args.complexity,
        args.output.as_deref(),
        args.seed,
        args.line_color.as_deref(),
        args.line_thickness,
    );

    println!("Generating maze with:");
    println!("  Width: {}", config.width);
    println!("  Height: {}", config.height);
    println!("  Algorithm: {}", config.algorithm.to_string());
    println!("  Complexity: {:.2}", config.complexity);
    if let Some(seed) = config.seed {
        println!("  Seed: {}", seed);
    } else {
        println!("  Seed: random");
    }
    println!("  Output: {}", config.output);

    // Select algorithm
    let generator: Box<dyn algorithms::MazeGenerator> = match config.algorithm {
        Algorithm::RecursiveBacktracking => {
            Box::new(algorithms::RecursiveBacktracking)
        }
        Algorithm::Kruskal => Box::new(algorithms::Kruskal),
        Algorithm::Prim => Box::new(algorithms::Prim),
        Algorithm::AldousBroder => Box::new(algorithms::AldousBroder),
        Algorithm::Wilsons => Box::new(algorithms::Wilsons),
        Algorithm::RecursiveDivision => Box::new(algorithms::RecursiveDivision),
        Algorithm::GrowingTree => Box::new(algorithms::GrowingTree),
        Algorithm::HuntAndKill => Box::new(algorithms::HuntAndKill),
        Algorithm::BinaryTree => Box::new(algorithms::BinaryTree),
        Algorithm::Sidewinder => Box::new(algorithms::Sidewinder),
        Algorithm::Eller => Box::new(algorithms::Eller),
        Algorithm::DfsIterative => Box::new(algorithms::DfsIterative),
        Algorithm::Bfs => Box::new(algorithms::Bfs),
        Algorithm::RecursiveBacktrackingBraided => Box::new(algorithms::RecursiveBacktrackingBraided),
        Algorithm::CellularAutomata => Box::new(algorithms::CellularAutomata),
        Algorithm::DrunkardsWalk => Box::new(algorithms::DrunkardsWalk),
        Algorithm::RandomObstacle => Box::new(algorithms::RandomObstacle),
        Algorithm::Hamiltonian => Box::new(algorithms::Hamiltonian),
        Algorithm::Voronoi => Box::new(algorithms::Voronoi),
    };

    // Generate maze
    println!("Generating maze...");
    let maze = generator.generate(config.width, config.height, config.complexity, config.seed);

    // Save to PNG
    println!("Rendering to PNG...");
    match save_maze(&maze, config.cell_size, &config.output) {
        Ok(()) => {
            println!("Maze saved to {}", config.output);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    // Always solve maze and save solution
    println!("Solving maze...");
    match maze.solve() {
        Some(solution) => {
            println!("Solution found with {} steps", solution.len());
            // Generate solved filename based on output filename
            let solved_path = if config.output.ends_with(".png") {
                config.output.replace(".png", "_solved.png")
            } else {
                format!("{}_solved.png", config.output)
            };
            match save_maze_with_solution(
                &maze,
                config.cell_size,
                &solution,
                &solved_path,
                &config.solution_line_color,
                config.solution_line_thickness,
            ) {
                Ok(()) => {
                    println!("Solved maze saved to {}", solved_path);
                }
                Err(e) => {
                    eprintln!("Error saving solved maze: {}", e);
                    std::process::exit(1);
                }
            }
        }
        None => {
            eprintln!("Error: Could not solve maze (no path found)");
            std::process::exit(1);
        }
    }
}

