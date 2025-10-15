use axum::{
    extract::{Multipart, Query},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::Deserialize;
use std::{fs, path::PathBuf};

mod gcode;
use gcode::{svg_bytes_to_gcode, PlotterTransform, ReferencePoint};

#[derive(Debug, Deserialize)]
struct ConvertParams {
    samples: Option<usize>,
    tolerance: Option<f64>,
    feedrate: Option<f64>,
    pen_down: Option<String>,
    pen_up: Option<String>,
    reference_point: Option<String>, // e.g., "bottom_left"
    flip_y: Option<bool>,
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
    let samples = params.samples.unwrap_or(10);
    let tolerance = params.tolerance;
    let feedrate = params.feedrate.unwrap_or(1000.0);
    let pen_down = params.pen_down.unwrap_or_else(|| "M3 S1000".to_string());
    let pen_up = params.pen_up.unwrap_or_else(|| "M5".to_string());
    // Parse reference point string to enum
    let reference_point = match params.reference_point.unwrap_or_else(|| "bottom_left".to_string()).as_str() {
        "bottom_left" => ReferencePoint::BottomLeft,
        "top_left" => ReferencePoint::TopLeft,
        "center" => ReferencePoint::Center,
        _ => ReferencePoint::BottomLeft, // fallback to bottom_left for unknown values
    };
    let flip_y = params.flip_y.unwrap_or(true);

    // --- Plotter transform ---
    let plotter_transform = PlotterTransform {
        reference_point,
        flip_y,
        offset_x: 0.0,
        offset_y: 0.0,
    };

    // --- Generate G-code ---
    let gcode = svg_bytes_to_gcode(
        &svg,
        samples,
        tolerance,
        feedrate,
        &pen_down,
        &pen_up,
        &plotter_transform,
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
