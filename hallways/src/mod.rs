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

static AUDIO: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/hallways/audio");
static SHADER: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/hallways/shader");
static TEXTURE: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/hallways/texture");

const WINDOW_TITLE: &str = "Hallways";

pub fn run() {
    App::new().run(WINDOW_TITLE);
    return;
}
