use strum::{EnumCount, EnumIter, IntoEnumIterator};
use url::Url;
use winit::keyboard::KeyCode;

use crate::settings;

const ADJUST_STEP: f32 = 0.1;

#[derive(EnumCount, EnumIter, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Volume,
    MouseSensitivity,
    Forward,
    Back,
    StrafeLeft,
    StrafeRight,
    Jump,
    Crouch,
    DefaultUrl,
    Vsync,
    Save,
    GoBack,
}

impl Item {
    pub fn name(&self) -> &'static str {
        return match self {
            Item::Volume => "VOLUME",
            Item::MouseSensitivity => "MOUSE SENS",
            Item::Forward => "FORWARDS",
            Item::Back => "BACKWARDS",
            Item::StrafeLeft => "STRAFE LEFT",
            Item::StrafeRight => "STRAFE RIGHT",
            Item::Jump => "JUMP",
            Item::Crouch => "CROUCH",
            Item::DefaultUrl => "DEFAULT URL",
            Item::Vsync => "VSYNC",
            Item::Save => "SAVE",
            Item::GoBack => "BACK",
        };
    }

    pub fn action(&self) -> Option<settings::Action> {
        return match self {
            Self::Forward => Some(settings::Action::Forward),
            Self::Back => Some(settings::Action::Back),
            Self::StrafeLeft => Some(settings::Action::StrafeLeft),
            Self::StrafeRight => Some(settings::Action::StrafeRight),
            Self::Jump => Some(settings::Action::Jump),
            Self::Crouch => Some(settings::Action::Crouch),
            _ => None,
        };
    }
}

pub struct Settings {
    pub hovered: usize,
    pub selected: bool,
    volume: f32,
    mouse_sensitivity: f32,
    default_url: String,
    default_url_value: Option<Url>,
    vsync_status: settings::VsyncStatus,
    keys: [KeyCode; settings::Action::COUNT],
    volume_pct: String,
    mouse_sensitivity_pct: String,
}

pub struct SettingsDefaultUrl<'a> {
    pub text: &'a str,
    pub value: Option<&'a Url>,
}

impl Settings {
    pub fn new(settings: &settings::Settings) -> Self {
        return Self {
            hovered: 0,
            selected: false,
            volume: settings.volume,
            mouse_sensitivity: settings.mouse_sensitivity,
            default_url: settings.default_url.to_string(),
            default_url_value: Some(settings.default_url.clone()),
            vsync_status: settings.vsync_status,
            keys: std::array::from_fn(|i| *settings.key(settings::Action::iter().nth(i).unwrap())),
            volume_pct: pct_create(settings.volume),
            mouse_sensitivity_pct: pct_create(settings.mouse_sensitivity),
        };
    }

    pub fn default_url(&self) -> SettingsDefaultUrl<'_> {
        return SettingsDefaultUrl {
            text: &self.default_url,
            value: self.default_url_value.as_ref(),
        };
    }

    pub fn default_url_pop(&mut self) {
        self.default_url.pop();
        self.default_url_value_refresh();
    }

    pub fn default_url_push(&mut self, c: char) {
        self.default_url.push(c);
        self.default_url_value_refresh();
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        self.volume_pct = pct_create(volume);
    }

    pub fn volume_down(&mut self) {
        self.set_volume((self.volume - ADJUST_STEP).clamp(0.0, 1.0));
    }

    pub fn volume_up(&mut self) {
        self.set_volume((self.volume + ADJUST_STEP).clamp(0.0, 1.0));
    }

    pub fn volume_pct(&self) -> &str {
        return &self.volume_pct;
    }

    fn set_mouse_sensitivity(&mut self, mouse_sensitivity: f32) {
        self.mouse_sensitivity = mouse_sensitivity;
        self.mouse_sensitivity_pct = pct_create(mouse_sensitivity);
    }

    pub fn mouse_sensitivity_down(&mut self) {
        self.set_mouse_sensitivity((self.mouse_sensitivity - ADJUST_STEP).clamp(0.0, 1.0));
    }

    pub fn mouse_sensitivity_up(&mut self) {
        self.set_mouse_sensitivity((self.mouse_sensitivity + ADJUST_STEP).clamp(0.0, 1.0));
    }

    pub fn mouse_sensitivity_pct(&self) -> &str {
        return &self.mouse_sensitivity_pct;
    }

    pub fn vsync_status(&self) -> settings::VsyncStatus {
        return self.vsync_status;
    }

    pub fn set_vsync_status(&mut self, vsync_status: settings::VsyncStatus) {
        self.vsync_status = vsync_status;
    }

    pub fn key(&self, action: settings::Action) -> &KeyCode {
        return &self.keys[action as usize];
    }

    pub fn set_key(&mut self, action: settings::Action, key: KeyCode) {
        self.keys[action as usize] = key;
    }

    pub fn settings_apply(&self, settings: &mut settings::Settings) {
        settings.volume = self.volume;
        settings.mouse_sensitivity = self.mouse_sensitivity;
        settings.default_url = self.default_url_value.as_ref().unwrap().clone();
        settings.vsync_status = self.vsync_status;
        for action in settings::Action::iter() {
            settings.set_key(action, *self.key(action));
        }
    }

    pub fn clear(&mut self, settings: &settings::Settings) {
        self.volume = settings.volume;
        self.mouse_sensitivity = settings.mouse_sensitivity;
        self.default_url = settings.default_url.to_string();
        self.default_url_value = Some(settings.default_url.clone());
        self.vsync_status = settings.vsync_status;
        self.keys =
            std::array::from_fn(|i| *settings.key(settings::Action::iter().nth(i).unwrap()));
        self.volume_pct = pct_create(settings.volume);
        self.mouse_sensitivity_pct = pct_create(settings.mouse_sensitivity);
        self.hovered = 0;
        self.selected = false;
    }

    fn default_url_value_refresh(&mut self) {
        self.default_url_value = Url::parse(&self.default_url).ok();
    }
}

fn pct_create(value: f32) -> String {
    return format!("{}%", (value * 100.0).round() as u32);
}
