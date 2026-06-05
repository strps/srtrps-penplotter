use axum::{
    extract::Multipart,
    response::IntoResponse,
    routing::post,
    Router,
};
use std::{collections::HashMap, fs, path::PathBuf};
use tower_http::cors::CorsLayer;

mod gcode;
use gcode::{svg_bytes_to_gcode, PlotterTransform, ReferencePoint};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/convert", post(convert_svg))
        .layer(CorsLayer::permissive());
    
    let port = std::env::var("PORT").unwrap_or_else(|_| "3005".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("❌ Failed to bind address {}. Is the port already in use?", addr));

    println!("🚀 Server running at http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn convert_svg(mut multipart: Multipart) -> impl IntoResponse {
    let mut svg_data: Option<Vec<u8>> = None;
    let mut fields: HashMap<String, String> = HashMap::new();

    // Read uploaded SVG file and form fields
    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name() {
            Some("file") => {
                svg_data = Some(field.bytes().await.unwrap().to_vec());
            }
            Some(name) => {
                let name = name.to_string();
                if let Ok(value) = field.text().await {
                    fields.insert(name, value);
                }
            }
            None => {}
        }
    }

    let svg = match svg_data {
        Some(data) => data,
        None => return "No file uploaded".into_response(),
    };

    // --- Default values ---
    let samples = fields
        .get("samples")
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);
    let tolerance = fields.get("tolerance").and_then(|v| v.parse().ok());
    let feedrate = fields
        .get("feedrate")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1000.0);
    let pen_down = fields
        .get("pen_down")
        .filter(|v| !v.is_empty())
        .cloned()
        .unwrap_or_else(|| "M3 S1000".to_string());
    let pen_up = fields
        .get("pen_up")
        .filter(|v| !v.is_empty())
        .cloned()
        .unwrap_or_else(|| "M5".to_string());
    // Parse reference point string to enum
    let reference_point = match fields
        .get("reference_point")
        .map(|s| s.as_str())
        .unwrap_or("bottom_left")
    {
        "bottom_left" => ReferencePoint::BottomLeft,
        "top_left" => ReferencePoint::TopLeft,
        "center" => ReferencePoint::Center,
        _ => ReferencePoint::BottomLeft, // fallback to bottom_left for unknown values
    };
    let flip_y = fields
        .get("flip_y")
        .and_then(|v| v.parse().ok())
        .unwrap_or(true);

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
    if let Some(filename) = fields.get("output") {
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
