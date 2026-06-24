import { BORDER_COLOR } from "@src/constant"
import { node } from "@lonnycorp/htmlforge"

type CodeBlockParams = {
    code: string
    language: string
}

export class CodeBlock implements node.Buildable {

    private readonly root: node.Element

    constructor(params: CodeBlockParams) {
        this.root = new node.Element("pre")
            .style("background", "#f7edcf")
            .style("border", `1px solid ${BORDER_COLOR}`)
            .style("box-sizing", "border-box")
            .style("color", "#000")
            .style("font-family", "'Courier New', Courier, monospace")
            .style("font-size", "13px")
            .style("line-height", "1.45")
            .style("margin", "0")
            .style("overflow-x", "auto")
            .style("width", "100%")
            .child(
                new node.Element("code")
                    .attribute("class", `language-${params.language}`)
                    .style("padding", "0")
                    .child(new node.Text(params.code))
            )
    }

    build() {
        return this.root.build()
    }

}
