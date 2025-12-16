use crate::config::parse_hex_color;
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

/// Draw a line between two points using Bresenham's line algorithm
fn draw_line(
    img: &mut RgbImage,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: Rgb<u8>,
    thickness: f32,
) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    let mut x = x0;
    let mut y = y0;

    let thickness_int = thickness.max(1.0) as i32;
    let half_thickness = (thickness_int / 2) as i32;

    loop {
        // Draw a circle/square of pixels for thickness
        for dy_offset in -half_thickness..=half_thickness {
            for dx_offset in -half_thickness..=half_thickness {
                let px = (x + dx_offset) as u32;
                let py = (y + dy_offset) as u32;
                if px < img.width() && py < img.height() {
                    *img.get_pixel_mut(px, py) = color;
                }
            }
        }

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

/// Render a maze with solution path highlighted as a line
pub fn render_maze_with_solution(
    maze: &Maze,
    cell_size: u32,
    solution: &[(u32, u32)],
    line_color: &str,
    line_thickness: Option<f32>,
) -> Result<RgbImage, String> {
    let mut img = render_maze(maze, cell_size)?;

    if solution.is_empty() {
        return Ok(img);
    }

    // Parse hex color
    let rgb = parse_hex_color(line_color)?;
    let color = Rgb([rgb[0], rgb[1], rgb[2]]);

    // Calculate line thickness (default: 1/3 of cell_size)
    let thickness = line_thickness.unwrap_or_else(|| cell_size as f32 / 3.0);

    // Convert solution cell coordinates to pixel coordinates (center of each cell)
    let pixel_coords: Vec<(i32, i32)> = solution
        .iter()
        .map(|&(x, y)| {
            let px = (x * cell_size + cell_size / 2) as i32;
            let py = (y * cell_size + cell_size / 2) as i32;
            (px, py)
        })
        .collect();

    // Draw lines connecting consecutive points
    for i in 0..(pixel_coords.len() - 1) {
        let (x0, y0) = pixel_coords[i];
        let (x1, y1) = pixel_coords[i + 1];
        draw_line(&mut img, x0, y0, x1, y1, color, thickness);
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
    line_color: &str,
    line_thickness: Option<f32>,
) -> Result<(), String> {
    let img = render_maze_with_solution(maze, cell_size, solution, line_color, line_thickness)?;
    img.save(output_path)
        .map_err(|e| format!("Failed to save image: {}", e))?;
    Ok(())
}

