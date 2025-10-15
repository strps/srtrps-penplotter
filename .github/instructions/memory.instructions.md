---
applyTo: '**'
---
Has built a Rust + Axum 0.8.6 project that provides a web API to convert SVG files to G-code for their plotter. The code:
- Accepts an SVG upload via multipart form.
- Recursively traverses all SVG nodes using the current `usvg` API (`Tree::root()`, `Group::children()`, `Node` enum, and `subroots`).
- Converts paths to G-code with configurable sampling rate, feedrate, and pen up/down values.
- Represents a working endpoint (`/convert`) returning plain text G-code.

This code was the result of a learning process to understand how to use `usvg`’s modern API and recursive traversal, while keeping the approach idiomatic to Rust. It serves as the base for adding a front-end interface in future steps.