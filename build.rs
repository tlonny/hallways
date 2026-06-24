const BUILD_WIN_ICO_PATH: &str = "BUILD_WIN_ICO_PATH";

fn main() {
    attach_windows_icon();
}

fn attach_windows_icon() {
    println!("cargo:rerun-if-env-changed={BUILD_WIN_ICO_PATH}");

    let ico_path = match std::env::var(BUILD_WIN_ICO_PATH) {
        Ok(path) => path,
        Err(_) => return,
    };

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os != "windows" {
        return;
    }

    println!("cargo:rerun-if-changed={ico_path}");
    let mut resource = winresource::WindowsResource::new();
    resource.set_icon(&ico_path);
    resource.compile().unwrap();
    return;
}
