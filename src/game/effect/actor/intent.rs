use std::f32::consts::TAU;
use std::time::{Duration, Instant};

use glam::{Mat3, Vec2, Vec3};

use crate::game::state::actor::Intent;
use crate::game::state::scene::Kind;
use crate::game::state::Keyboard;
use crate::game::state::Scene;
use crate::settings::{Action, Settings};

const PITCH_LIMIT: f32 = 1.53589;
const BASE_MOUSE_SENSITIVITY: f32 = 0.002;
const FLOAT_REPRESS_WINDOW: Duration = Duration::from_millis(350);

fn direction_rotate(direction: Vec3, rotation: Vec2) -> Vec3 {
    if direction == Vec3::ZERO {
        return Vec3::ZERO;
    }

    let pitch = Mat3::from_rotation_x(rotation.x);
    let yaw = Mat3::from_rotation_y(rotation.y);
    return yaw * pitch * direction.normalize();
}

fn jump_update(intent: &mut Intent, jumping: bool) {
    if !jumping {
        intent.jumping = false;
        intent.float = false;
        return;
    }

    let jump_time = Instant::now();
    intent.jumping = true;
    intent.float = intent.float_jump_time.is_some_and(|float_jump_time| {
        jump_time.duration_since(float_jump_time) <= FLOAT_REPRESS_WINDOW
    });
    intent.float_jump_time = Some(jump_time);
}

pub fn update(
    state_scene: &Scene,
    intent: &mut Intent,
    keyboard: &Keyboard,
    mouse_delta: Vec2,
    settings: &Settings,
) {
    if !matches!(state_scene.scene(), Kind::Simulation) {
        return;
    }

    let sensitivity = settings.mouse_sensitivity * BASE_MOUSE_SENSITIVITY;
    let rotation = Vec2::new(
        (intent.rotation.x - mouse_delta.y * sensitivity).clamp(-PITCH_LIMIT, PITCH_LIMIT),
        (intent.rotation.y - mouse_delta.x * sensitivity).rem_euclid(TAU),
    );
    intent.rotation = rotation;

    let jump_key = *settings.key(Action::Jump);
    jump_update(intent, keyboard.pressed(jump_key, true));

    let mut input_direction = Vec3::ZERO;
    if keyboard.held(*settings.key(Action::Forward)) {
        input_direction.z -= 1.0;
    }
    if keyboard.held(*settings.key(Action::Back)) {
        input_direction.z += 1.0;
    }
    if keyboard.held(*settings.key(Action::StrafeRight)) {
        input_direction.x += 1.0;
    }
    if keyboard.held(*settings.key(Action::StrafeLeft)) {
        input_direction.x -= 1.0;
    }
    let direction = direction_rotate(input_direction, rotation);
    intent.direction = direction;

    let crouching = keyboard.held(*settings.key(Action::Crouch));
    intent.crouching = crouching;
}
