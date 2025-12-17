use crate::algorithms::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

pub struct Voronoi;

impl MazeGenerator for Voronoi {
    fn generate(&self, width: u32, height: u32, complexity: f64, seed: Option<u64>) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => {
                let seed = rand::thread_rng().gen();
                ChaCha8Rng::seed_from_u64(seed)
            }
        };

        // Number of Voronoi points based on complexity
        // Lower complexity = fewer points (larger regions)
        // Higher complexity = more points (smaller regions)
        let num_points = ((5.0 + complexity * 15.0) as usize).min((width * height / 4) as usize);
        let num_points = num_points.max(2);

        // Generate random points
        let mut points = Vec::new();
        let mut point_set = HashSet::new();
        for _ in 0..num_points {
            let mut x = rng.gen_range(0..width);
            let mut y = rng.gen_range(0..height);
            let mut attempts = 0;
            while point_set.contains(&(x, y)) && attempts < 100 {
                x = rng.gen_range(0..width);
                y = rng.gen_range(0..height);
                attempts += 1;
            }
            points.push((x, y));
            point_set.insert((x, y));
        }

        // Assign each cell to nearest point (Voronoi diagram)
        let mut cell_to_point: Vec<Vec<usize>> = (0..height)
            .map(|_| vec![0; width as usize])
            .collect();

        for y in 0..height {
            for x in 0..width {
                let mut min_dist = f64::MAX;
                let mut nearest = 0;
                for (i, &(px, py)) in points.iter().enumerate() {
                    let dx = x as f64 - px as f64;
                    let dy = y as f64 - py as f64;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < min_dist {
                        min_dist = dist;
                        nearest = i;
                    }
                }
                cell_to_point[y as usize][x as usize] = nearest;
            }
        }

        // Build graph: edges between adjacent Voronoi regions
        let mut edges = HashSet::new();
        for y in 0..height {
            for x in 0..width {
                let region1 = cell_to_point[y as usize][x as usize];
                
                // Check east neighbor
                if x < width - 1 {
                    let region2 = cell_to_point[y as usize][(x + 1) as usize];
                    if region1 != region2 {
                        let edge = if region1 < region2 {
                            (region1, region2)
                        } else {
                            (region2, region1)
                        };
                        edges.insert(edge);
                    }
                }
                
                // Check south neighbor
                if y < height - 1 {
                    let region2 = cell_to_point[(y + 1) as usize][x as usize];
                    if region1 != region2 {
                        let edge = if region1 < region2 {
                            (region1, region2)
                        } else {
                            (region2, region1)
                        };
                        edges.insert(edge);
                    }
                }
            }
        }

        // Convert edges to list and shuffle based on complexity
        let mut edge_list: Vec<(usize, usize)> = edges.into_iter().collect();
        if complexity > 0.0 {
            for i in 0..edge_list.len() {
                let j = rng.gen_range(i..edge_list.len());
                edge_list.swap(i, j);
            }
        }

        // Union-Find for spanning tree
        struct UnionFind {
            parent: Vec<usize>,
        }

        impl UnionFind {
            fn new(size: usize) -> Self {
                UnionFind {
                    parent: (0..size).collect(),
                }
            }

            fn find(&mut self, x: usize) -> usize {
                if self.parent[x] != x {
                    self.parent[x] = self.find(self.parent[x]);
                }
                self.parent[x]
            }

            fn union(&mut self, x: usize, y: usize) -> bool {
                let root_x = self.find(x);
                let root_y = self.find(y);
                if root_x != root_y {
                    self.parent[root_y] = root_x;
                    true
                } else {
                    false
                }
            }
        }

        let mut uf = UnionFind::new(num_points);
        let mut selected_edges = Vec::new();

        // Build spanning tree (or near-tree based on complexity)
        for (r1, r2) in edge_list {
            if uf.find(r1) != uf.find(r2) {
                uf.union(r1, r2);
                selected_edges.push((r1, r2));
            } else if complexity > 0.5 {
                // Add extra edges for loops (higher complexity)
                let extra_prob = (complexity - 0.5) * 2.0; // Range: 0.0 to 1.0
                if rng.gen::<f64>() < extra_prob {
                    selected_edges.push((r1, r2));
                }
            }
        }

        // Map edges to grid cells and carve passages
        for (r1, r2) in selected_edges {
            // Find boundary cells between regions
            let mut boundary_cells = Vec::new();
            for y in 0..height {
                for x in 0..width {
                    let region = cell_to_point[y as usize][x as usize];
                    if region == r1 {
                        // Check if neighbor is in r2
                        if x < width - 1
                            && cell_to_point[y as usize][(x + 1) as usize] == r2
                        {
                            boundary_cells.push((x, y, x + 1, y));
                        }
                        if y < height - 1
                            && cell_to_point[(y + 1) as usize][x as usize] == r2
                        {
                            boundary_cells.push((x, y, x, y + 1));
                        }
                    } else if region == r2 {
                        // Check if neighbor is in r1
                        if x < width - 1
                            && cell_to_point[y as usize][(x + 1) as usize] == r1
                        {
                            boundary_cells.push((x, y, x + 1, y));
                        }
                        if y < height - 1
                            && cell_to_point[(y + 1) as usize][x as usize] == r1
                        {
                            boundary_cells.push((x, y, x, y + 1));
                        }
                    }
                }
            }

            // Carve passage through boundary (select one or more based on complexity)
            if !boundary_cells.is_empty() {
                let num_passages = if complexity < 0.3 {
                    1 // Single passage
                } else {
                    // Multiple passages for higher complexity
                    let max_passages = boundary_cells.len().min(3);
                    (1.0 + complexity * (max_passages - 1) as f64) as usize
                };

                // Shuffle boundary cells
                if boundary_cells.len() > 1 {
                    for i in 0..boundary_cells.len() {
                        let j = rng.gen_range(i..boundary_cells.len());
                        boundary_cells.swap(i, j);
                    }
                }

                // Carve selected passages
                for i in 0..num_passages.min(boundary_cells.len()) {
                    let (x1, y1, x2, y2) = boundary_cells[i];
                    maze.remove_wall(x1, y1, x2, y2);
                }
            }
        }

        // Ensure connectivity from start to end
        maze.ensure_connectivity();

        maze
    }
}

