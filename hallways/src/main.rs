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

pub static ASSET: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/../asset");
pub static SHADER: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/shader");

const WINDOW_TITLE: &str = "Hallways";

fn main() {
    App::new().run(WINDOW_TITLE);
}
