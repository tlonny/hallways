import { BORDER_COLOR, MOBILE_MEDIA_QUERY } from "@src/constant"
import { node } from "@lonnycorp/htmlforge"

export class Card implements node.Buildable {

    private readonly root: node.Element

    constructor() {
        this.root = new node.Element("section")
            .style("background", "#f4e8c9")
            .style("border", `1px solid ${BORDER_COLOR}`)
            .style("box-shadow", "4px 4px 0 rgba(0, 0, 0, 0.18)")
            .style("box-sizing", "border-box")
            .style("display", "flex")
            .style("flex-direction", "column")
            .style("gap", "24px")
            .style("padding", "26px")
            .style("padding", "18px", { mediaQuery: MOBILE_MEDIA_QUERY })
            .style("width", "100%")
    }

    child(child: node.Buildable) {
        this.root.child(child)

        return this
    }

    build() {
        return this.root.build()
    }

}
