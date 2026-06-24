mod artifact;
mod icon;
mod linux;
mod win;

use std::env;
use std::fs;
use std::path::Path;

const APP_NAME: &str = "Hallways";
const DESCRIPTION: &str = "A web browser for 3D spaces";
const RELEASE_DIST_PATH: &str = "dist/hallways";

enum ReleaseTarget {
    Linux,
    Windows,
}

fn dist_clean() {
    let dist = Path::new(RELEASE_DIST_PATH);
    if dist.exists() {
        fs::remove_dir_all(dist).unwrap();
    }
    fs::create_dir_all(dist).unwrap();
    return;
}

fn hallways_version_read() -> String {
    return env!("CARGO_PKG_VERSION").to_string();
}

fn release_target_read() -> ReleaseTarget {
    let mut args = env::args();
    let _program = args.next();
    let target = match args.next().as_deref() {
        Some("linux") => ReleaseTarget::Linux,
        Some("windows") => ReleaseTarget::Windows,
        Some(value) => panic!("unknown target: {value}"),
        None => panic!("expected target as first argument: linux|windows"),
    };

    return target;
}

fn main() {
    let target = release_target_read();
    dist_clean();

    match target {
        ReleaseTarget::Linux => {
            linux::linux_release();
        }
        ReleaseTarget::Windows => {
            win::windows_iconset_render();
            win::windows_build();
            win::windows_package();
        }
    }
    return;
}
