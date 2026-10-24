"""Vector fields and streamline tracing."""

from __future__ import annotations

import math
from typing import Callable

import numpy as np

from .paper import Area

# angle(x, y) -> radians
AngleField = Callable[[float, float], float]


def noise_field(vsk, scale: float = 0.01, turns: float = 2.0, z: float = 0.0) -> AngleField:
    """Angle field from vsketch's Perlin noise. `turns` = how many full rotations
    the noise range [0, 1] maps to."""

    def field(x: float, y: float) -> float:
        return vsk.noise(x * scale, y * scale, z) * turns * 2 * math.pi

    return field


def trace(
    field: AngleField,
    x: float,
    y: float,
    area: Area,
    step: float = 0.5,
    max_steps: int = 400,
    both_directions: bool = True,
) -> np.ndarray:
    """Follow `field` from (x, y) until leaving `area`. Returns an (N, 2) array."""

    def walk(sign: float) -> list[tuple[float, float]]:
        px, py = x, y
        pts = []
        for _ in range(max_steps):
            a = field(px, py)
            px += sign * step * math.cos(a)
            py += sign * step * math.sin(a)
            if not area.contains(px, py):
                break
            pts.append((px, py))
        return pts

    forward = walk(1.0)
    if not both_directions:
        return np.array([(x, y), *forward])
    backward = walk(-1.0)
    return np.array([*reversed(backward), (x, y), *forward])
