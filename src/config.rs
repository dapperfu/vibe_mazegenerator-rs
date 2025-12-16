use std::fs;
use std::path::Path;

/// Parse a hex color string to RGB values
/// Accepts formats: "#ff0000", "ff0000", "#FF0000", "FF0000"
/// Returns (r, g, b) as u8 values
pub fn parse_hex_color(hex: &str) -> Result<[u8; 3], String> {
    let hex = hex.trim();
    let hex = if hex.starts_with('#') {
        &hex[1..]
    } else {
        hex
    };

    if hex.len() != 6 {
        return Err(format!("Hex color must be 6 digits, got: {}", hex));
    }

    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|e| format!("Invalid hex color (R): {}", e))?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|e| format!("Invalid hex color (G): {}", e))?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|e| format!("Invalid hex color (B): {}", e))?;

    Ok([r, g, b])
}

#[derive(Debug, Clone)]
pub enum Algorithm {
    RecursiveBacktracking,
    Kruskal,
    Prim,
    AldousBroder,
}

impl Algorithm {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "recursive_backtracking" | "recursive-backtracking" => Some(Algorithm::RecursiveBacktracking),
            "kruskal" => Some(Algorithm::Kruskal),
            "prim" => Some(Algorithm::Prim),
            "aldous_broder" | "aldous-broder" => Some(Algorithm::AldousBroder),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Algorithm::RecursiveBacktracking => "recursive_backtracking",
            Algorithm::Kruskal => "kruskal",
            Algorithm::Prim => "prim",
            Algorithm::AldousBroder => "aldous_broder",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub algorithm: Algorithm,
    pub complexity: f64,
    pub output: String,
    pub cell_size: u32,
    pub seed: Option<u64>,
    pub solution_line_color: String,
    pub solution_line_thickness: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 50,
            height: 50,
            algorithm: Algorithm::RecursiveBacktracking,
            complexity: 0.5,
            output: "maze.png".to_string(),
            cell_size: 10,
            seed: None,
            solution_line_color: "#ff0000".to_string(),
            solution_line_thickness: 0.33,
        }
    }
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        let parsed: toml::Value = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;

        let mut config = Config::default();

        if let Some(width) = parsed.get("width").and_then(|v| v.as_integer()) {
            config.width = width as u32;
        }

        if let Some(height) = parsed.get("height").and_then(|v| v.as_integer()) {
            config.height = height as u32;
        }

        if let Some(algorithm) = parsed.get("algorithm").and_then(|v| v.as_str()) {
            config.algorithm = Algorithm::from_str(algorithm)
                .ok_or_else(|| format!("Unknown algorithm: {}", algorithm))?;
        }

        if let Some(complexity) = parsed.get("complexity").and_then(|v| v.as_float()) {
            config.complexity = complexity.max(0.0).min(1.0);
        }

        if let Some(output) = parsed.get("output").and_then(|v| v.as_str()) {
            config.output = output.to_string();
        }

        if let Some(cell_size) = parsed.get("cell_size").and_then(|v| v.as_integer()) {
            config.cell_size = cell_size as u32;
        }

        if let Some(seed) = parsed.get("seed").and_then(|v| v.as_integer()) {
            config.seed = Some(seed as u64);
        }

        if let Some(line_color) = parsed.get("line_color").and_then(|v| v.as_str()) {
            // Validate the hex color format
            if parse_hex_color(line_color).is_ok() {
                config.solution_line_color = line_color.to_string();
            } else {
                return Err(format!("Invalid line_color format: {}", line_color));
            }
        }

        if let Some(line_thickness) = parsed.get("line_thickness").and_then(|v| v.as_float()) {
            let thickness = line_thickness as f32;
            if thickness >= 0.0 && thickness <= 1.0 {
                config.solution_line_thickness = thickness;
            } else {
                return Err("line_thickness must be between 0.0 and 1.0".to_string());
            }
        }

        Ok(config)
    }

    /// Load configuration, trying config file first, then using defaults
    pub fn load(config_path: Option<&str>) -> Self {
        let default_path = "config.toml";
        let path = config_path.unwrap_or(default_path);

        if Path::new(path).exists() {
            match Config::from_file(path) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Warning: Failed to load config file: {}. Using defaults.", e);
                    Config::default()
                }
            }
        } else {
            Config::default()
        }
    }

    /// Apply CLI overrides to the configuration
    pub fn with_cli_overrides(
        mut self,
        width: Option<u32>,
        height: Option<u32>,
        algorithm: Option<&str>,
        complexity: Option<f64>,
        output: Option<&str>,
        seed: Option<u64>,
        line_color: Option<&str>,
        line_thickness: Option<f32>,
    ) -> Self {
        if let Some(w) = width {
            self.width = w;
        }
        if let Some(h) = height {
            self.height = h;
        }
        if let Some(alg) = algorithm {
            if let Some(alg_enum) = Algorithm::from_str(alg) {
                self.algorithm = alg_enum;
            }
        }
        if let Some(c) = complexity {
            self.complexity = c.max(0.0).min(1.0);
        }
        if let Some(o) = output {
            self.output = o.to_string();
        }
        if let Some(s) = seed {
            self.seed = Some(s);
        }
        if let Some(lc) = line_color {
            if parse_hex_color(lc).is_ok() {
                self.solution_line_color = lc.to_string();
            }
        }
        if let Some(lt) = line_thickness {
            if lt >= 0.0 && lt <= 1.0 {
                self.solution_line_thickness = lt;
            }
        }
        self
    }
}

