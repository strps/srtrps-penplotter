// src/gcode/geometry.rs
//
// Geometry Module: Handles path flattening, sampling, and transformations.
//
// This module provides utilities for converting kurbo paths into sampled points,
// applying tolerances for adaptive sampling, and preparing data for G-code output.
//
// Dependencies: kurbo for geometry, usvg for tree context if needed.

use kurbo::{BezPath, Point, Vec2};
use usvg::Tree;

/// Flattens a kurbo BezPath into a sequence of points for G-code generation.
/// Uses kurbo's built-in flattening with optional tolerance.
/// Returns one vector of (f64, f64) points per subpath, in SVG coordinates,
/// so each subpath can be drawn as a separate pen-down stroke.
pub fn flatten_path(path: &BezPath, tolerance: Option<f64>) -> Vec<Vec<(f64, f64)>> {
    let default_tolerance = 0.1; // Default tolerance if none provided
    let tol = tolerance.unwrap_or(default_tolerance);

    // Use kurbo's flatten free function with a callback to collect points
    let mut subpaths: Vec<Vec<(f64, f64)>> = Vec::new();
    // Start of the current subpath, needed so ClosePath can draw back to it
    let mut subpath_start = kurbo::Point::ZERO;

    kurbo::flatten(path, tol, |elem| {
        match elem {
            kurbo::PathEl::MoveTo(p) => {
                subpaths.push(vec![(p.x, p.y)]);
                subpath_start = p;
            }
            kurbo::PathEl::LineTo(p) => {
                if subpaths.is_empty() {
                    subpaths.push(Vec::new());
                }
                subpaths.last_mut().unwrap().push((p.x, p.y));
            }
            kurbo::PathEl::ClosePath => {
                if let Some(points) = subpaths.last_mut() {
                    points.push((subpath_start.x, subpath_start.y));
                }
            }
            _ => {
                // Ignore other elements as flattening should only produce MoveTo, LineTo, and ClosePath
            }
        }
    });

    subpaths.retain(|points| points.len() > 1);
    subpaths
}

// TODO: Integrate with PlotterTransform for coordinate mapping.
// TODO: Add bounding box calculations or other geometric utilities.
