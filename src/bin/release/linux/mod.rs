mod app;
mod build;
mod deb;
mod icon;

pub fn linux_release() {
    icon::linux_iconset_render();
    build::linux_build();
    app::linux_appimage_package();
    deb::linux_deb_package();
}
