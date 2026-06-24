import { BORDER_COLOR } from "@www/constant"
import { node } from "@lonnycorp/htmlforge"

export class Divider implements node.Buildable {

    private readonly root: node.Element

    constructor() {
        this.root = new node.Element("hr")
            .style("border", "0")
            .style("border-top", `1px solid ${BORDER_COLOR}`)
            .style("box-sizing", "border-box")
            .style("height", "0")
            .style("margin", "0")
            .style("width", "100%")
    }

    build() {
        return this.root.build()
    }

}
