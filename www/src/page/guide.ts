import * as component from "@www/component"
import { ROOT_DIRECTORY } from "@www/root"
import { shim } from "@www/shim"
import { readFileSync } from "fs"
import { join } from "path"
import { Document, node } from "@lonnycorp/htmlforge"

type ManifestField = {
    name: string
    detail: string
}

type MaterialField = {
    name: string
    detail: string
}

type TextureLimit = {
    size: string
    max: string
}

function createGuideSection(title: string): node.Element {
    return new node.Element("section")
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("gap", "14px")
        .style("width", "100%")
        .child(new component.Heading({ kind: component.Kind.H2, title }))
}

function createParagraph(text: string): node.Element {
    return new node.Element("p")
        .style("line-height", "1.6")
        .style("margin", "0")
        .child(new node.Text(text))
}

function createText(text: string): node.Text {
    return new node.Text(text)
}

function createManifestFieldRow(field: ManifestField): node.Buildable[] {
    return [
        createText(field.name),
        createText(field.detail),
    ]
}

function createManifestFieldTable(fields: ManifestField[]): component.Table {
    const table = new component.Table(["Field", "Use"])
    for (const field of fields) {
        table.row(createManifestFieldRow(field))
    }

    return table
}

function createMaterialFieldRow(field: MaterialField): node.Buildable[] {
    return [
        createText(field.name),
        createText(field.detail),
    ]
}

function createMaterialFieldTable(): component.Table {
    const table = new component.Table(["Field", "Use"])
    const fields: MaterialField[] = [
        { name: "frames", detail: "Texture frames for the material. Defaults to an empty list." },
        { name: "animation_speed", detail: "Animation speed for frame cycling. Defaults to 0.5." },
        { name: "color", detail: "RGBA tint as [r, g, b, a]. Defaults to white." },
        { name: "texture_addressing", detail: "Linear or Nearest. Defaults to Linear." },
    ]

    for (const field of fields) {
        table.row(createMaterialFieldRow(field))
    }

    return table
}

function createTextureLimitRow(limit: TextureLimit): node.Buildable[] {
    return [
        createText(limit.size),
        createText(limit.max),
    ]
}

function createTextureLimitTable(): component.Table {
    const table = new component.Table(["Size", "Maximum textures per level"])
    const limits: TextureLimit[] = [
        { size: "2048x2048", max: "1" },
        { size: "1024x1024", max: "4" },
        { size: "512x512", max: "8" },
        { size: "256x256", max: "32" },
        { size: "128x128", max: "64" },
        { size: "64x64", max: "256" },
    ]

    for (const limit of limits) {
        table.row(createTextureLimitRow(limit))
    }

    return table
}

function createIntroSection(): node.Element {
    return new node.Element("section")
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("gap", "14px")
        .style("width", "100%")
        .child(new component.Heading({ kind: component.Kind.H1, title: "Creating Hallways Levels" }))
        .child(createParagraph("Creating a Hallways level is very straightforward: you need Blender, a text editor, and somewhere static to host a few files. A level is a folder of assets tied together by a JSON manifest."))
        .child(createParagraph("At minimum, a level has:"))
        .child(
            new component.BulletList()
                .child(createText("A render model, usually model.glb."))
                .child(createText("A manifest JSON file."))
                .child(createText("One manifest entry for each material used by the model."))
        )
        .child(createParagraph("Most levels will also have textures, a simplified collision mesh, portal meshes, and background music."))
}

function createBlenderSection(): node.Element {
    return createGuideSection("Blender")
        .child(createParagraph("Build the visible space in Blender and export it as a .glb, usually named model.glb."))
        .child(createParagraph("Only export the data Hallways needs:"))
        .child(
            new component.BulletList()
                .child(createText("Positions."))
                .child(createText("Material assignments."))
                .child(createText("Diffuse UVs."))
                .child(createText("Vertex colors, when you need them."))
        )
        .child(createParagraph("Do not embed texture image binaries in the .glb. Reference texture files from the manifest instead."))
}

function createCollisionSection(): node.Element {
    return createGuideSection("Collision")
        .child(createParagraph("If your level has simple geometry, you can omit collider and Hallways will use model.glb for collision."))
        .child(createParagraph("For detailed scenes, export a separate collision mesh such as collider.glb. This should be a simplified version of the level that keeps only the surfaces the player should collide with. It only needs positions."))
        .child(createParagraph("Use a separate collider when visual detail would make movement snaggy, expensive, or too precise."))
}

function createManifestSection(): node.Element {
    const requiredFields: ManifestField[] = [
        { name: "_version", detail: "Must be \"coco\"." },
        { name: "model", detail: "Path to the visible .glb model." },
        { name: "materials", detail: "Map of glTF material names to Hallways material settings. Add one entry for each material used by the model." },
        { name: "portals", detail: "Map of portal names to portal definitions. It can be empty." },
    ]
    const optionalFields: ManifestField[] = [
        { name: "collider", detail: "Path to a separate collision .glb. If omitted, model is used for collision." },
        { name: "spawn", detail: "Player spawn position as [x, y, z]. If omitted, the player starts at the origin." },
        { name: "track", detail: "Background music file. Use an Ogg container with Vorbis audio." },
    ]

    return createGuideSection("Manifest")
        .child(createParagraph("The manifest tells Hallways which assets to load and how to interpret them. A level's canonical URL is the URL of its manifest."))
        .child(new component.CodeBlock({
            code: readFileSync(join(ROOT_DIRECTORY, "www", "src", "content", "manifest.example.json"), "utf8"),
            language: "json",
        }))
        .child(createParagraph("Manifest fields are strict. Unknown fields will fail to load."))
        .child(new component.Heading({ kind: component.Kind.H3, title: "Required Fields" }))
        .child(createManifestFieldTable(requiredFields))
        .child(new component.Heading({ kind: component.Kind.H3, title: "Optional Fields" }))
        .child(createManifestFieldTable(optionalFields))
}

function createMaterialsSection(): node.Element {
    return createGuideSection("Materials")
        .child(createParagraph("Each key in materials must match a material name from the glTF model. For example, if a Blender material is named Wall, configure it under \"Wall\" in the manifest."))
        .child(new component.CodeBlock({
            code: readFileSync(join(ROOT_DIRECTORY, "www", "src", "content", "material.example.json"), "utf8"),
            language: "json",
        }))
        .child(createParagraph("Material fields:"))
        .child(createMaterialFieldTable())
        .child(createParagraph("The final diffuse color is texture color * material color * vertex color."))
        .child(createParagraph("If no texture frames are specified, texture color is white. If material color is not specified, material color is white. If no vertex color is specified, vertex color is white."))
        .child(createParagraph("The final diffuse alpha follows the same multiplication:"))
        .child(
            new component.BulletList()
                .child(createText("Alpha 0.0 is discarded completely. Use this for cut-out holes and masks."))
                .child(createText("Alpha 1.0 is rendered as opaque geometry."))
                .child(createText("Alpha between 0.0 and 1.0 is rendered as transparent geometry."))
        )
}

function createTexturesSection(): node.Element {
    return createGuideSection("Textures")
        .child(createParagraph("Texture dimensions must match one of the supported square sizes."))
        .child(createTextureLimitTable())
}

function createPortalsSection(): node.Element {
    return createGuideSection("Portals")
        .child(createParagraph("Portals are separate .glb meshes referenced from the portals manifest map. Each portal entry has its own collider mesh:"))
        .child(new component.CodeBlock({
            code: `{
    "collider": "portal_a.glb",
    "target": {
        "href": "../other-level/level.json",
        "name": "portal_b"
    }
}`,
            language: "json",
        }))
        .child(createParagraph("target.href is a URL to another manifest. It can be absolute, such as https://example.com/level.json, or relative to the current level's manifest URL. target.name is the portal name in that destination manifest."))
        .child(createParagraph("Portal rules:"))
        .child(
            new component.BulletList()
                .child(createText("A level can contain up to 4 portals."))
                .child(createText("The mesh must have at least 3 vertices."))
                .child(createText("Portal geometry can be any coplanar polygon."))
                .child(createText("Portals must be either wall-aligned or floor/ceiling-aligned."))
                .child(createText("Wall portals only link to wall portals."))
                .child(createText("Floor and ceiling portals only link to floor and ceiling portals."))
                .child(createText("Linked floor and ceiling portals must have matching normals so gravity stays consistent and up/down does not invert."))
                .child(createText("Linked portals should be the same size and shape. Hallways does not enforce this, but mismatched portals can cause collision issues."))
                .child(createText("For floor and ceiling portals, Hallways uses the first indexed mesh vertex as an orientation anchor so it can rotate the player correctly when they pass through."))
        )
}

function createHostingSection(): node.Element {
    return createGuideSection("Hosting")
        .child(createParagraph("Host the manifest and assets on any static file host. GitHub Pages is a good starting point."))
        .child(createParagraph("Make sure the host serves the files directly and allows the browser to fetch them. Test by opening the manifest URL in a browser tab. If the JSON appears as text, Hallways should be able to request it too."))
        .child(createParagraph("Keep paths relative where possible. This makes levels easier to move between hosts."))
}

export function guide(): string {
    const doc = new Document()

    doc.attribute("lang", "en")
    shim(doc.body)

    doc.head.child(new component.Helmet({ title: "Create Levels | Hallways" }))

    doc.body
        .child(
            new component.Main()
                .child(
                    new component.Card()
                        .child(new component.Header({ text: "Hallways" }))
                        .child(new component.Divider())
                        .child(createIntroSection())
                        .child(createBlenderSection())
                        .child(createCollisionSection())
                        .child(createManifestSection())
                        .child(createMaterialsSection())
                        .child(createTexturesSection())
                        .child(createPortalsSection())
                        .child(createHostingSection())
                )
        )
        .child(new component.Footer())

    return doc.toString()
}
