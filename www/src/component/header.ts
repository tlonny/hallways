import { HIGHLIGHT_COLOR, MOBILE_MEDIA_QUERY } from "@www/constant"
import { Nav } from "@www/component/nav"
import { node } from "@lonnycorp/htmlforge"

type HeaderParams = {
    text: string
}

export class Header implements node.Buildable {

    private readonly root: node.Element

    constructor(params: HeaderParams) {
        this.root = new node.Element("header")
            .style("align-items", "center")
            .style("display", "flex")
            .style("justify-content", "center")
            .style("text-align", "center")
            .style("width", "100%")
            .child(
                new node.Element("div")
                    .style("align-items", "center")
                    .style("display", "flex")
                    .style("flex-direction", "column")
                    .style("gap", "8px")
                    .child(
                        new node.Element("h1")
                            .style("color", HIGHLIGHT_COLOR)
                            .style("font-family", "Georgia, 'Times New Roman', Times, serif")
                            .style("font-size", "60px")
                            .style("font-size", "38px", { mediaQuery: MOBILE_MEDIA_QUERY })
                            .style("font-weight", "400")
                            .style("letter-spacing", "0")
                            .style("line-height", "1")
                            .style("margin", "0")
                            .child(new node.Text(params.text))
                    )
                    .child(new Nav())
            )
    }

    build() {
        return this.root.build()
    }

}
