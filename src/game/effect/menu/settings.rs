use glam::Vec2;
use strum::{EnumCount, IntoEnumIterator};
use winit::keyboard::KeyCode;

use crate::audio::Speaker;
use crate::game::state::menu;
use crate::game::state::menu::settings::Item;
use crate::game::state::scene::Kind;
use crate::game::state::Keyboard;
use crate::game::state::Scene;
use crate::gpu::buffer::vertex::overlay;
use crate::settings::Settings;
use crate::settings::VsyncStatus;
use crate::sprite::text::character::TEXT_SIZE;
use crate::sprite::text::label::Alignment;
use crate::sprite::text::option::State;
use crate::sprite::text::{Character, Input, Label, Option as TextOption};
use crate::sprite::Border;
use crate::util;
use crate::util::keycode::Ext;

pub const MAX_ITEM_NAME_LEN: usize = 14;
pub const MAX_ITEM_VALUE_LEN: usize = 48;

const BORDER: f32 = 3.0;
const TEXT_PADDING: f32 = 3.0;
const SCREEN_PADDING: f32 = 6.0;
const INSET: f32 = BORDER + TEXT_PADDING;
const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;
const ITEM_COUNT: usize = Item::COUNT;
const ROW_WIDTH: f32 = ITEM_INDENT + (MAX_ITEM_NAME_LEN + MAX_ITEM_VALUE_LEN) as f32 * TEXT_SIZE.x;
const BOX_WIDTH: f32 = ROW_WIDTH + INSET * 2.0;
const BOX_HEIGHT: f32 = ITEM_COUNT as f32 * TEXT_SIZE.y + INSET * 2.0;

pub struct UpdateParams<'a> {
    pub buffer: &'a mut Vec<overlay::Data>,
    pub state_scene: &'a mut Scene,
    pub state_menu_settings: &'a mut menu::Settings,
    pub keyboard: &'a Keyboard,
    pub settings: &'a mut Settings,
    pub surface: &'a wgpu::Surface<'static>,
    pub device: &'a wgpu::Device,
    pub surface_config: &'a mut wgpu::SurfaceConfiguration,
    pub select_speaker: &'a Speaker,
    pub move_speaker: &'a Speaker,
    pub tick: u64,
}

pub fn update(params: UpdateParams<'_>) {
    if !matches!(params.state_scene.scene(), Kind::MenuSettings) {
        return;
    }

    if params.state_scene.transitioned() {
        params.state_menu_settings.clear(params.settings);
    }

    let active_item = params
        .state_menu_settings
        .selected
        .then(|| Item::iter().nth(params.state_menu_settings.hovered))
        .flatten();

    if !params.state_menu_settings.selected && params.keyboard.pressed(KeyCode::ArrowUp, false) {
        params.state_menu_settings.hovered =
            (params.state_menu_settings.hovered + ITEM_COUNT - 1) % ITEM_COUNT;
        params.move_speaker.reset();
        params.move_speaker.play();
    } else if !params.state_menu_settings.selected
        && params.keyboard.pressed(KeyCode::ArrowDown, false)
    {
        params.state_menu_settings.hovered = (params.state_menu_settings.hovered + 1) % ITEM_COUNT;
        params.move_speaker.reset();
        params.move_speaker.play();
    } else if !params.state_menu_settings.selected
        && params.keyboard.pressed(KeyCode::Escape, false)
    {
        params.move_speaker.reset();
        params.move_speaker.play();
        params.state_scene.set_scene(Kind::MenuHome);
    } else if !params.state_menu_settings.selected && params.keyboard.pressed(KeyCode::Enter, false)
    {
        if let Some(item) = Item::iter().nth(params.state_menu_settings.hovered) {
            select_item(SelectItemParams {
                item,
                state_menu_settings: params.state_menu_settings,
                state_scene: params.state_scene,
                settings: params.settings,
                surface: params.surface,
                device: params.device,
                surface_config: params.surface_config,
                select_speaker: params.select_speaker,
            });
        }
    }

    let box_pos = Vec2::new(SCREEN_PADDING, SCREEN_PADDING);
    Border::new(box_pos, Vec2::new(BOX_WIDTH, BOX_HEIGHT)).write(params.buffer);

    let content_x = box_pos.x + INSET;
    let content_y = box_pos.y + INSET;

    for (i, item) in Item::iter().enumerate() {
        let y = content_y + i as f32 * TEXT_SIZE.y;
        let hovered = i == params.state_menu_settings.hovered;
        update_item(UpdateItemParams {
            item,
            state_menu_settings: params.state_menu_settings,
            buffer: &mut *params.buffer,
            position: Vec2::new(content_x, y),
            hovered,
            active_item,
            keyboard: params.keyboard,
            move_speaker: params.move_speaker,
            tick: params.tick,
        });
    }
}

struct SelectItemParams<'a> {
    item: Item,
    state_menu_settings: &'a mut menu::Settings,
    state_scene: &'a mut Scene,
    settings: &'a mut Settings,
    surface: &'a wgpu::Surface<'static>,
    device: &'a wgpu::Device,
    surface_config: &'a mut wgpu::SurfaceConfiguration,
    select_speaker: &'a Speaker,
}

struct UpdateItemParams<'a> {
    item: Item,
    state_menu_settings: &'a mut menu::Settings,
    buffer: &'a mut Vec<overlay::Data>,
    position: Vec2,
    hovered: bool,
    active_item: Option<Item>,
    keyboard: &'a Keyboard,
    move_speaker: &'a Speaker,
    tick: u64,
}

fn pct_render(buffer: &mut Vec<overlay::Data>, position: Vec2, text: &str, color: util::Color) {
    let width = text.len() as f32 * TEXT_SIZE.x;
    let x = position.x + MAX_ITEM_VALUE_LEN as f32 * TEXT_SIZE.x - width;
    for (j, c) in text.chars().enumerate() {
        let pos = Vec2::new(x + j as f32 * TEXT_SIZE.x, position.y);
        Character::new(c, false, color).quad(pos).write(buffer);
    }
}

fn select_item(params: SelectItemParams<'_>) {
    params.select_speaker.reset();
    params.select_speaker.play();

    match params.item {
        Item::Save => {
            if params.state_menu_settings.default_url().value.is_some() {
                params.state_menu_settings.settings_apply(params.settings);
                params.surface_config.present_mode = params.settings.vsync_status.present_mode();
                params
                    .surface
                    .configure(params.device, params.surface_config);
                params.settings.save();
                params.state_scene.set_scene(Kind::MenuHome);
            }
        }
        Item::GoBack => {
            params.state_scene.set_scene(Kind::MenuHome);
        }
        _ => {
            params.state_menu_settings.selected = true;
        }
    }
}

fn update_item(params: UpdateItemParams<'_>) {
    if matches!(params.active_item, Some(item) if item == params.item) {
        if params.keyboard.pressed(KeyCode::Escape, false)
            || params.keyboard.pressed(KeyCode::Enter, false)
        {
            params.state_menu_settings.selected = false;
            params.move_speaker.reset();
            params.move_speaker.play();
        } else {
            match params.item {
                Item::Volume => {
                    if params.keyboard.pressed(KeyCode::ArrowLeft, false) {
                        params.state_menu_settings.volume_down();
                        params.move_speaker.reset();
                        params.move_speaker.play();
                    }
                    if params.keyboard.pressed(KeyCode::ArrowRight, false) {
                        params.state_menu_settings.volume_up();
                        params.move_speaker.reset();
                        params.move_speaker.play();
                    }
                }
                Item::MouseSensitivity => {
                    if params.keyboard.pressed(KeyCode::ArrowLeft, false) {
                        params.state_menu_settings.mouse_sensitivity_down();
                        params.move_speaker.reset();
                        params.move_speaker.play();
                    }
                    if params.keyboard.pressed(KeyCode::ArrowRight, false) {
                        params.state_menu_settings.mouse_sensitivity_up();
                        params.move_speaker.reset();
                        params.move_speaker.play();
                    }
                }
                Item::Forward
                | Item::Back
                | Item::StrafeLeft
                | Item::StrafeRight
                | Item::Jump
                | Item::Crouch => {
                    if let Some(key) = params.keyboard.last_pressed() {
                        let action = params.item.action().unwrap();
                        params.state_menu_settings.set_key(action, key);
                        params.state_menu_settings.selected = false;
                        params.move_speaker.reset();
                        params.move_speaker.play();
                    }
                }
                Item::DefaultUrl => {
                    if params.keyboard.pressed(KeyCode::Backspace, false) {
                        params.state_menu_settings.default_url_pop();
                    }
                    for c in params.keyboard.typed() {
                        params.state_menu_settings.default_url_push(*c);
                    }
                }
                Item::Vsync => {
                    if params.keyboard.pressed(KeyCode::ArrowLeft, false)
                        || params.keyboard.pressed(KeyCode::ArrowRight, false)
                    {
                        params.state_menu_settings.set_vsync_status(
                            match params.state_menu_settings.vsync_status() {
                                VsyncStatus::Enabled => VsyncStatus::Disabled,
                                VsyncStatus::Disabled => VsyncStatus::Enabled,
                            },
                        );
                        params.move_speaker.reset();
                        params.move_speaker.play();
                    }
                }
                Item::Save | Item::GoBack => {}
            }
        }
    }

    let active = params.state_menu_settings.selected
        && matches!(params.active_item, Some(item) if item == params.item);
    let default_url = params.state_menu_settings.default_url();
    let option_state = if matches!(params.item, Item::Save) && default_url.value.is_none() {
        State::Disabled
    } else if active {
        State::Selected
    } else {
        State::Unselected
    };

    let option = TextOption::new(
        params.position,
        MAX_ITEM_NAME_LEN,
        params.hovered,
        option_state,
        params.item.name(),
    );
    option.write(params.buffer);

    let value_x = params.position.x + ITEM_INDENT + MAX_ITEM_NAME_LEN as f32 * TEXT_SIZE.x;
    let value_y = params.position.y;

    if let Some(text) = match params.item {
        Item::Volume => Some(params.state_menu_settings.volume_pct()),
        Item::MouseSensitivity => Some(params.state_menu_settings.mouse_sensitivity_pct()),
        _ => None,
    } {
        pct_render(
            params.buffer,
            Vec2::new(value_x, value_y),
            text,
            util::color::WHITE,
        );
    }

    if let Some(name) = match params.item {
        Item::Vsync => Some(match params.state_menu_settings.vsync_status() {
            VsyncStatus::Enabled => "ENABLED",
            VsyncStatus::Disabled => "DISABLED",
        }),
        Item::Forward
        | Item::Back
        | Item::StrafeLeft
        | Item::StrafeRight
        | Item::Jump
        | Item::Crouch => {
            let action = params.item.action().unwrap();
            Some(params.state_menu_settings.key(action).name())
        }
        _ => None,
    } {
        let position = Vec2::new(value_x, value_y);
        Label::new(
            position,
            Some(MAX_ITEM_VALUE_LEN),
            util::color::WHITE,
            false,
            Alignment::Right,
            name,
        )
        .write(params.buffer);
    }

    if let Some(input) = matches!(params.item, Item::DefaultUrl).then_some(Input::new(
        Vec2::new(value_x, value_y),
        MAX_ITEM_VALUE_LEN,
        default_url.text,
        active,
        params.tick,
    )) {
        input.write(params.buffer);
    }
}
