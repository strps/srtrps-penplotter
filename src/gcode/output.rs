// src/gcode/output.rs
//
// G-code Output Module: Builds and optimizes G-code from processed paths.
//
// This module takes flattened points and generates optimized G-code commands,
// handling pen movements, feedrates, and transformations.


/// Generates G-code from a list of flattened points for a single path.
/// Applies plotter transforms and emits G-code commands.
pub fn generate_gcode_for_path(
    points: &[(f64, f64)],
    feedrate: f64,
    pen_down: &str,
    pen_up: &str,
) -> Vec<String> {
    if points.is_empty() {
        return Vec::new();
    }

    let mut gcode = Vec::new();
    gcode.push("; --- path start ---".to_string());
    let (sx, sy) = points[0];
    gcode.push(format!("G0 X{:.3} Y{:.3}", sx, sy)); // rapid move to start
    gcode.push(format!("{} ; pen up", pen_up));
    gcode.push(format!("{} ; pen down", pen_down));

    for &(x, y) in points.iter().skip(1) {
        gcode.push(format!("G1 X{:.3} Y{:.3} F{}", x, y, feedrate));
    }

    gcode.push(format!("{} ; pen up", pen_up));
    gcode.push("; --- path end ---".to_string());
    gcode
}

/// Generates complete G-code from multiple paths.
/// Includes header comments and end command.
pub fn generate_full_gcode(
    paths: &[Vec<(f64, f64)>],
    feedrate: f64,
    pen_down: &str,
    pen_up: &str,
) -> String {
    let mut gcode = Vec::new();
    gcode.push("G21 ; units = mm".to_string());
    gcode.push("G90 ; absolute positioning".to_string());
    gcode.push(format!("; feedrate default: {:.2}", feedrate));

    for points in paths {
        let path_gcode = generate_gcode_for_path(points, feedrate, pen_down, pen_up);
        gcode.extend(path_gcode);
    }

    gcode.push("M2 ; end".to_string());
    gcode.join("\n")
}

/// Optimizes G-code by removing redundant moves or smoothing paths.
/// Placeholder for future optimizations like arc fitting or dead zone removal.
pub fn optimize_gcode(gcode: Vec<String>) -> Vec<String> {
    // TODO: Implement optimizations like arc conversion, duplicate removal, etc.
    gcode
}

// TODO: Add support for different G-code dialects (e.g., GRBL, Marlin).
// TODO: Integrate with devices module for streaming.
