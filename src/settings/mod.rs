mod action;
mod vsync;

use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use strum::{EnumCount, IntoEnumIterator};
use url::Url;
use winit::keyboard::KeyCode;

pub use action::Action;
pub use vsync::VsyncStatus;

const SETTINGS_PATH: &str = "hallways/settings.json";
const DEFAULT_URL: &str = "https://tlonny.github.io/hallways-nostalgia/hangar.json";

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub volume: f32,
    pub mouse_sensitivity: f32,
    pub default_url: Url,
    pub vsync_status: VsyncStatus,
    keys: [KeyCode; Action::COUNT],
}

fn settings_path() -> PathBuf {
    let dir = dirs::config_local_dir().unwrap();
    return dir.join(SETTINGS_PATH);
}

impl Settings {
    pub fn load() -> Self {
        let path = settings_path();
        let mut settings = fs::read_to_string(&path)
            .ok()
            .and_then(|data| serde_json::from_str::<Self>(&data).ok())
            .unwrap_or_else(Self::new);
        settings.volume = settings.volume.clamp(0.0, 1.0);
        return settings;
    }

    pub fn save(&self) {
        let path = settings_path();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, serde_json::to_string(self).unwrap()).unwrap();
    }

    pub fn new() -> Self {
        let keys: [KeyCode; Action::COUNT] = Action::iter()
            .map(|action| action.key_default())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        return Self {
            volume: 1.0,
            mouse_sensitivity: 1.0,
            default_url: Url::parse(DEFAULT_URL).unwrap(),
            vsync_status: VsyncStatus::Enabled,
            keys,
        };
    }

    pub fn key(&self, action: Action) -> &KeyCode {
        return &self.keys[action as usize];
    }

    pub fn set_key(&mut self, action: Action, key: KeyCode) {
        self.keys[action as usize] = key;
    }
}
