// src/gcode/parser.rs
//
// SVG Parser Module: Bridges usvg and kurbo for path parsing.
//
// This module handles loading SVG data using usvg and converting
// parsed paths into kurbo shapes for further geometry processing.
//
// Dependencies: usvg for SVG parsing, kurbo for geometry.

use kurbo::{Affine, BezPath, Vec2};
use usvg::{Group, Node, Path, Tree};

/// Loads and parses an SVG byte slice into a usvg Tree.
/// Returns the parsed tree or an error if parsing fails.
pub fn parse_svg(svg_data: &[u8]) -> Result<Tree, String> {
    if svg_data.is_empty() {
        return Err("SVG data is empty".to_string());
    }

    let opt = usvg::Options {
        dpi: 25.4, // Set DPI to 25.4 for mm units
        ..Default::default()
    };

    Tree::from_data(svg_data, &opt).map_err(|e| format!("SVG parsing failed: {}", e))
}

/// Extracts all paths from the usvg Tree and converts them to kurbo BezPath.
/// This performs recursive traversal similar to gcode.rs, handling groups and subroots.
/// Returns a vector of BezPath for each path in the SVG, or an error if extraction fails.
pub fn extract_paths(tree: &Tree, reference_point: Option<&str>) -> Result<Vec<BezPath>, String> {
    let mut paths = Vec::new();
    let root: &Group = tree.root();

    // Basic check: ensure the tree has a root
    if root.children().is_empty() && root.children().len() == 0 {
        return Err("SVG tree is empty or has no root children".to_string());
    }

    process_group(root, tree, &mut paths, reference_point);
    Ok(paths)
}

/// Recursively process a Group and its children.
/// Follows the same logic as gcode.rs: iterate over children, handle Group/Path nodes,
/// and process subroots (clipPaths, masks, patterns).
/// Collects paths into the provided vector.
fn process_group(group: &Group, tree: &Tree, paths: &mut Vec<BezPath>, reference_point: Option<&str>) {
    let mut affine_transform = Affine::FLIP_Y;

    // Calculate translation based on SVG size and reference point
    let svg_size = tree.size();
    let (width, height) = (svg_size.width() as f64, svg_size.height() as f64);
    let translation = calculate_translation(width, height, reference_point);
    affine_transform = affine_transform.then_translate(translation);

    for node in group.children() {
        match node {
            Node::Group(subgroup) => {
                // Recurse into subgroup
                process_group(subgroup, tree, paths, reference_point);
            }
            Node::Path(path) => {
                // Convert path to kurbo and add to list
                let mut bez_path = convert_usvg_path_to_kurbo(path);
                if !bez_path.is_empty() {
                    bez_path.apply_affine(affine_transform);
                    paths.push(bez_path);
                } else {
                    eprintln!("Warning: Empty path encountered and skipped");
                }
            }
            _ => {
                // Ignore Image, Text, etc. (could be extended)
            }
        }

        // Handle subroots (clipPaths, masks, patterns) which contain their own subtrees
        node.subroots(|subroot_group| {
            process_group(subroot_group, tree, paths, reference_point);
        });
    }
}

/// Converts a usvg Path to a kurbo BezPath, applying absolute transforms.
/// Handles path segments and applies the path's absolute transform to all points.
/// Returns the transformed BezPath.
fn convert_usvg_path_to_kurbo(path: &Path) -> BezPath {
    let mut bez_path = BezPath::new();
    let transform = path.abs_transform(); // Get absolute transform including ancestors
    let sk_path = path.data();

    for segment in sk_path.segments() {
        match segment {
            usvg::tiny_skia_path::PathSegment::MoveTo(p) => {
                let mut point = usvg::tiny_skia_path::Point::from_xy(p.x, p.y);
                transform.map_point(&mut point);
                bez_path.move_to(kurbo::Point::new(point.x as f64, point.y as f64));
            }
            usvg::tiny_skia_path::PathSegment::LineTo(p) => {
                let mut point = usvg::tiny_skia_path::Point::from_xy(p.x, p.y);
                transform.map_point(&mut point);
                bez_path.line_to(kurbo::Point::new(point.x as f64, point.y as f64));
            }
            usvg::tiny_skia_path::PathSegment::CubicTo(c1, c2, end) => {
                let mut p1 = usvg::tiny_skia_path::Point::from_xy(c1.x, c1.y);
                let mut p2 = usvg::tiny_skia_path::Point::from_xy(c2.x, c2.y);
                let mut pend = usvg::tiny_skia_path::Point::from_xy(end.x, end.y);
                transform.map_point(&mut p1);
                transform.map_point(&mut p2);
                transform.map_point(&mut pend);
                bez_path.curve_to(
                    kurbo::Point::new(p1.x as f64, p1.y as f64),
                    kurbo::Point::new(p2.x as f64, p2.y as f64),
                    kurbo::Point::new(pend.x as f64, pend.y as f64),
                );
            }
            usvg::tiny_skia_path::PathSegment::QuadTo(c1, end) => {
                let mut p1 = usvg::tiny_skia_path::Point::from_xy(c1.x, c1.y);
                let mut pend = usvg::tiny_skia_path::Point::from_xy(end.x, end.y);
                transform.map_point(&mut p1);
                transform.map_point(&mut pend);
                bez_path.quad_to(
                    kurbo::Point::new(p1.x as f64, p1.y as f64),
                    kurbo::Point::new(pend.x as f64, pend.y as f64),
                );
            }
            usvg::tiny_skia_path::PathSegment::Close => {
                bez_path.close_path();
            }
        }
    }

    bez_path
}

/// Calculates the translation vector based on SVG dimensions and reference point.
/// This ensures the SVG is positioned correctly relative to the specified reference point.
/// Note: We apply FLIP_Y first, so we need to account for the flipped Y coordinate system.
fn calculate_translation(width: f64, height: f64, reference_point: Option<&str>) -> Vec2 {
    match reference_point {
        Some("center") => Vec2::new(-width / 2.0, height / 2.0),
        Some("top_right") => Vec2::new(-width, height),
        Some("bottom_right") => Vec2::new(-width, 0.0),
        Some("bottom_left") | None => Vec2::new(0.0, height), // Default to bottom_left
        _ => {
            eprintln!("Warning: Unknown reference point '{}', defaulting to bottom_left", reference_point.unwrap());
            Vec2::new(0.0, height)
        }
    }
}

// TODO: Add support for other SVG features like text or images if required.
