use crate::maze::Maze;
use image::{ImageBuffer, Rgb, RgbImage};

/// Render a maze to a PNG image
pub fn render_maze(maze: &Maze, cell_size: u32) -> Result<RgbImage, String> {
    let img_width = maze.width() * cell_size + 1;
    let img_height = maze.height() * cell_size + 1;

    let mut img: RgbImage = ImageBuffer::new(img_width, img_height);

    // Fill with white background
    for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    // Draw walls
    for y in 0..maze.height() {
        for x in 0..maze.width() {
            if let Some(cell) = maze.get_cell(x, y) {
                let px = x * cell_size;
                let py = y * cell_size;

                // Draw north wall
                if cell.north {
                    for i in 0..=cell_size {
                        let px_pos = px + i;
                        if px_pos < img_width && py < img_height {
                            *img.get_pixel_mut(px_pos, py) = Rgb([0, 0, 0]);
                        }
                    }
                }

                // Draw south wall
                if cell.south {
                    let sy = py + cell_size;
                    if sy < img_height {
                        for i in 0..=cell_size {
                            let px_pos = px + i;
                            if px_pos < img_width {
                                *img.get_pixel_mut(px_pos, sy) = Rgb([0, 0, 0]);
                            }
                        }
                    }
                }

                // Draw west wall
                if cell.west {
                    for i in 0..=cell_size {
                        let py_pos = py + i;
                        if py_pos < img_height && px < img_width {
                            *img.get_pixel_mut(px, py_pos) = Rgb([0, 0, 0]);
                        }
                    }
                }

                // Draw east wall
                if cell.east {
                    let sx = px + cell_size;
                    if sx < img_width {
                        for i in 0..=cell_size {
                            let py_pos = py + i;
                            if py_pos < img_height {
                                *img.get_pixel_mut(sx, py_pos) = Rgb([0, 0, 0]);
                            }
                        }
                    }
                }
            }
        }
    }

    // Mark entry (top-left) - remove north and west walls
    if maze.get_cell(0, 0).is_some() {
        // Remove entry walls visually (make opening)
        for i in 1..cell_size {
            if i < img_width {
                *img.get_pixel_mut(i, 0) = Rgb([255, 255, 255]);
            }
            if i < img_height {
                *img.get_pixel_mut(0, i) = Rgb([255, 255, 255]);
            }
        }
    }

    // Mark exit (bottom-right) - remove south and east walls
    let exit_x = maze.width() - 1;
    let exit_y = maze.height() - 1;
    if maze.get_cell(exit_x, exit_y).is_some() {
        let px = exit_x * cell_size;
        let py = exit_y * cell_size;
        // Remove exit walls visually (make opening)
        let sx = px + cell_size;
        let sy = py + cell_size;
        for i in 1..cell_size {
            let x_pos = sx.saturating_sub(i);
            let y_pos = sy.saturating_sub(i);
            if x_pos < img_width && sy < img_height {
                *img.get_pixel_mut(x_pos, sy) = Rgb([255, 255, 255]);
            }
            if sx < img_width && y_pos < img_height {
                *img.get_pixel_mut(sx, y_pos) = Rgb([255, 255, 255]);
            }
        }
    }

    Ok(img)
}

/// Render a maze with solution path highlighted
pub fn render_maze_with_solution(
    maze: &Maze,
    cell_size: u32,
    solution: &[(u32, u32)],
) -> Result<RgbImage, String> {
    let mut img = render_maze(maze, cell_size)?;

    // Draw solution path in red
    for &(x, y) in solution {
        let px = x * cell_size;
        let py = y * cell_size;

        // Fill the cell center with red (leave some border)
        let margin = cell_size / 4;
        for dy in margin..(cell_size - margin) {
            for dx in margin..(cell_size - margin) {
                let px_pos = px + dx;
                let py_pos = py + dy;
                if px_pos < img.width() && py_pos < img.height() {
                    *img.get_pixel_mut(px_pos, py_pos) = Rgb([255, 0, 0]);
                }
            }
        }
    }

    Ok(img)
}

/// Save a maze to a PNG file
pub fn save_maze(maze: &Maze, cell_size: u32, output_path: &str) -> Result<(), String> {
    let img = render_maze(maze, cell_size)?;
    img.save(output_path)
        .map_err(|e| format!("Failed to save image: {}", e))?;
    Ok(())
}

/// Save a maze with solution to a PNG file
pub fn save_maze_with_solution(
    maze: &Maze,
    cell_size: u32,
    solution: &[(u32, u32)],
    output_path: &str,
) -> Result<(), String> {
    let img = render_maze_with_solution(maze, cell_size, solution)?;
    img.save(output_path)
        .map_err(|e| format!("Failed to save image: {}", e))?;
    Ok(())
}

