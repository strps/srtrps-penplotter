# vsketch sketches

```
vsketch/
├── tools/            # reusable helpers, installable package `plottools`
│   └── plottools/
│       ├── paper.py     # page size, margins, drawing area
│       ├── geometry.py  # pure-math helpers (no vsketch dependency)
│       ├── fields.py    # noise flow fields + streamline tracing
│       ├── hatching.py  # shapely-based hatch fills
│       └── export.py    # save SVG / send to the svg2gcode API
└── art/              # one folder per sketch (vsketch convention)
    ├── _template/       # copy this to start a new sketch
    ├── flow_field/
    └── hatched_circles/
```

Rule of thumb: if two sketches need it, it goes in `tools/`. Art code should only
describe the piece.

## Setup

```bash
cd sketches/vsketch
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt   # vsketch + editable install of tools/
```

## Run

```bash
vsk run art/flow_field          # live preview, reloads on save
vsk save art/flow_field         # export SVG to art/flow_field/output/
vsk save -n 5 -s 42 art/flow_field   # 5 variations starting at seed 42
```

## New sketch

```bash
cp -r art/_template art/my_sketch
mv art/my_sketch/sketch_template.py art/my_sketch/sketch_my_sketch.py
```

vsketch expects the file to be named `sketch_<folder>.py`.

## To G-code

Start the converter (`cd svg2gcode && cargo run`, listens on `:3005`), then either use
the frontend or from Python:

```python
from plottools.export import send_to_converter
send_to_converter("art/flow_field/output/flow_field_seed42.svg")
```
