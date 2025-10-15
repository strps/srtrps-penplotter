// src/gcode/mod.rs
//
// G-code Module: Entry point for SVG to G-code conversion logic.
//
// This module exposes the public API for parsing SVG and generating G-code.

pub mod parser;
pub mod geometry;
pub mod output;

// Re-export key functions for easy access
pub use parser::{parse_svg, extract_paths};
pub use geometry::flatten_path;
pub use output::{generate_gcode_for_path, generate_full_gcode};
