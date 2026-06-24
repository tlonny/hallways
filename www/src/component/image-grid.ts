import { BLACK_COLOR, MOBILE_MEDIA_QUERY } from "@www/constant"
import { node } from "@lonnycorp/htmlforge"

export class ImageGrid implements node.Buildable {

    private readonly root: node.Element

    constructor(private readonly aspectRatio: string) {
        this.root = new node.Element("div")
            .style("align-items", "stretch")
            .style("display", "grid")
            .style("gap", "12px")
            .style("grid-template-columns", "repeat(2, minmax(0, 1fr))")
            .style("grid-template-columns", "1fr", { mediaQuery: MOBILE_MEDIA_QUERY })
            .style("width", "100%")
    }

    image(src: string, caption: string) {
        this.root.child(
            new node.Element("figure")
                .style("border", "1px solid #000")
                .style("box-sizing", "border-box")
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("height", "100%")
                .style("margin", "0")
                .style("width", "100%")
                .child(
                    new node.Element("img")
                        .attribute("alt", caption)
                        .attribute("loading", "lazy")
                        .attribute("src", src)
                        .style("aspect-ratio", this.aspectRatio)
                        .style("display", "block")
                        .style("height", "auto")
                        .style("object-fit", "cover")
                        .style("width", "100%")
                )
                .child(
                    new node.Element("figcaption")
                        .style("background", BLACK_COLOR)
                        .style("box-sizing", "border-box")
                        .style("color", "#fff")
                        .style("flex", "1")
                        .style("font-size", "12px")
                        .style("line-height", "1.35")
                        .style("padding", "8px 10px")
                        .style("width", "100%")
                        .child(new node.Text(caption))
                )
        )

        return this
    }

    build() {
        return this.root.build()
    }

}
