"""Getting sketches out: SVG files and G-code via the svg2gcode API."""

from __future__ import annotations

import urllib.request
import uuid
from pathlib import Path

DEFAULT_CONVERTER_URL = "http://localhost:3005/api/convert"


def send_to_converter(
    svg_path: str | Path,
    out_path: str | Path | None = None,
    url: str = DEFAULT_CONVERTER_URL,
    **params: str | int | float | bool,
) -> Path:
    """POST an SVG to the svg2gcode backend and write the returned G-code.

    Extra keyword args are sent as form fields (samples, tolerance, feedrate,
    pen_down, pen_up, reference_point, flip_y). Returns the G-code path.
    """
    svg_path = Path(svg_path)
    out_path = Path(out_path) if out_path else svg_path.with_suffix(".gcode")

    boundary = uuid.uuid4().hex
    parts: list[bytes] = []
    for key, value in params.items():
        if isinstance(value, bool):
            value = str(value).lower()
        parts.append(
            f'--{boundary}\r\nContent-Disposition: form-data; name="{key}"\r\n\r\n{value}\r\n'.encode()
        )
    parts.append(
        f'--{boundary}\r\nContent-Disposition: form-data; name="file"; filename="{svg_path.name}"\r\n'
        "Content-Type: image/svg+xml\r\n\r\n".encode()
    )
    parts.append(svg_path.read_bytes())
    parts.append(f"\r\n--{boundary}--\r\n".encode())

    req = urllib.request.Request(
        url,
        data=b"".join(parts),
        headers={"Content-Type": f"multipart/form-data; boundary={boundary}"},
        method="POST",
    )
    with urllib.request.urlopen(req) as resp:
        out_path.write_bytes(resp.read())
    return out_path
