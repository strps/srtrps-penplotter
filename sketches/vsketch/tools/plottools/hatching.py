"""Hatch fills for arbitrary shapes using shapely."""

from __future__ import annotations

import math

import numpy as np
from shapely import affinity
from shapely.geometry import LineString, MultiLineString
from shapely.geometry.base import BaseGeometry


def hatch(shape: BaseGeometry, spacing: float = 1.0, angle_deg: float = 45.0) -> list[np.ndarray]:
    """Parallel lines `spacing` mm apart at `angle_deg`, clipped to `shape`."""
    if shape.is_empty:
        return []
    c = shape.centroid
    rotated = affinity.rotate(shape, -angle_deg, origin=c)
    minx, miny, maxx, maxy = rotated.bounds
    lines = [
        LineString([(minx - 1, y), (maxx + 1, y)])
        for y in np.arange(miny + spacing / 2, maxy, spacing)
    ]
    clipped = MultiLineString(lines).intersection(rotated)
    clipped = affinity.rotate(clipped, angle_deg, origin=c)
    return [np.asarray(g.coords) for g in _lines(clipped)]


def cross_hatch(shape: BaseGeometry, spacing: float = 1.0, angle_deg: float = 45.0) -> list[np.ndarray]:
    return hatch(shape, spacing, angle_deg) + hatch(shape, spacing, angle_deg + 90)


def _lines(geom: BaseGeometry):
    if geom.is_empty:
        return
    if isinstance(geom, LineString):
        yield geom
    elif hasattr(geom, "geoms"):
        for g in geom.geoms:
            yield from _lines(g)


def spacing_for_tone(tone: float, min_spacing: float = 0.4, max_spacing: float = 4.0) -> float:
    """Map tone 0 (white) .. 1 (black) to hatch spacing; darker = denser."""
    tone = min(max(tone, 0.0), 1.0)
    return max_spacing * math.pow(min_spacing / max_spacing, tone)
