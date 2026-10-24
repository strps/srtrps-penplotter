# 🖋️ Plotter G-code Server

A Rust-based web service that converts **SVG drawings into G-code** for plotters and CNC devices.  
Built with [Axum](https://docs.rs/axum), [usvg](https://docs.rs/usvg), and [tokio](https://tokio.rs), it’s designed to run locally or on embedded controller machines.

---

## ✨ Features

- 🧩 **SVG → G-code conversion** via `usvg` recursive traversal  
- ⚙️ **Configurable parameters:** sampling rate, tolerance, feedrate, pen up/down commands  
- 🦭 **Coordinate transforms:** map SVG space to plotter coordinates with origin and Y-flip options  
- 🌐 **REST API** with `/convert` endpoint for easy integration  
- 🧱 Built in **Rust + Axum 0.8.6** (async, safe, and blazing fast)
- 🚀 Future: **GRBL control**, WebSocket streaming, and live plot preview

---

## 🚀 Quick Start

### 1️⃣ Requirements

- Rust 1.80+  
- Cargo  
- (optional) Docker  

### 2️⃣ Clone & build

```bash
git clone https://github.com/yourname/plotter-gcode-server.git
cd plotter-gcode-server
cargo run
```

Server starts at:
```
http://localhost:3000
```

---

## 📡 API Usage

### `POST /convert`

Upload an SVG file and get G-code in return.

#### Form Fields
| Field | Type | Description |
|-------|------|-------------|
| `file` | File | The SVG file to convert |
| `samples` | number | Sampling rate for curves |
| `tolerance` | number (optional) | Curve approximation tolerance |
| `feedrate` | number | Movement feedrate |
| `pen_down` | string | Command for pen-down |
| `pen_up` | string | Command for pen-up |
| `origin` | string (optional) | `"bottom_left"`, `"center"`, `"top_left"`, or custom |
| `flip_y` | bool | Flip Y axis for plotter coordinates |
| `output_name` | string (optional) | Save G-code to file |

#### Example request
```bash
curl -X POST http://localhost:3000/convert \
  -F "file=@drawing.svg" \
  -F "samples=10" \
  -F "tolerance=0.05" \
  -F "feedrate=1000" \
  -F "pen_down=M3 S50" \
  -F "pen_up=M5" \
  -F "origin=bottom_left" \
  -F "flip_y=true"
```

#### Response
Plain-text G-code, optionally saved to `/output/<filename>.gcode`.

---

## 🦭 Coordinate Transform System

- The SVG coordinate space is automatically normalized using the document’s `size()`.  
- The Y-axis is optionally flipped for plotter compatibility.  
- You can select any reference point (`origin`) for coordinate mapping.  
- Future support: scaling, rotation, and calibration.

---

## 🧠 Architecture Overview

- **main.rs** — Web API entry point (Axum router)  
- **gcode.rs** — Core SVG → G-code engine  
- **models.rs** — Common data structures (`PlotterTransform`, etc.)  
- **output/** — Generated G-code files  

---

## 📍 Roadmap

- [x] Basic SVG traversal and G-code generation  
- [x] Plotter coordinate mapping  
- [ ] GRBL serial communication  
- [ ] Real-time preview (WebSocket + Canvas)  
- [ ] Authentication & settings persistence  

---

## 🤝 Contributing

Feel free to open issues or pull requests — this project is intended to grow into a flexible plotting and control framework.

---

## ⚖️ License

MIT License © 2025 Your Name

