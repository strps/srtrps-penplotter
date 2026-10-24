"""Pure-math helpers. No vsketch dependency so they're easy to test."""

from __future__ import annotations

import math

import numpy as np


def lerp(a: float, b: float, t: float) -> float:
    return a + (b - a) * t


def remap(v: float, in_min: float, in_max: float, out_min: float, out_max: float) -> float:
    return out_min + (v - in_min) * (out_max - out_min) / (in_max - in_min)


def polar(cx: float, cy: float, r: float, angle: float) -> tuple[float, float]:
    return cx + r * math.cos(angle), cy + r * math.sin(angle)


def circle_points(cx: float, cy: float, r: float, n: int = 128) -> np.ndarray:
    """Closed polyline (n+1 points) approximating a circle, as an (N, 2) array."""
    t = np.linspace(0, 2 * np.pi, n + 1)
    return np.column_stack([cx + r * np.cos(t), cy + r * np.sin(t)])


def rotate(points: np.ndarray, angle: float, origin: tuple[float, float] = (0.0, 0.0)) -> np.ndarray:
    c, s = math.cos(angle), math.sin(angle)
    ox, oy = origin
    p = np.asarray(points, dtype=float) - (ox, oy)
    return p @ np.array([[c, s], [-s, c]]) + (ox, oy)


def to_complex(points: np.ndarray) -> np.ndarray:
    """(N, 2) array -> complex array, the format vpype/vsketch use for lines."""
    p = np.asarray(points, dtype=float)
    return p[:, 0] + 1j * p[:, 1]
