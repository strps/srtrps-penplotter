use axum::{
    routing::post,
    extract::Multipart,
    response::IntoResponse,
    Router,
};

mod gcode; // we’ll move SVG → G-code logic here

#[tokio::main]
async fn main() {
    // Build Axum router
    let app = Router::new().route("/convert", post(convert_svg));

    
    println!("🚀 Server running at http://{}", "addr");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn convert_svg(mut multipart: Multipart) -> impl IntoResponse {
    let mut svg_data: Option<Vec<u8>> = None;

    // read uploaded file
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            svg_data = Some(field.bytes().await.unwrap().to_vec());
        }
    }

    let svg = match svg_data {
        Some(data) => data,
        None => return "No file uploaded".into_response(),
    };

    // generate G-code
    let gcode = gcode::svg_bytes_to_gcode(&svg, 10, 1000.0, 30, 50);

    axum::response::Response::builder()
        .header("Content-Type", "text/plain")
        .body(gcode.into())
        .unwrap()
}
