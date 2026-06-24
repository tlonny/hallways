// Avoid spawning a console window on Windows builds.
#![windows_subsystem = "windows"]

fn main() {
    hallways::hallways::run();
}
