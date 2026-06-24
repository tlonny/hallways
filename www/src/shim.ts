import { BACKGROUND_COLOR, TEXT_COLOR } from "@www/constant"
import { node } from "@lonnycorp/htmlforge"

const BODY_PADDING = "22px 0 0"

export const shim = (element: node.Element) => {
    element
        .style("background-color", BACKGROUND_COLOR)
        .style("box-sizing", "border-box")
        .style("color", TEXT_COLOR)
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("font-family", "Georgia, 'Times New Roman', Times, serif")
        .style("font-size", "16px")
        .style("margin", "0")
        .style("min-height", "100vh")
        .style("padding", BODY_PADDING)
}
