use std::fs;
use std::path::Path;
use std::process::Command;

use super::build::LINUX_TARGET;
use crate::artifact::Artifact;
use crate::icon::RELEASE_ICONSET_DIR;
use crate::{hallways_version_read, APP_NAME, DESCRIPTION, RELEASE_DIST_PATH};

fn executable_set(path: &Path) {
    let status = Command::new("chmod").arg("755").arg(path).status().unwrap();
    if !status.success() {
        panic!("chmod failed");
    }
}

pub fn linux_appimage_package() {
    let dist_dir = Path::new(RELEASE_DIST_PATH);
    let iconset_dir = Path::new(RELEASE_ICONSET_DIR);
    let version = hallways_version_read();

    let tar_path = dist_dir.join(Artifact::LinuxBinaryTarGz.file_name(&version));
    let status = Command::new("tar")
        .arg("czf")
        .arg(&tar_path)
        .arg("-C")
        .arg(format!("target/{LINUX_TARGET}/release"))
        .arg("hallways")
        .status()
        .unwrap();
    if !status.success() {
        panic!("tar failed");
    }

    let appdir = dist_dir.join(format!("{APP_NAME}.AppDir"));
    fs::create_dir_all(appdir.join("usr/bin")).unwrap();
    fs::create_dir_all(appdir.join("usr/share/applications")).unwrap();
    fs::create_dir_all(appdir.join("usr/share/icons/hicolor/256x256/apps")).unwrap();

    let binary_src = Path::new("target")
        .join(LINUX_TARGET)
        .join("release")
        .join("hallways");
    fs::copy(binary_src, appdir.join("usr/bin/hallways")).unwrap();

    fs::copy(
        iconset_dir.join("icon_256x256.png"),
        appdir.join("usr/share/icons/hicolor/256x256/apps/hallways.png"),
    )
    .unwrap();

    let desktop_template =
        fs::read_to_string("release/template/hallways.desktop.template").unwrap();
    let desktop_entry = desktop_template
        .replace("{APP_NAME}", APP_NAME)
        .replace("{DESCRIPTION}", DESCRIPTION);
    fs::write(
        appdir.join("usr/share/applications/hallways.desktop"),
        &desktop_entry,
    )
    .unwrap();

    let apprun = r#"#!/usr/bin/env sh
HERE="$(dirname "$(readlink -f "$0")")"
exec "$HERE/usr/bin/hallways" "$@"
"#;
    fs::write(appdir.join("AppRun"), apprun).unwrap();
    executable_set(&appdir.join("AppRun"));

    fs::write(appdir.join("hallways.desktop"), &desktop_entry).unwrap();
    fs::copy(
        iconset_dir.join("icon_256x256.png"),
        appdir.join("hallways.png"),
    )
    .unwrap();

    let appimagetool = dist_dir.join("appimagetool.AppImage");
    let status = Command::new("wget")
        .arg("-O")
        .arg(&appimagetool)
        .arg("https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage")
        .status()
        .unwrap();
    if !status.success() {
        panic!("wget failed");
    }
    executable_set(&appimagetool);

    let appimage_path = dist_dir.join(Artifact::LinuxAppImage.file_name(&version));
    let status = Command::new(&appimagetool)
        .env("ARCH", "x86_64")
        .arg(&appdir)
        .arg(&appimage_path)
        .status()
        .unwrap();
    if !status.success() {
        panic!("appimagetool failed");
    }
}
