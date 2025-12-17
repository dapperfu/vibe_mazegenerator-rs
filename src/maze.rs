/// Represents a single cell in the maze with its walls
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
}

impl Cell {
    /// Create a new cell with all walls present
    pub fn new() -> Self {
        Cell {
            north: true,
            south: true,
            east: true,
            west: true,
        }
    }

    /// Check if a cell has all walls (completely isolated)
    #[allow(dead_code)]
    pub fn is_isolated(&self) -> bool {
        self.north && self.south && self.east && self.west
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new()
    }
}

/// Represents a complete maze with a grid of cells
#[derive(Clone, Debug)]
pub struct Maze {
    width: u32,
    height: u32,
    cells: Vec<Vec<Cell>>,
}

impl Maze {
    /// Create a new maze with all walls intact
    pub fn new(width: u32, height: u32) -> Self {
        let cells = (0..height)
            .map(|_| (0..width).map(|_| Cell::new()).collect())
            .collect();

        Maze {
            width,
            height,
            cells,
        }
    }

    /// Get the width of the maze
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the height of the maze
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get a reference to a cell at (x, y)
    pub fn get_cell(&self, x: u32, y: u32) -> Option<&Cell> {
        if x < self.width && y < self.height {
            Some(&self.cells[y as usize][x as usize])
        } else {
            None
        }
    }

    /// Get a mutable reference to a cell at (x, y)
    #[allow(dead_code)]
    pub fn get_cell_mut(&mut self, x: u32, y: u32) -> Option<&mut Cell> {
        if x < self.width && y < self.height {
            Some(&mut self.cells[y as usize][x as usize])
        } else {
            None
        }
    }

    /// Remove the wall between two adjacent cells
    pub fn remove_wall(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        if x1 >= self.width || y1 >= self.height || x2 >= self.width || y2 >= self.height {
            return;
        }

        let dx = x2 as i32 - x1 as i32;
        let dy = y2 as i32 - y1 as i32;

        // Check if cells are adjacent
        if (dx == 0 && dy.abs() == 1) || (dy == 0 && dx.abs() == 1) {
            let y1_idx = y1 as usize;
            let x1_idx = x1 as usize;
            let y2_idx = y2 as usize;
            let x2_idx = x2 as usize;

            if dx == 1 {
                // cell2 is east of cell1
                self.cells[y1_idx][x1_idx].east = false;
                self.cells[y2_idx][x2_idx].west = false;
            } else if dx == -1 {
                // cell2 is west of cell1
                self.cells[y1_idx][x1_idx].west = false;
                self.cells[y2_idx][x2_idx].east = false;
            } else if dy == 1 {
                // cell2 is south of cell1
                self.cells[y1_idx][x1_idx].south = false;
                self.cells[y2_idx][x2_idx].north = false;
            } else if dy == -1 {
                // cell2 is north of cell1
                self.cells[y1_idx][x1_idx].north = false;
                self.cells[y2_idx][x2_idx].south = false;
            }
        }
    }

    /// Check if there's a wall between two adjacent cells
    #[allow(dead_code)]
    pub fn has_wall(&self, x1: u32, y1: u32, x2: u32, y2: u32) -> bool {
        if x1 >= self.width || y1 >= self.height || x2 >= self.width || y2 >= self.height {
            return true;
        }

        let dx = x2 as i32 - x1 as i32;
        let dy = y2 as i32 - y1 as i32;

        if let Some(cell) = self.get_cell(x1, y1) {
            if dx == 1 {
                return cell.east;
            } else if dx == -1 {
                return cell.west;
            } else if dy == 1 {
                return cell.south;
            } else if dy == -1 {
                return cell.north;
            }
        }

        true
    }

    /// Get all valid neighbors of a cell
    pub fn get_neighbors(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let mut neighbors = Vec::new();

        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.height - 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
    }

    /// Get accessible neighbors (neighbors without walls between them)
    pub fn get_accessible_neighbors(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let mut accessible = Vec::new();

        if let Some(cell) = self.get_cell(x, y) {
            // Check west
            if x > 0 && !cell.west {
                accessible.push((x - 1, y));
            }
            // Check east
            if x < self.width - 1 && !cell.east {
                accessible.push((x + 1, y));
            }
            // Check north
            if y > 0 && !cell.north {
                accessible.push((x, y - 1));
            }
            // Check south
            if y < self.height - 1 && !cell.south {
                accessible.push((x, y + 1));
            }
        }

        accessible
    }

    /// Ensure there is a path from (0,0) to (width-1, height-1)
    /// This method carves a path if one doesn't already exist
    pub fn ensure_connectivity(&mut self) {
        use std::collections::{HashSet, VecDeque};

        let start = (0, 0);
        let end = (self.width - 1, self.height - 1);

        // First, check if there's already a path
        let mut queue = VecDeque::new();
        queue.push_back(start);
        let mut visited = HashSet::new();
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            if current == end {
                // Path already exists, nothing to do
                return;
            }

            for neighbor in self.get_accessible_neighbors(current.0, current.1) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        // No path exists, need to create one
        // Find the component containing start
        let mut start_component = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);
        start_component.insert(start);

        while let Some(current) = queue.pop_front() {
            for neighbor in self.get_accessible_neighbors(current.0, current.1) {
                if !start_component.contains(&neighbor) {
                    start_component.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        // Find the component containing end
        let mut end_component = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(end);
        end_component.insert(end);

        while let Some(current) = queue.pop_front() {
            for neighbor in self.get_accessible_neighbors(current.0, current.1) {
                if !end_component.contains(&neighbor) {
                    end_component.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        // Find the closest pair of cells between the two components
        let mut min_dist = u32::MAX;
        let mut best_pair = None;

        for &(x1, y1) in &start_component {
            for &(x2, y2) in &end_component {
                let dist = (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs();
                if dist < min_dist as i32 {
                    min_dist = dist as u32;
                    best_pair = Some((x1, y1, x2, y2));
                }
            }
        }

        // Carve a path between the two components
        let (x1, y1, x2, y2) = best_pair.unwrap_or((start.0, start.1, end.0, end.1));
        let mut cx = x1;
        let mut cy = y1;

        // Move horizontally first, then vertically
        while cx != x2 {
            let next_x = if cx < x2 { cx + 1 } else { cx - 1 };
            self.remove_wall(cx, cy, next_x, cy);
            cx = next_x;
        }

        while cy != y2 {
            let next_y = if cy < y2 { cy + 1 } else { cy - 1 };
            self.remove_wall(cx, cy, cx, next_y);
            cy = next_y;
        }
    }

    /// Solve the maze using BFS, returning the path from (0,0) to (width-1, height-1)
    pub fn solve(&self) -> Option<Vec<(u32, u32)>> {
        use std::collections::{HashMap, VecDeque};

        let start = (0, 0);
        let end = (self.width - 1, self.height - 1);

        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut visited = HashMap::new();
        visited.insert(start, None); // None means this is the start

        while let Some(current) = queue.pop_front() {
            if current == end {
                // Reconstruct path
                let mut path = Vec::new();
                let mut node = Some(end);
                while let Some(pos) = node {
                    path.push(pos);
                    node = visited[&pos];
                }
                path.reverse();
                return Some(path);
            }

            for neighbor in self.get_accessible_neighbors(current.0, current.1) {
                if !visited.contains_key(&neighbor) {
                    visited.insert(neighbor, Some(current));
                    queue.push_back(neighbor);
                }
            }
        }

        None // No path found
    }
}

