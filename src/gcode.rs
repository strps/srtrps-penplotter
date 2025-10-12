use usvg::tiny_skia_path::PathSegment;
use usvg::{Options, Tree};

pub fn svg_bytes_to_gcode(
    svg_data: &[u8],
    samples: usize,
    feedrate: f64,
    pen_down: u8,
    pen_up: u8
) -> String {
    let opt = Options::default();
    let tree = Tree::from_data(svg_data, &opt).expect("❌ Failed to parse SVG data.");

    let mut gcode = Vec::new();
    gcode.push("G21 ; units in mm".to_string());
    gcode.push("G90 ; absolute positioning".to_string());

    for node in tree.root().children() {
        if let usvg::Node::Path(path) = node {
            println!("Processing path with {} segments", path.data().len());
            let mut current_pos = (0.0, 0.0);
            let mut all_points = Vec::new();

            for segment in path.data().segments() {
                let pts = segment_to_points(&segment, current_pos, samples);
                if !pts.is_empty() {
                    current_pos = *pts.last().unwrap();
                }
                all_points.extend(pts);
            }

            if !all_points.is_empty() {
                let (x_start, y_start) = all_points[0];
                gcode.push(format!("G0 X{:.2} Y{:.2}", x_start, y_start));
                gcode.push(format!("M300 S{}", pen_down));

                for (x, y) in all_points.iter().skip(1) {
                    gcode.push(format!("G1 X{:.2} Y{:.2} F{}", x, y, feedrate));
                }

                gcode.push(format!("M300 S{}", pen_up));
            }
        }else {
            println!("Skipping non-path node");
        }
    }

    gcode.push("M2".to_string());
    gcode.join("\n")
}

fn segment_to_points(segment: &PathSegment, start_pos: (f64, f64), samples: usize) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let (x0, y0) = start_pos;

    match segment {
        PathSegment::MoveTo(p) => points.push((p.x as f64, p.y as f64)),
        PathSegment::LineTo(p) => points.push((p.x as f64, p.y as f64)),
        PathSegment::CubicTo(p, p1, p2) => {
            let steps = samples.max(2);
            for i in 0..=steps {
                let t = i as f64 / steps as f64;
                let xt = (1.0 - t).powi(3) * x0
                       + 3.0*(1.0 - t).powi(2)*t * p1.x as f64
                       + 3.0*(1.0 - t)*t.powi(2) * p2.x as f64
                       + t.powi(3) * p.x as f64;
                let yt = (1.0 - t).powi(3) * y0
                       + 3.0*(1.0 - t).powi(2)*t * p1.y as f64
                       + 3.0*(1.0 - t)*t.powi(2) * p2.y as f64
                       + t.powi(3) * p.y as f64;
                points.push((xt, yt));
            }
        }
        PathSegment::QuadTo(p, p1) => {
            let steps = samples.max(2);
            for i in 0..=steps {
                let t = i as f64 / steps as f64;
                let xt = (1.0 - t).powi(2) * x0
                       + 2.0*(1.0 - t)*t * p1.x as f64
                       + t.powi(2) * p.x as f64;
                let yt = (1.0 - t).powi(2) * y0
                       + 2.0*(1.0 - t)*t * p1.y as f64
                       + t.powi(2) * p.y as f64;
                points.push((xt, yt));
            }
        }
        PathSegment::Close => {}
    }

    points
}
