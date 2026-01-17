use glam::{Mat3, Vec2, Vec3};

use crate::app::SIM_STEP;
use crate::game::state::actor::Intent;
use crate::game::state::actor::Kinematics;
use crate::game::state::actor::Stance;

const SIM_STEP_SECS: f32 = SIM_STEP.as_secs_f32();
const GRAVITY: f32 = 80.0;
const JUMP_SPEED: f32 = 20.0;

fn velocity_plane_project(vector: Vec3, normal: Vec3) -> Vec3 {
    return vector - normal * vector.dot(normal);
}

fn floating_plane_normal_get(rotation: Vec2) -> Vec3 {
    let pitch = Mat3::from_rotation_x(rotation.x);
    let yaw = Mat3::from_rotation_y(rotation.y);
    return (yaw * pitch * Vec3::Y).normalize_or_zero();
}

fn jump_intended(kinematics: &Kinematics, intent: &Intent, previous_stance: Stance) -> bool {
    return intent.jumping
        && matches!(previous_stance, Stance::Grounded { .. })
        && matches!(kinematics.stance, Stance::Airborne { .. });
}

fn update_velocity(
    velocity: &mut Vec3,
    intent_direction: Vec3,
    plane_normal: Vec3,
    intent_speed: f32,
    accel: f32,
    planar_velocity: bool,
) {
    let normal_component = plane_normal * velocity.dot(plane_normal);
    let mut planar = velocity_plane_project(*velocity, plane_normal);
    let intent_direction =
        velocity_plane_project(intent_direction, plane_normal).normalize_or_zero();
    if intent_direction == Vec3::ZERO || intent_speed <= 0.0 {
        if planar_velocity {
            *velocity = planar;
        } else {
            *velocity = normal_component + planar;
        }
        return;
    }

    let intent_velocity = intent_direction * intent_speed;
    let delta = intent_velocity - planar;
    let delta_len = delta.length();
    if delta_len <= 0.0 {
        if planar_velocity {
            *velocity = planar;
        } else {
            *velocity = normal_component + planar;
        }
        return;
    }

    let max_step = accel * SIM_STEP_SECS;
    if delta_len <= max_step {
        planar = intent_velocity;
    } else {
        planar += delta / delta_len * max_step;
    }

    if planar_velocity {
        *velocity = planar;
    } else {
        *velocity = normal_component + planar;
    }
}

pub fn apply(kinematics: &mut Kinematics, intent: &Intent, previous_stance: Stance) {
    let dynamics = kinematics.stance.dynamics();
    let intent_direction = intent.direction;

    match kinematics.stance {
        Stance::Grounded {
            normal,
            crouching: _,
        } => {
            update_velocity(
                &mut kinematics.velocity,
                intent_direction,
                normal,
                dynamics.speed,
                dynamics.accel,
                true,
            );
        }
        Stance::Airborne { .. } => {
            update_velocity(
                &mut kinematics.velocity,
                intent_direction,
                Vec3::Y,
                dynamics.speed,
                dynamics.accel,
                false,
            );
            if jump_intended(kinematics, intent, previous_stance) {
                kinematics.velocity.y = JUMP_SPEED;
            }
            kinematics.velocity.y -= GRAVITY * SIM_STEP_SECS;
        }
        Stance::Floating { normal } => {
            let plane_normal = match normal {
                Some(normal) => normal,
                None => floating_plane_normal_get(intent.rotation),
            };
            update_velocity(
                &mut kinematics.velocity,
                intent_direction,
                plane_normal,
                dynamics.speed,
                dynamics.accel,
                true,
            );
        }
    }
}
