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
/// Returns a vector of (f64, f64) points in SVG coordinates.
pub fn flatten_path(path: &BezPath, tolerance: Option<f64>) -> Vec<(f64, f64)> {
    let default_tolerance = 0.1; // Default tolerance if none provided
    let tol = tolerance.unwrap_or(default_tolerance);

    // Use kurbo's flatten free function with a callback to collect points
    let mut points = Vec::new();
    let mut current_pos = kurbo::Point::ZERO;

    kurbo::flatten(path, tol, |elem| {
        match elem {
            kurbo::PathEl::MoveTo(p) => {
                points.push((p.x, p.y));
                current_pos = p;
            }
            kurbo::PathEl::LineTo(p) => {
                points.push((p.x, p.y));
                current_pos = p;
            }
            kurbo::PathEl::ClosePath => {
                points.push((current_pos.x, current_pos.y));
            }
            _ => {
                // Ignore other elements as flattening should only produce MoveTo, LineTo, and ClosePath
            }
        }
    });

    points
}

// TODO: Integrate with PlotterTransform for coordinate mapping.
// TODO: Add bounding box calculations or other geometric utilities.
