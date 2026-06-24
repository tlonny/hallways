import { GUIDE_PATH, HOME_PATH, LINK_COLOR } from "@src/constant"
import { node } from "@lonnycorp/htmlforge"

type NavLink = {
    name: string
    href: string
}

const DISCORD_URL = "https://discord.gg/XFWV8ZbqaS"
const DOWNLOAD_URL = "https://github.com/tlonny/hallways/releases"
const NAV_LINKS: NavLink[] = [
    {
        name: "Guide",
        href: GUIDE_PATH,
    },
    {
        name: "Download",
        href: DOWNLOAD_URL,
    },
    {
        name: "Discord",
        href: DISCORD_URL,
    },
]

export class Nav implements node.Buildable {

    private readonly root: node.Element

    constructor() {
        this.root = new node.Element("nav")
            .style("box-sizing", "border-box")
            .style("display", "flex")
            .style("flex-wrap", "wrap")
            .style("font-size", "14px")
            .style("gap", "10px")
            .style("justify-content", "center")
            .style("line-height", "1.4")
            .style("margin", "0")
            .style("padding", "0")
            .style("text-align", "center")
            .style("width", "100%")
            .child(navLinksCreate())
    }

    build() {
        return this.root.build()
    }

}

function navLinksCreate(): node.Element {
    const links = new node.Element("div")
        .style("display", "flex")
        .style("flex-wrap", "wrap")
        .style("gap", "8px")
        .style("justify-content", "center")
        .child(navLinkCreate("Home", HOME_PATH))

    for (const link of NAV_LINKS) {
        links
            .child(new node.Text("|"))
            .child(navLinkCreate(link.name, link.href))
    }

    return links
}

function navLinkCreate(name: string, href: string): node.Element {
    return new node.Element("a")
        .attribute("href", href)
        .style("color", LINK_COLOR)
        .style("text-decoration", "underline")
        .child(new node.Text(name))
}
