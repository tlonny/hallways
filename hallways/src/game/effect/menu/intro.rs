use std::f32::consts::PI;

use glam::Vec2;
use winit::keyboard::KeyCode;

use crate::audio::Speaker;
use crate::game::state::menu::intro::START;
use crate::game::state::menu::Intro;
use crate::game::state::scene::Kind;
use crate::game::state::Keyboard;
use crate::game::state::Scene;
use crate::gpu::buffer::vertex::overlay;
use crate::sprite::system::Logo;
use crate::sprite::system::Solid;
use crate::sprite::text::character::TEXT_SIZE;
use crate::sprite::text::Character;
use crate::util;

const TITLE: &[u8] = b"LONNYCORP";
const TITLE_CHAR_BOUNCE_DURATION: f32 = 0.05;
const TITLE_CHAR_STAGGER_DELAY: f32 = 0.01;
const TITLE_SPACING: f32 = 2.0;
const LOGO_UV_SIZE: f32 = 480.0;
const TITLE_BOUNCE_HEIGHT: f32 = 32.0;

fn ramp(t: f32, start: f32, end: f32) -> f32 {
    return ((t - start) / (end - start)).clamp(0.0, 1.0);
}

pub fn update(
    state_scene: &mut Scene,
    state_menu_intro: &mut Intro,
    jingle_speaker: &Speaker,
    keyboard: &Keyboard,
    overlay_buffer: &mut Vec<overlay::Data>,
    sprite_resolution: Vec2,
) {
    if !matches!(state_scene.scene(), Kind::Intro) {
        return;
    }

    let progress = state_menu_intro.progress();
    if progress >= 1.0 {
        state_scene.set_scene(Kind::MenuHome);
    }

    if keyboard.pressed(KeyCode::Escape, false) || keyboard.pressed(KeyCode::Enter, false) {
        state_scene.set_scene(Kind::MenuHome);
    }

    if !state_menu_intro.jingle_played && progress >= START {
        jingle_speaker.reset();
        jingle_speaker.play();
        state_menu_intro.jingle_played = true;
    }

    Solid::new(util::color::WHITE)
        .quad(glam::Vec2::ZERO, sprite_resolution)
        .write(overlay_buffer);
    Logo::new(sprite_resolution / 2.0).write(overlay_buffer);

    let title_width = ((TITLE.len() - 1) as f32 * TITLE_SPACING + 1.0) * TEXT_SIZE.x;
    let title_x = (sprite_resolution.x - title_width) / 2.0;
    let title_y = sprite_resolution.y / 2.0 + LOGO_UV_SIZE / 2.0 + TEXT_SIZE.y;

    for (i, &c) in TITLE.iter().enumerate() {
        let char_start = START + i as f32 * TITLE_CHAR_STAGGER_DELAY;
        let char_life = ramp(
            progress,
            char_start,
            char_start + TITLE_CHAR_BOUNCE_DURATION,
        );
        let x = title_x + i as f32 * TITLE_SPACING * TEXT_SIZE.x;
        let y_offset = (char_life * PI).sin() * TITLE_BOUNCE_HEIGHT;
        let color = util::Color::from([0.0, 0.0, 0.0, char_life]);
        Character::new(c as char, true, color)
            .quad(glam::Vec2::new(x, title_y - y_offset))
            .write(overlay_buffer);
    }
}
