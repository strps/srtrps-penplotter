import vsketch
from shapely.geometry import Point

from plottools import setup_page
from plottools.hatching import hatch, spacing_for_tone


class HatchedCirclesSketch(vsketch.SketchClass):
    page_size = vsketch.Param("a4", choices=["a3", "a4", "a5", "letter"])
    landscape = vsketch.Param(False)
    margin = vsketch.Param(15.0)  # mm

    count = vsketch.Param(40, 1)
    min_radius = vsketch.Param(5.0)  # mm
    max_radius = vsketch.Param(35.0)  # mm
    outline = vsketch.Param(True)

    def draw(self, vsk: vsketch.Vsketch) -> None:
        area = setup_page(vsk, self.page_size, self.landscape, self.margin)

        # Each new circle is clipped by the ones already drawn, so they
        # appear stacked without overlapping ink.
        covered = Point(0, 0).buffer(0)
        for _ in range(self.count):
            r = vsk.random(self.min_radius, self.max_radius)
            x = vsk.random(area.x + r, area.right - r)
            y = vsk.random(area.y + r, area.bottom - r)
            visible = Point(x, y).buffer(r, quad_segs=32).difference(covered)
            covered = covered.union(Point(x, y).buffer(r, quad_segs=32))

            spacing = spacing_for_tone(vsk.random(0.1, 0.9))
            for line in hatch(visible, spacing, vsk.random(0, 180)):
                vsk.polygon(line)
            if self.outline:
                vsk.geometry(visible.boundary)

    def finalize(self, vsk: vsketch.Vsketch) -> None:
        vsk.vpype("linemerge linesimplify reloop linesort")


if __name__ == "__main__":
    HatchedCirclesSketch.display()
