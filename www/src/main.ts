import { copyFileSync, mkdirSync, rmSync, writeFileSync } from "fs"
import { dirname, join } from "path"
import { GUIDE_PATH, HOME_PATH } from "@src/constant"
import { IMAGES, type Image } from "@src/image"
import { guide, home } from "@src/page"
import { ROOT_DIRECTORY } from "@src/root"

type Page = {
    path: string
    content: string
}

const DIST_PATH = join(ROOT_DIRECTORY, "dist", "www")

const PAGES = [
    {
        path: HOME_PATH,
        content: home(),
    },
    {
        path: GUIDE_PATH,
        content: guide(),
    },
]

const clean = () => {
    rmSync(DIST_PATH, { recursive: true, force: true })
    mkdirSync(DIST_PATH, { recursive: true })
}

const buildPage = (page: Page) => {
    const relPath = page.path.substring(1)
    const path = join(DIST_PATH, relPath)

    mkdirSync(dirname(path), { recursive: true })
    writeFileSync(path, page.content)
}

const buildImage = (image: Image) => {
    const srcPath = join(ROOT_DIRECTORY, image.srcPath)
    const dstPath = join(DIST_PATH, image.publicPath.substring(1))

    mkdirSync(dirname(dstPath), { recursive: true })
    copyFileSync(srcPath, dstPath)
}

clean()
for (const image of IMAGES) {
    buildImage(image)
}

for (const page of PAGES) {
    buildPage(page)
}
