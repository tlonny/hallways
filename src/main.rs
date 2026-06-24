// Avoid spawning a console window on Windows builds.
#![windows_subsystem = "windows"]

mod app;
mod audio;
mod game;
mod gltf;
mod gpu;
mod level;
mod settings;
mod sprite;
mod util;

use app::App;
use include_dir::include_dir;

static AUDIO: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/asset/audio");
static SHADER: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/asset/shader");
static TEXTURE: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/asset/texture");

const WINDOW_TITLE: &str = "Hallways";

fn main() {
    App::new().run(WINDOW_TITLE);
    return;
}
