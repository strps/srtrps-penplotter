"""Page setup: paper size, margins and the usable drawing area (all in mm)."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class Area:
    """Rectangular drawing area in mm."""

    x: float
    y: float
    width: float
    height: float

    @property
    def cx(self) -> float:
        return self.x + self.width / 2

    @property
    def cy(self) -> float:
        return self.y + self.height / 2

    @property
    def right(self) -> float:
        return self.x + self.width

    @property
    def bottom(self) -> float:
        return self.y + self.height

    def contains(self, px: float, py: float) -> bool:
        return self.x <= px <= self.right and self.y <= py <= self.bottom


def setup_page(vsk, size: str = "a4", landscape: bool = False, margin: float = 15.0) -> Area:
    """Configure the vsketch page in mm and return the area inside the margins.

    After this call, drawing coordinates are millimetres with (0, 0) at the
    top-left corner of the paper. Note: do not combine with `vsketch.Param(unit=...)`,
    which already converts values to pixels.
    """
    vsk.size(size, landscape=landscape, center=False)
    vsk.scale("mm")
    # vsk.width/height are in CSS pixels (96 dpi)
    width_mm = vsk.width / 96 * 25.4
    height_mm = vsk.height / 96 * 25.4
    return Area(margin, margin, width_mm - 2 * margin, height_mm - 2 * margin)
