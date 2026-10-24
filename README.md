# srtrps-penplotter

Pen plotter workspace.

| Folder | What |
| --- | --- |
| [svg2gcode/](svg2gcode/) | Rust + Axum API (and React frontend) that converts SVG to G-code |
| [sketches/](sketches/) | Generative art sketches, grouped by the framework they're written with |

Typical flow: write a sketch in `sketches/` → export SVG → convert with `svg2gcode` → plot.
