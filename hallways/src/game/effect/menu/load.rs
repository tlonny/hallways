use std::f32::consts::PI;
use std::sync::Arc;

use glam::{Vec2, Vec3};
use winit::keyboard::KeyCode;

use crate::audio::CrossFader;
use crate::audio::Speaker;
use crate::game::state::actor::Intent;
use crate::game::state::actor::Kinematics;
use crate::game::state::actor::Stance;
use crate::game::state::menu;
use crate::game::state::menu::intro::INTRO_DURATION_SECONDS;
use crate::game::state::scene::Kind;
use crate::game::state::Debug;
use crate::game::state::Keyboard;
use crate::game::state::Scene;
use crate::gpu::buffer::vertex::overlay;
use crate::level::cache::CacheEntry;
use crate::level::Cache;
use crate::sprite::text::character::TEXT_SIZE;
use crate::sprite::text::Character;
use crate::util;

const LOAD_RESULT_FADE_DURATION: f32 = 0.35;
const RESULT_TEXT_MAX_LEN: usize = 96;
const LOADING_TEXT: &str = "LOADING";
const LOADING_JUMP_PERIOD: f32 = 2.0;
const LOADING_CHAR_BOUNCE_DURATION: f32 = 0.05;
const LOADING_CHAR_STAGGER_DELAY: f32 = 0.01;
const LOADING_CHAR_SPACING: f32 = 2.0;
const LOADING_JUMP_HEIGHT: f32 = 32.0;

pub struct UpdateParams<'a> {
    pub buffer: &'a mut Vec<overlay::Data>,
    pub resolution: Vec2,
    pub state_menu_load: &'a mut menu::Load,
    pub state_scene: &'a mut Scene,
    pub keyboard: &'a Keyboard,
    pub kinematics: &'a mut Kinematics,
    pub intent: &'a mut Intent,
    pub state_debug: &'a mut Debug,
    pub cross_fader: &'a mut CrossFader,
    pub cache: &'a mut Cache,
    pub move_speaker: &'a Speaker,
}

struct LevelStartParams<'a> {
    level: &'a crate::level::Level,
    level_url: url::Url,
    kinematics: &'a mut Kinematics,
    intent: &'a mut Intent,
    state_debug: &'a mut Debug,
    cross_fader: &'a mut CrossFader,
    state_scene: &'a mut Scene,
}

fn ramp(t: f32, start: f32, end: f32) -> f32 {
    return ((t - start) / (end - start)).clamp(0.0, 1.0);
}

fn loading_text_write(
    buffer: &mut Vec<overlay::Data>,
    resolution: Vec2,
    state_menu_load: &menu::Load,
    opacity: f32,
) {
    let text_len = LOADING_TEXT.chars().count();
    let text_width = ((text_len - 1) as f32 * LOADING_CHAR_SPACING + 1.0) * TEXT_SIZE.x;
    let text_x = (resolution.x - text_width) / 2.0;
    let text_y = (resolution.y - TEXT_SIZE.y) / 2.0;
    let progress = state_menu_load
        .elapsed_seconds()
        .rem_euclid(LOADING_JUMP_PERIOD)
        / INTRO_DURATION_SECONDS;

    for (i, c) in LOADING_TEXT.chars().enumerate() {
        let char_start = i as f32 * LOADING_CHAR_STAGGER_DELAY;
        let jump_life = ramp(
            progress,
            char_start,
            char_start + LOADING_CHAR_BOUNCE_DURATION,
        );
        let jump_height = (jump_life * PI).sin() * LOADING_JUMP_HEIGHT;
        let position = Vec2::new(
            text_x + i as f32 * LOADING_CHAR_SPACING * TEXT_SIZE.x,
            text_y - jump_height,
        );
        Character::new(c, true, util::color::WHITE.with_alpha(opacity))
            .quad(position)
            .write(buffer);
    }
}

fn message_segment_push(message: &mut Vec<Character>, text: &str, color: util::Color, bold: bool) {
    for c in text.chars() {
        message.push(Character::new(c, bold, color));
    }
}

fn result_text_write(
    buffer: &mut Vec<overlay::Data>,
    resolution: Vec2,
    text: &[Character],
    opacity: f32,
) {
    let line_count = text.len().div_ceil(RESULT_TEXT_MAX_LEN);
    let text_height = line_count as f32 * TEXT_SIZE.y;
    let text_y = (resolution.y - text_height) / 2.0;

    for line_ix in 0..line_count {
        let line_start = line_ix * RESULT_TEXT_MAX_LEN;
        let line_end = (line_start + RESULT_TEXT_MAX_LEN).min(text.len());
        let line = &text[line_start..line_end];
        let text_width = (line_end - line_start) as f32 * TEXT_SIZE.x;
        let text_x = (resolution.x - text_width) / 2.0;
        let y = text_y + line_ix as f32 * TEXT_SIZE.y;
        for (i, character) in line.iter().enumerate() {
            let position = Vec2::new(text_x + i as f32 * TEXT_SIZE.x, y);
            character.with_alpha(opacity).quad(position).write(buffer);
        }
    }
}

fn load_text_write(
    buffer: &mut Vec<overlay::Data>,
    resolution: Vec2,
    state_menu_load: &menu::Load,
    cache_entry: Option<&CacheEntry>,
) {
    if cache_entry.is_none() {
        loading_text_write(buffer, resolution, state_menu_load, 1.0);
        return;
    }

    let elapsed = state_menu_load.result_elapsed_seconds();
    let loading_opacity = 1.0 - ramp(elapsed, 0.0, LOAD_RESULT_FADE_DURATION);
    let result_opacity = ramp(
        elapsed,
        LOAD_RESULT_FADE_DURATION,
        LOAD_RESULT_FADE_DURATION * 2.0,
    );
    loading_text_write(buffer, resolution, state_menu_load, loading_opacity);

    result_text_write(
        buffer,
        resolution,
        state_menu_load.result_message(),
        result_opacity,
    );
}

fn result_message_create(cache_entry: &CacheEntry) -> Vec<Character> {
    let mut message = Vec::new();

    match cache_entry {
        CacheEntry::Ready(_) => {
            message_segment_push(&mut message, "Load ", util::color::WHITE, false);
            message_segment_push(&mut message, "SUCCEEDED", util::color::GREEN, true);
            message_segment_push(&mut message, ". Press ", util::color::WHITE, false);
            message_segment_push(&mut message, "ENTER", util::color::LIGHT_BLUE, true);
            message_segment_push(&mut message, " to start!", util::color::WHITE, false);
        }
        CacheEntry::Failed(error) => {
            message_segment_push(&mut message, "Load ", util::color::WHITE, false);
            message_segment_push(&mut message, "FAILED", util::color::RED, true);
            message_segment_push(&mut message, ". Press ", util::color::WHITE, false);
            message_segment_push(&mut message, "ESCAPE", util::color::LIGHT_BLUE, true);
            message_segment_push(
                &mut message,
                " to go back. Error: ",
                util::color::WHITE,
                false,
            );
            message_segment_push(&mut message, &format!("{error}"), util::color::RED, false);
        }
    }

    return message;
}

fn level_start(params: LevelStartParams<'_>) {
    match params.level.track().cloned() {
        Some(data) => params.cross_fader.fade_in(data),
        None => params.cross_fader.fade_out(),
    }

    params.kinematics.position = params.level.spawn_position();
    params.kinematics.level_url = Some(Arc::new(params.level_url));
    params.kinematics.velocity = Vec3::ZERO;
    params.intent.float = false;
    params.intent.float_jump_time = None;
    params.state_debug.visible = false;
    params.kinematics.stance = Stance::Airborne { crouching: false };
    params.state_scene.set_scene(Kind::Simulation);
}

pub fn update(params: UpdateParams<'_>) {
    if !matches!(params.state_scene.scene(), Kind::MenuLoad) {
        return;
    }

    if params.state_scene.transitioned() {
        params.state_menu_load.clear();
    }

    if params.keyboard.pressed(KeyCode::Escape, false) {
        params.move_speaker.reset();
        params.move_speaker.play();
        params.state_scene.set_scene(Kind::MenuVisit);
        return;
    }

    if let Some(level_url) = params.kinematics.level_url.as_deref().cloned() {
        let cache_entry = params.cache.get(&level_url);
        if let Some(cache_entry) = cache_entry.as_ref() {
            if !params.state_menu_load.started_result() {
                params
                    .state_menu_load
                    .start_result(result_message_create(cache_entry));
            }
        }

        match &cache_entry {
            Some(CacheEntry::Ready(level)) => {
                if params.keyboard.pressed(KeyCode::Enter, false) {
                    level_start(LevelStartParams {
                        level,
                        level_url,
                        kinematics: params.kinematics,
                        intent: params.intent,
                        state_debug: params.state_debug,
                        cross_fader: params.cross_fader,
                        state_scene: params.state_scene,
                    });
                    return;
                }
            }
            Some(CacheEntry::Failed(_)) | None => {}
        }

        load_text_write(
            params.buffer,
            params.resolution,
            params.state_menu_load,
            cache_entry.as_ref(),
        );
        return;
    }

    loading_text_write(
        params.buffer,
        params.resolution,
        params.state_menu_load,
        1.0,
    );
}
