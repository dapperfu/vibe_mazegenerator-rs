mod algorithms;
mod config;
mod maze;
mod render;

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

    /// Algorithm to use: recursive_backtracking, kruskal, prim, aldous_broder
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
            match save_maze_with_solution(&maze, config.cell_size, &solution, &solved_path) {
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

