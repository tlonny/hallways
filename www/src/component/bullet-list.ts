import { node } from "@lonnycorp/htmlforge"

export class BulletList implements node.Buildable {

    private readonly root: node.Element

    constructor() {
        this.root = new node.Element("ul")
            .style("display", "flex")
            .style("flex-direction", "column")
            .style("gap", "8px")
            .style("line-height", "1.35")
            .style("margin", "0")
            .style("padding-left", "22px")
    }

    child(child: node.Buildable) {
        this.root.child(
            new node.Element("li")
                .child(child)
        )

        return this
    }

    build() {
        return this.root.build()
    }

}
