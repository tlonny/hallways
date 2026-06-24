import { BLACK_COLOR } from "@www/constant"
import { node } from "@lonnycorp/htmlforge"

const FOOTER_FONT_SIZE = "14px"
const FOOTER_PADDING = "8px"
const LONNY_URL = "https://www.lonnycorp.com"

export class Footer implements node.Buildable {

    private readonly root: node.Element

    constructor() {
        this.root = new node.Element("footer")
            .style("background", BLACK_COLOR)
            .style("box-sizing", "border-box")
            .style("color", "#fff")
            .style("font-size", FOOTER_FONT_SIZE)
            .style("line-height", "1.4")
            .style("padding", FOOTER_PADDING)
            .style("text-align", "center")
            .style("width", "100%")
            .child(new node.Text("Created by the "))
            .child(
                new node.Element("a")
                    .attribute("href", LONNY_URL)
                    .style("color", "#fff")
                    .style("text-decoration", "underline")
                    .child(new node.Text("Lonny Corporation"))
            )
    }

    build() {
        return this.root.build()
    }

}
