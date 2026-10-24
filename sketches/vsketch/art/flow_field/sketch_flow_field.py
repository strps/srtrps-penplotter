import vsketch

from plottools import setup_page
from plottools.fields import noise_field, trace


class FlowFieldSketch(vsketch.SketchClass):
    page_size = vsketch.Param("a4", choices=["a3", "a4", "a5", "letter"])
    landscape = vsketch.Param(False)
    margin = vsketch.Param(15.0)  # mm

    lines = vsketch.Param(600, 1)
    noise_scale = vsketch.Param(0.008, decimals=4)
    turns = vsketch.Param(1.5, decimals=2)
    step = vsketch.Param(0.6, decimals=2)
    max_steps = vsketch.Param(300, 1)

    def draw(self, vsk: vsketch.Vsketch) -> None:
        area = setup_page(vsk, self.page_size, self.landscape, self.margin)
        field = noise_field(vsk, self.noise_scale, self.turns)

        for _ in range(self.lines):
            x = vsk.random(area.x, area.right)
            y = vsk.random(area.y, area.bottom)
            pts = trace(field, x, y, area, self.step, self.max_steps)
            if len(pts) > 2:
                vsk.polygon(pts)

    def finalize(self, vsk: vsketch.Vsketch) -> None:
        vsk.vpype("linemerge linesimplify reloop linesort")


if __name__ == "__main__":
    FlowFieldSketch.display()
