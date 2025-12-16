use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use maze_generator::algorithms::{MazeGenerator, RecursiveBacktracking, Kruskal, Prim, AldousBroder};
use maze_generator::render::render_maze;

fn benchmark_algorithm(c: &mut Criterion, name: &str, generator: Box<dyn MazeGenerator>) {
    let mut group = c.benchmark_group(name);
    
    // Benchmark different sizes
    for size in [10, 20, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                b.iter(|| {
                    let maze = generator.generate(
                        black_box(size),
                        black_box(size),
                        black_box(0.5),
                        black_box(Some(42)), // Fixed seed for consistency
                    );
                    black_box(maze)
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_all_algorithms(c: &mut Criterion) {
    benchmark_algorithm(c, "recursive_backtracking", Box::new(RecursiveBacktracking));
    benchmark_algorithm(c, "kruskal", Box::new(Kruskal));
    benchmark_algorithm(c, "prim", Box::new(Prim));
    benchmark_algorithm(c, "aldous_broder", Box::new(AldousBroder));
}

fn benchmark_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("complexity_variations");
    let generator = RecursiveBacktracking;
    let size = 50;
    
    for complexity in [0.0, 0.25, 0.5, 0.75, 1.0].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("complexity_{}", complexity)),
            complexity,
            |b, &complexity| {
                b.iter(|| {
                    let maze = generator.generate(
                        black_box(size),
                        black_box(size),
                        black_box(complexity),
                        black_box(Some(42)),
                    );
                    black_box(maze)
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_solving(c: &mut Criterion) {
    let mut group = c.benchmark_group("maze_solving");
    let generator = RecursiveBacktracking;
    
    for size in [10, 20, 50, 100].iter() {
        // Generate maze once
        let maze = generator.generate(*size, *size, 0.5, Some(42));
        
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, _| {
                b.iter(|| {
                    let solution = black_box(&maze).solve();
                    black_box(solution)
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("rendering");
    let generator = RecursiveBacktracking;
    
    for size in [10, 20, 50, 100].iter() {
        let maze = generator.generate(*size, *size, 0.5, Some(42));
        let cell_size = 10;
        
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, _| {
                b.iter(|| {
                    let img = render_maze(black_box(&maze), black_box(cell_size));
                    black_box(img)
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_all_algorithms,
    benchmark_complexity,
    benchmark_solving,
    benchmark_rendering
);
criterion_main!(benches);

