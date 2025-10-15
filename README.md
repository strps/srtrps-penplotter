# PenPlotter

A Rust-based API for converting SVG to G-code for pen plotters, with pattern filling support using Lyon.

## Backend (Rust + Axum)
- API endpoint: `POST /api/convert` (multipart form with SVG file and params).
- Serves frontend at `/`.
- Supports hatching/cross-hatching for filled paths.

### Run Backend
1. `cargo run`
2. Visit `http://localhost:3000` for frontend.

## Frontend (Vite + React + TypeScript)
- Upload SVG, configure params (samples, tolerance, feedrate, pen commands, pattern).
- Download generated G-code.

### Run Frontend Dev
1. `cd frontend`
2. `npm install`
3. `npm run dev` (for hot reload during development).

### Build Frontend
1. `cd frontend`
2. `npm run build` (outputs to `dist/`, served by backend).

## Usage
- Upload SVG file.
- Set params (e.g., pattern: `hatch:1.0:45` for 1mm diagonal hatching).
- Convert and download G-code.

## Dependencies
- Backend: Axum, Tokio, usvg, Lyon.
- Frontend: Vite, React, TypeScript.

## License
MIT