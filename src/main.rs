use axum::{
    extract::{Multipart, Query},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::Deserialize;
use std::{fs, path::PathBuf};

mod gcode;
use gcode::{parser::{parse_svg, extract_paths}, geometry::flatten_path, output::generate_full_gcode};

#[derive(Debug, Deserialize)]
struct ConvertParams {
    tolerance: Option<f64>,
    feedrate: Option<f64>,
    pen_down: Option<String>,
    pen_up: Option<String>,
    reference_point: Option<String>, // e.g., "bottom_left"
    output: Option<String>,          // e.g., "plot1.gcode"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/convert", post(convert_svg));

    println!("🚀 Server running at http://0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("❌ Failed to bind address");

    axum::serve(listener, app).await.unwrap();
}

async fn convert_svg(
    Query(params): Query<ConvertParams>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut svg_data: Option<Vec<u8>> = None;

    // Read uploaded SVG file
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            svg_data = Some(field.bytes().await.unwrap().to_vec());
        }
    }

    let svg = match svg_data {
        Some(data) => data,
        None => return "No file uploaded".into_response(),
    };

    // --- Default values ---
    let tolerance = params.tolerance;
    let feedrate = params.feedrate.unwrap_or(1000.0);
    let pen_down = params.pen_down.unwrap_or_else(|| "M3 S1000".to_string());
    let pen_up = params.pen_up.unwrap_or_else(|| "M5".to_string());

    // --- Generate G-code ---
    let gcode = generate_gcode_from_svg(
        &svg,
        tolerance,
        feedrate,
        &pen_down,
        &pen_up,
        params.reference_point.as_deref(),
    );

    // --- Optional file output ---
    if let Some(filename) = params.output {
        let output_dir = PathBuf::from("./output");
        if let Err(e) = fs::create_dir_all(&output_dir) {
            eprintln!("⚠️ Failed to create output directory: {}", e);
        }

        let output_path = output_dir.join(filename);
        match fs::write(&output_path, &gcode) {
            Ok(_) => println!("💾 G-code saved to {:?}", output_path),
            Err(e) => eprintln!("❌ Failed to save G-code: {}", e),
        }
    }

    axum::response::Response::builder()
        .header("Content-Type", "text/plain")
        .body(gcode.into())
        .unwrap()
}

/// Generates G-code from SVG data using the new modules.
/// This replaces the old svg_bytes_to_gcode function.
fn generate_gcode_from_svg(
    svg_data: &[u8],
    tolerance: Option<f64>,
    feedrate: f64,
    pen_down: &str,
    pen_up: &str,
    reference_point: Option<&str>,
) -> String {
    // Parse SVG
    let tree = match parse_svg(svg_data) {
        Ok(t) => t,
        Err(e) => return format!("❌ SVG parsing failed: {}", e),
    };

    // Extract paths
    let bez_paths = match extract_paths(&tree, reference_point) {
        Ok(paths) => paths,
        Err(e) => return format!("❌ Path extraction failed: {}", e),
    };

    // Flatten paths (no transform for now)
    let mut all_points = Vec::new();
    for bez_path in bez_paths {
        let points = flatten_path(&bez_path, tolerance);

        if !points.is_empty() {
            all_points.push(points);
        }
    }

    // Generate G-code
    generate_full_gcode(&all_points, feedrate, pen_down, pen_up)
}
