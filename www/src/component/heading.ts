import { HIGHLIGHT_COLOR } from "@src/constant"
import { node } from "@lonnycorp/htmlforge"

export enum Kind {
    H1 = "h1",
    H2 = "h2",
    H3 = "h3",
}

type HeadingParams = {
    kind: Kind
    title: string
}

type HeadingStyle = {
    fontSize: string
    lineHeight: string
}

export class Heading implements node.Buildable {

    private readonly root: node.Element

    constructor(params: HeadingParams) {
        const style = styleFor(params.kind)

        this.root = new node.Element(params.kind)
            .style("color", HIGHLIGHT_COLOR)
            .style("font-family", "Georgia, 'Times New Roman', Times, serif")
            .style("font-size", style.fontSize)
            .style("font-weight", "700")
            .style("line-height", style.lineHeight)
            .style("margin", "0")
            .child(new node.Text(params.title))
    }

    build() {
        return this.root.build()
    }

}

function styleFor(kind: Kind): HeadingStyle {
    if (kind === Kind.H1) {
        return {
            fontSize: "38px",
            lineHeight: "1.1",
        }
    }

    if (kind === Kind.H2) {
        return {
            fontSize: "28px",
            lineHeight: "1.15",
        }
    }

    return {
        fontSize: "21px",
        lineHeight: "1.2",
    }
}
