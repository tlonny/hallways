import { BORDER_COLOR, TEXT_COLOR } from "@src/constant"
import { node } from "@lonnycorp/htmlforge"

export class Table implements node.Buildable {

    private readonly body = new node.Element("tbody")
    private readonly root: node.Element

    constructor(headers: string[]) {
        this.root = new node.Element("div")
            .style("overflow-x", "auto")
            .style("width", "100%")
            .child(
                new node.Element("table")
                    .style("border-collapse", "collapse")
                    .style("color", TEXT_COLOR)
                    .style("font-size", "14px")
                    .style("line-height", "1.45")
                    .style("width", "100%")
                    .child(createHeader(headers))
                    .child(this.body)
            )
    }

    row(cells: node.Buildable[]) {
        const row = new node.Element("tr")
        for (const cell of cells) {
            row.child(createBodyCell(cell))
        }

        this.body.child(row)

        return this
    }

    build() {
        return this.root.build()
    }

}

function createHeader(headers: string[]): node.Element {
    const row = new node.Element("tr")
    for (const header of headers) {
        row.child(
            new node.Element("th")
                .style("background", "#e3d2ac")
                .style("border", `1px solid ${BORDER_COLOR}`)
                .style("font-weight", "700")
                .style("padding", "8px")
                .style("text-align", "left")
                .child(new node.Text(header))
        )
    }

    return new node.Element("thead")
        .child(row)
}

function createBodyCell(cell: node.Buildable): node.Element {
    return new node.Element("td")
        .style("border", `1px solid ${BORDER_COLOR}`)
        .style("padding", "8px")
        .style("vertical-align", "top")
        .child(cell)
}
