import { MOBILE_MEDIA_QUERY } from "@www/constant"
import { node } from "@lonnycorp/htmlforge"

const MAIN_GAP = "26px"
const MAIN_MAX_WIDTH = "920px"
const MAIN_MOBILE_PADDING = "24px 14px 44px"
const MAIN_PADDING = "30px 26px 64px"

export class Main implements node.Buildable {

    private readonly content: node.Element
    private readonly root: node.Element

    constructor() {
        this.content = new node.Element("div")
            .style("align-items", "center")
            .style("box-sizing", "border-box")
            .style("display", "flex")
            .style("flex-direction", "column")
            .style("gap", MAIN_GAP)
            .style("padding", MAIN_PADDING)
            .style("padding", MAIN_MOBILE_PADDING, { mediaQuery: MOBILE_MEDIA_QUERY })
            .style("width", "100%")

        this.root = new node.Element("main")
            .style("box-sizing", "border-box")
            .style("display", "flex")
            .style("flex", "1")
            .style("flex-direction", "column")
            .style("margin", "0 auto")
            .style("max-width", MAIN_MAX_WIDTH)
            .style("width", "100%")
            .child(this.content)
    }

    child(child: node.Buildable) {
        this.content.child(child)

        return this
    }

    build() {
        return this.root.build()
    }

}
