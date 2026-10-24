import vsketch

from plottools import setup_page


class TemplateSketch(vsketch.SketchClass):
    page_size = vsketch.Param("a4", choices=["a3", "a4", "a5", "letter"])
    landscape = vsketch.Param(False)
    margin = vsketch.Param(15.0)  # mm

    def draw(self, vsk: vsketch.Vsketch) -> None:
        area = setup_page(vsk, self.page_size, self.landscape, self.margin)

        vsk.rect(area.x, area.y, area.width, area.height)

    def finalize(self, vsk: vsketch.Vsketch) -> None:
        vsk.vpype("linemerge linesimplify reloop linesort")


if __name__ == "__main__":
    TemplateSketch.display()
