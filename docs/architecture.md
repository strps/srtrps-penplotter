# 🧱 Project Architecture

This document describes the structure, responsibilities, and design rationale behind the **Plotter G-code Server** — a Rust-based system for converting SVG drawings into G-code, ready for CNC or GRBL-controlled devices.

---

## 📁 Folder Overview

```
src/
├── main.rs                  # Axum API entry point
│
├── api/                     # Web layer (routes, handlers, serialization)
│   ├── mod.rs
│   ├── routes.rs            # Defines /convert, /status, etc.
│   ├── models.rs            # API-level structs (requests, responses)
│   └── error.rs             # Centralized HTTP error handling
│
├── gcode/                   # Core SVG → G-code logic
│   ├── mod.rs               # Entry point for gcode module
│   ├── parser.rs            # usvg + kurbo bridge (parse SVG paths)
│   ├── geometry.rs          # Path math, transforms, flattening
│   ├── hatch.rs             # Fill pattern generation (kurbo + geo)
│   ├── output.rs            # G-code building and optimization
│   └── writer.rs            # Save to file and stream G-code output
│
├── devices/                 # Optional layer for GRBL or future devices
│   ├── mod.rs
│   ├── grbl.rs              # Serial communication for GRBL
│   └── simulator.rs         # Optional simulated device driver
│
├── utils/                   # Shared helpers
│   ├── mod.rs
│   ├── transform.rs         # Coordinate + calibration helpers
│   ├── config.rs            # Configuration, defaults, constants
│   └── logging.rs           # Logging & diagnostics
│
└── output/                  # G-code output files (runtime-generated)
```

---

## 🧩 Module Responsibilities

### `main.rs`
- Initializes the Axum HTTP server.
- Mounts all API routes and global middleware.
- Manages startup logs and configuration loading.

### `api/`
Handles everything web-facing.
- `routes.rs` defines `/convert`, `/status`, and future endpoints.
- `models.rs` defines HTTP request/response types.
- `error.rs` provides consistent error handling across endpoints.

### `gcode/`
Core computational logic for converting SVG to G-code.
- `parser.rs`: Loads and walks through SVG trees using **usvg** and **kurbo**.
- `geometry.rs`: Handles geometric operations, path flattening, and transformations.
- `hatch.rs`: Generates fill or hatch patterns inside closed paths.
- `output.rs`: Translates sampled coordinates into valid G-code commands.
- `writer.rs`: Writes G-code to disk or streams it to clients.

### `devices/`
Abstracts hardware communication layers.
- `grbl.rs`: Handles serial communication for GRBL-based controllers.
- `simulator.rs`: Emulates a GRBL device for local testing.

### `utils/`
Reusable helpers and shared services.
- `transform.rs`: Manages coordinate conversion from SVG → plotter space.
- `config.rs`: Defines runtime configuration and environment constants.
- `logging.rs`: Sets up logging, tracing, and diagnostics.

### `output/`
Runtime directory for generated `.gcode` files. Ignored by version control.

---

## 🧭 Data Flow Overview

```
SVG Upload (via /convert)
        │
        ▼
API Layer (routes.rs)
        │  - Validates parameters
        │  - Passes file & options to gcode::svg_bytes_to_gcode()
        ▼
G-code Engine (parser.rs → geometry.rs → output.rs)
        │  - Parses SVG tree recursively
        │  - Applies transforms & flattening
        │  - Builds G-code commands
        ▼
Output (writer.rs)
        │  - Returns G-code response
        │  - Optionally saves to /output/file.gcode
        ▼
Device Layer (optional)
        - Streams commands to GRBL or simulated controller
```

---

## ⚙️ Design Principles

| Principle | Description |
|------------|--------------|
| **Separation of concerns** | Clear boundary between API, logic, and device control. |
| **Extensibility** | Each module can evolve independently (e.g., new fill algorithms or GRBL commands). |
| **Safety** | Full use of Rust's ownership and borrowing system to ensure memory safety and thread safety. |
| **Testability** | Modules like `geometry` and `hatch` can be tested in isolation. |
| **Performance** | Async I/O (Axum + Tokio) and efficient geometry processing via `kurbo` and `usvg`. |

---

## 🚀 Future Expansion

- Add support for multiple device backends (GRBL, Marlin, FluidNC).
- WebSocket-based live G-code streaming.
- Preview service (SVG → Canvas → G-code path visualization).
- Local caching and job queue for multiple plotters.
- Optional configuration UI built in React or Svelte.

---

## 🧱 Why kurbo Fits Perfectly

Once we integrate kurbo, all your G-code logic can work with a standard geometric model that is:
- Device-independent (just coordinates & transforms)
- Precise (double-precision math)
- Easily serializable for streaming to the controller

That will make adding things like:
- Preview rendering
- Hatch fills
- Bounding-box visualization
- Collision detection

Much easier down the road.

## 🔌 Next Milestones

Here’s a logical roadmap you could follow:

### Phase 1: Geometry Migration (now)
- ✅ Replace manual flattening + transforms with kurbo
- ✅ Keep the same API for /convert

### Phase 2: Plotter Transform & Offsets
- ✅ Implement device coordinate mapping cleanly with a PlotterTransform struct
- ✅ Support arbitrary SVG reference points (bottom-left, center, etc.)

### Phase 3: Fill Patterns
- ⚙️ Use kurbo paths + geo clipping to create hatch fills

### Phase 4: Local G-code Execution
- ⚙️ Add serial streaming using tokio-serial
- ⚙️ Basic GRBL protocol: $, ?, ~, !

### Phase 5: WebSocket + Dashboard
- 🌐 Live status updates + G-code progress
- 🌐 Manual controls (jogging, homing)

### Phase 6: Frontend
- 💻 Simple SPA served from Axum or static directory
- 💻 Real-time view of toolpath + controls

---

## 🧠 Summary

This architecture is designed to be **modular, scalable, and hardware-agnostic**.  
The G-code generation logic lives independently from the API and device layers, enabling this project to evolve from a simple converter into a **full-featured plotter control platform**.