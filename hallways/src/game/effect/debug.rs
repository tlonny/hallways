use glam::Vec2;
use winit::keyboard::KeyCode;

use crate::hallways::game::state::debug::CHARACTER_DIMENSIONS;
use crate::hallways::game::state::scene::Kind;
use crate::hallways::game::state::Debug;
use crate::hallways::game::state::Keyboard;
use crate::hallways::game::state::Scene;
use crate::hallways::gpu::buffer::vertex::overlay;
use crate::hallways::sprite::system::Solid;
use crate::hallways::sprite::text::character::TEXT_SIZE;
use crate::hallways::sprite::text::Character;
use crate::hallways::util;

const BACKGROUND_OPACITY: f32 = 0.5;
const SCREEN_PADDING: f32 = 6.0;
const BOX_WIDTH: f32 = CHARACTER_DIMENSIONS.x as f32 * TEXT_SIZE.x + SCREEN_PADDING * 2.0;
const BOX_HEIGHT: f32 = CHARACTER_DIMENSIONS.y as f32 * TEXT_SIZE.y + SCREEN_PADDING * 2.0;

fn log_level_name(level: util::log::Level) -> &'static str {
    return match level {
        util::log::Level::Debug => "debg",
        util::log::Level::Info => "info",
        util::log::Level::Error => "erro",
    };
}

fn log_level_color(level: util::log::Level) -> util::Color {
    return match level {
        util::log::Level::Debug => util::color::LIGHT_BLUE,
        util::log::Level::Info => util::color::GREEN,
        util::log::Level::Error => util::color::RED,
    };
}

fn write_characters(characters: &mut Vec<Character>, text: &str, bold: bool, color: util::Color) {
    for c in text.chars() {
        characters.push(Character::new(c, bold, color));
    }
}

fn write_log(state_debug: &mut Debug, message: util::log::Message) {
    let mut characters = Vec::new();
    write_characters(&mut characters, "[", false, util::color::WHITE);
    write_characters(
        &mut characters,
        log_level_name(message.level),
        true,
        log_level_color(message.level),
    );
    write_characters(&mut characters, "] ", false, util::color::WHITE);
    write_characters(&mut characters, &message.message, false, util::color::WHITE);
    state_debug.push(&characters);
}

pub fn update(
    buffer: &mut Vec<overlay::Data>,
    resolution: Vec2,
    state_scene: &Scene,
    keyboard: &Keyboard,
    state_debug: &mut Debug,
    log_listener: &util::log::Listener,
) {
    while let Some(message) = log_listener.get_message() {
        write_log(state_debug, message);
    }

    if !matches!(state_scene.scene(), Kind::Simulation) {
        return;
    }

    if keyboard.pressed(KeyCode::Tab, false) {
        state_debug.visible = !state_debug.visible;
    }

    if !state_debug.visible {
        return;
    }

    let box_pos = (resolution - Vec2::new(BOX_WIDTH, BOX_HEIGHT)) / 2.0;
    Solid::new(util::color::BLACK.with_alpha(BACKGROUND_OPACITY))
        .quad(box_pos, Vec2::new(BOX_WIDTH, BOX_HEIGHT))
        .write(buffer);

    let left_x = box_pos.x + SCREEN_PADDING;
    let y_start = box_pos.y + SCREEN_PADDING;

    for (line_ix, line) in state_debug.lines.iter().enumerate() {
        let y = y_start + line_ix as f32 * TEXT_SIZE.y;
        for (x, character) in line.characters.iter().enumerate() {
            let position = Vec2::new(left_x + x as f32 * TEXT_SIZE.x, y);
            character.quad(position).write(buffer);
        }
    }
}
