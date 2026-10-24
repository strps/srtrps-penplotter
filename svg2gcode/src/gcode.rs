// src/gcode.rs
//
// SVG -> G-code converter (recursive, transform-aware, adaptive sampling)
// Uses usvg 0.45.x API:
// - Tree::root() -> &Group
// - Group::children() -> &[Node]
// - Node enum has Group(Box<Group>), Path(Box<Path>), ...
// - Path::data() gives tiny_skia_path::Path (absolute segments)
// - Path::abs_transform() returns resolved absolute transform
//
// Public API preserved:
// pub fn svg_bytes_to_gcode(svg_data: &[u8], samples: usize, tolerance: Option<f64>, feedrate: f64, pen_down: u8, pen_up: u8) -> String

use usvg::tiny_skia_path::{Path as SkiaPath, PathSegment, Point as SkPoint};
use usvg::{Group, Node, Options, Path, Transform, Tree};

/// Top-level public function (keeps same interface semantics)
/// - `samples`: base number of samples per curve when not using adaptive tolerance
/// - `tolerance`: optional adaptive tolerance in same units as SVG (e.g. mm)
/// - `feedrate`: feedrate for G1 moves
/// - `pen_down` / `pen_up`: servo values (or Z positions) for pen down/up
pub fn svg_bytes_to_gcode(
    svg_data: &[u8],
    samples: usize,
    tolerance: Option<f64>,
    feedrate: f64,
    pen_down: &String,
    pen_up: &String,
    plotter_transform: &PlotterTransform,
) -> String {
    let opt = Options {
        dpi: 25.4f32, // Set DPI to 25.4 so that parsed coordinates are in mm (1 pixel = 1 mm)
        ..Default::default()
    };
    let tree = Tree::from_data(svg_data, &opt).expect("❌ Failed to parse SVG data.");

    let mut gcode = Vec::new();

    gcode.push("G21 ; units = mm".to_string());
    gcode.push("G90 ; absolute positioning".to_string());
    gcode.push(format!("; feedrate default: {:.2}", feedrate));

    // Start recursion at root group
    let root: &Group = tree.root();
    process_group(
        root, &mut gcode, samples, tolerance, feedrate, pen_down, pen_up,
        &tree, plotter_transform,
    );

    gcode.push("M2 ; end".to_string());
    gcode.join("\n")
}

/// Recursively process a Group and its children.
/// We follow the docs' recommended approach: iterate over group.children(),
/// match Node::Group / Node::Path, and also call node.subroots(...) to
/// process clipPaths/masks/pattern subtrees.
fn process_group(
    group: &Group,
    gcode: &mut Vec<String>,
    samples: usize,
    tolerance: Option<f64>,
    feedrate: f64,
    pen_down: &String,
    pen_up: &String,
    tree: &Tree,
    plotter_transform: &PlotterTransform,
) {
    for node in group.children() {
        match node {
            Node::Group(subg) => {
                // Recurse into subgroup
                process_group(subg, gcode, samples, tolerance, feedrate, pen_down, pen_up, tree, plotter_transform);
            }
            Node::Path(path) => {
                process_path(path, gcode, samples, tolerance, feedrate, pen_down, pen_up, tree, plotter_transform);
            }
            _ => {
                // ignore Image, Text, etc. (could be extended)
            }
        }

        // Handle subroots (clipPaths, masks, patterns) which contain their own subtrees
        node.subroots(|subroot_group| {
            process_group(
                subroot_group,
                gcode,
                samples,
                tolerance,
                feedrate,
                pen_down,
                pen_up,
                tree,
                plotter_transform,
            )
        });
    }
}

/// Process a single Path node: sample its segments, apply the absolute transform,
/// and emit G-code moves.
///
/// Important: we use Path::data() (absolute coordinates) and Path::abs_transform()
/// (which returns the already-resolved transform including ancestors).
fn process_path(
    path: &Path,
    gcode: &mut Vec<String>,
    samples: usize,
    tolerance: Option<f64>,
    feedrate: f64,
    pen_down: &String,
    pen_up: &String,
    tree : &Tree,
    plotter_transform : &PlotterTransform,
) {
    // Get absolute transform for this node (includes ancestors)
    let transform: Transform = path.abs_transform();

    // We'll iterate path.data().segments(); segments are in absolute coordinates (object space)
    // but we still need the "previous point" when sampling a cubic/quadratic,
    // so we maintain current_pos while we iterate.
    let sk_path: &SkiaPath = path.data();
    let mut current_pos = (0.0f64, 0.0f64);
    let mut all_points: Vec<(f64, f64)> = Vec::new();

    for seg in sk_path.segments() {
        // produce points in object coordinates using current_pos
        let pts = segment_to_points(&seg, current_pos, samples, tolerance);
        if !pts.is_empty() {
            // update current position to last sampled point (segments are absolute)
            current_pos = *pts.last().unwrap();
            // apply absolute transform to each point and push to all_points
            for (x, y) in pts {
                // Transform::map_point needs a tiny_skia_path::Point
                let mut p = SkPoint::from_xy(x as f32, y as f32);
                transform.map_point(&mut p);
                let (px, py) = to_plotter_coords(p.x as f64, p.y as f64, tree, plotter_transform);
                all_points.push((px, py));
            }
        }
    }

    if all_points.is_empty() {
        return;
    }

    // Emit G-code for the path:
    // Move (pen up) to first point, lower pen, draw, raise pen.
    let (sx, sy) = all_points[0];
    gcode.push(format!("; --- path start ---"));
    gcode.push(format!("G0 X{:.3} Y{:.3}", sx, sy)); // rapid move to start
    gcode.push(format!("{} ; pen up", pen_up));
    gcode.push(format!("{} ; pen down", pen_down));

    for &(x, y) in all_points.iter().skip(1) {
        gcode.push(format!("G1 X{:.3} Y{:.3} F{}", x, y, feedrate));
    }

    gcode.push(format!("{} ; pen up", pen_up));
    gcode.push(format!("; --- path end ---"));
}

/// Convert a single PathSegment into a sequence of sampled points in *object coordinates*
///
/// - `start_pos` is the starting point of the segment (required for curves)
/// - If `tolerance` is Some(t), adaptive sampling is used. Otherwise `samples` fixed steps are used.
fn segment_to_points(
    segment: &PathSegment,
    start_pos: (f64, f64),
    samples: usize,
    tolerance: Option<f64>,
) -> Vec<(f64, f64)> {
    let mut pts: Vec<(f64, f64)> = Vec::new();

    match segment {
        PathSegment::MoveTo(p) => {
            pts.push((p.x as f64, p.y as f64));
        }
        PathSegment::LineTo(p) => {
            pts.push((p.x as f64, p.y as f64));
        }
        PathSegment::CubicTo( c1, c2, end) => {
            let x0 = start_pos.0;
            let y0 = start_pos.1;



            if let Some(tol) = tolerance {
                // Adaptive subdivision based on deviation from chord.
                adaptive_cubic(x0, y0, c1, c2, end, tol, &mut pts);
            } else {
                // Fixed sampling
                let steps = samples.max(2);
                for i in 0..=steps {
                    let t = i as f64 / steps as f64;
                    let (x, y) = cubic_point(x0, y0, c1, c2, end, t);
                    pts.push((x, y));
                }
            }
        }
        PathSegment::QuadTo(end, c1) => {
            let x0 = start_pos.0;
            let y0 = start_pos.1;

            if let Some(tol) = tolerance {
                adaptive_quad(x0, y0, c1, end, tol, &mut pts);
            } else {
                let steps = samples.max(2);
                for i in 0..=steps {
                    let t = i as f64 / steps as f64;
                    let (x, y) = quad_point(x0, y0, c1, end, t);
                    pts.push((x, y));
                }
            }
        }
        PathSegment::Close => {
            // nothing to add (could return to subpath start if you track it)
        }
    }

    pts
}

/// Evaluate cubic Bezier at parameter t (0..1)
fn cubic_point(x0: f64, y0: f64, c1: &SkPoint, c2: &SkPoint, end: &SkPoint, t: f64) -> (f64, f64) {
    let t1 = 1.0 - t;
    let xt = t1.powi(3) * x0
        + 3.0 * t1.powi(2) * t * (c1.x as f64)
        + 3.0 * t1 * t.powi(2) * (c2.x as f64)
        + t.powi(3) * (end.x as f64);
    let yt = t1.powi(3) * y0
        + 3.0 * t1.powi(2) * t * (c1.y as f64)
        + 3.0 * t1 * t.powi(2) * (c2.y as f64)
        + t.powi(3) * (end.y as f64);
    (xt, yt)
}

/// Evaluate quadratic Bezier at parameter t (0..1)
fn quad_point(x0: f64, y0: f64, c1: &SkPoint, end: &SkPoint, t: f64) -> (f64, f64) {
    let t1 = 1.0 - t;
    let xt = t1.powi(2) * x0 + 2.0 * t1 * t * (c1.x as f64) + t.powi(2) * (end.x as f64);
    let yt = t1.powi(2) * y0 + 2.0 * t1 * t * (c1.y as f64) + t.powi(2) * (end.y as f64);
    (xt, yt)
}

/// Adaptive cubic subdivision: push only the end points of segments that are within tolerance
fn adaptive_cubic(
    x0: f64,
    y0: f64,
    c1: &SkPoint,
    c2: &SkPoint,
    end: &SkPoint,
    tol: f64,
    out: &mut Vec<(f64, f64)>,
) {
    adaptive_cubic_rec(
        (x0, y0),
        (c1.x as f64, c1.y as f64),
        (c2.x as f64, c2.y as f64),
        (end.x as f64, end.y as f64),
        tol,
        out,
    );
}

fn adaptive_cubic_rec(
    start: (f64, f64),
    c1: (f64, f64),
    c2: (f64, f64),
    end: (f64, f64),
    tol: f64,
    out: &mut Vec<(f64, f64)>,
) {
    // midpoint on the curve (approx) using De Casteljau / averaged control points
    let mid_curve = (
        (start.0 + 3.0 * c1.0 + 3.0 * c2.0 + end.0) / 8.0,
        (start.1 + 3.0 * c1.1 + 3.0 * c2.1 + end.1) / 8.0,
    );
    // midpoint on the chord
    let mid_chord = ((start.0 + end.0) / 2.0, (start.1 + end.1) / 2.0);

    let dx = mid_curve.0 - mid_chord.0;
    let dy = mid_curve.1 - mid_chord.1;
    let dist = (dx * dx + dy * dy).sqrt();

    if dist <= tol {
        // good approximation: append endpoint
        out.push(end);
    } else {
        // subdivide curve into two halves via De Casteljau
        let mid1 = ((start.0 + c1.0) / 2.0, (start.1 + c1.1) / 2.0);
        let mid2 = ((c1.0 + c2.0) / 2.0, (c1.1 + c2.1) / 2.0);
        let mid3 = ((c2.0 + end.0) / 2.0, (c2.1 + end.1) / 2.0);

        let left_ctrl = ((mid1.0 + mid2.0) / 2.0, (mid1.1 + mid2.1) / 2.0);
        let right_ctrl = ((mid2.0 + mid3.0) / 2.0, (mid2.1 + mid3.1) / 2.0);
        let mid_pt = (
            (left_ctrl.0 + right_ctrl.0) / 2.0,
            (left_ctrl.1 + right_ctrl.1) / 2.0,
        );

        // Recurse left and right halves
        adaptive_cubic_rec(start, mid1, left_ctrl, mid_pt, tol, out);
        adaptive_cubic_rec(mid_pt, right_ctrl, mid3, end, tol, out);
    }
}

/// Adaptive quadratic subdivision
fn adaptive_quad(
    x0: f64,
    y0: f64,
    c1: &SkPoint,
    end: &SkPoint,
    tol: f64,
    out: &mut Vec<(f64, f64)>,
) {
    adaptive_quad_rec(
        (x0, y0),
        (c1.x as f64, c1.y as f64),
        (end.x as f64, end.y as f64),
        tol,
        out,
    );
}

fn adaptive_quad_rec(
    start: (f64, f64),
    ctrl: (f64, f64),
    end: (f64, f64),
    tol: f64,
    out: &mut Vec<(f64, f64)>,
) {
    // measure curve midpoint vs chord midpoint
    let mid_curve = (
        (start.0 + 2.0 * ctrl.0 + end.0) / 4.0,
        (start.1 + 2.0 * ctrl.1 + end.1) / 4.0,
    );
    let mid_chord = ((start.0 + end.0) / 2.0, (start.1 + end.1) / 2.0);
    let dx = mid_curve.0 - mid_chord.0;
    let dy = mid_curve.1 - mid_chord.1;
    let dist = (dx * dx + dy * dy).sqrt();

    if dist <= tol {
        out.push(end);
    } else {
        // subdivide
        let mid1 = ((start.0 + ctrl.0) / 2.0, (start.1 + ctrl.1) / 2.0);
        let mid2 = ((ctrl.0 + end.0) / 2.0, (ctrl.1 + end.1) / 2.0);
        let mid_pt = ((mid1.0 + mid2.0) / 2.0, (mid1.1 + mid2.1) / 2.0);

        adaptive_quad_rec(start, mid1, mid_pt, tol, out);
        adaptive_quad_rec(mid_pt, mid2, end, tol, out);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ReferencePoint {
    BottomLeft,
    TopLeft,
    Center,
    // #[allow(dead_code)]
    Custom(f64, f64),
}

#[derive(Debug, Clone, Copy)]
pub struct PlotterTransform {
    pub reference_point: ReferencePoint,
    pub flip_y: bool,
    pub offset_x: f64,
    pub offset_y: f64,
}

fn to_plotter_coords(
    x: f64,
    y: f64,
    tree: &Tree,
    transform: &PlotterTransform,
) -> (f64, f64) {
    // Get SVG canvas dimensions (now in mm, since dpi=25.4)
    let size = tree.size();
    let width = size.width() as f64;
    let height = size.height() as f64;


    // Coordinates are already in mm
    let mut px = x;
    let mut py = y;

    // Flip Y if needed (SVG Y+ down → Plotter Y+ up)
    if transform.flip_y {
        py = height - y;
    }

    // Adjust based on reference point
    match transform.reference_point {
        ReferencePoint::BottomLeft => {
            // Default behavior, nothing else needed
        }
        ReferencePoint::TopLeft => {
            py -= height;
        }
        ReferencePoint::Center => {
            px -= width / 2.0;
            py -= height / 2.0;
        }
        ReferencePoint::Custom(cx, cy) => {
            px -= cx;
            py -= cy;
        }
    }

    // Apply user offsets
    px += transform.offset_x;
    py += transform.offset_y;

    (px, py)
}