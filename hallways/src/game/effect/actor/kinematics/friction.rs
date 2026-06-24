use glam::{Mat3, Vec2, Vec3};

use crate::hallways::app::SIM_STEP;
use crate::hallways::game::state::actor::Intent;
use crate::hallways::game::state::actor::Kinematics;
use crate::hallways::game::state::actor::Stance;

const SIM_STEP_SECS: f32 = SIM_STEP.as_secs_f32();

fn velocity_plane_project(vector: Vec3, normal: Vec3) -> Vec3 {
    return vector - normal * vector.dot(normal);
}

fn floating_plane_normal_get(rotation: Vec2) -> Vec3 {
    let pitch = Mat3::from_rotation_x(rotation.x);
    let yaw = Mat3::from_rotation_y(rotation.y);
    return (yaw * pitch * Vec3::Y).normalize_or_zero();
}

fn update_velocity(velocity: &mut Vec3, plane_normal: Vec3, friction: f32, planar_velocity: bool) {
    let normal_component = plane_normal * velocity.dot(plane_normal);
    let mut planar = velocity_plane_project(*velocity, plane_normal);
    let speed = planar.length();
    if speed <= 0.0 {
        if planar_velocity {
            *velocity = planar;
        } else {
            *velocity = normal_component + planar;
        }
        return;
    }

    let next_speed = (speed - friction * SIM_STEP_SECS).max(0.0);
    if next_speed <= 0.0 {
        planar = Vec3::ZERO;
        if planar_velocity {
            *velocity = planar;
        } else {
            *velocity = normal_component + planar;
        }
        return;
    }

    planar *= next_speed / speed;
    if planar_velocity {
        *velocity = planar;
    } else {
        *velocity = normal_component + planar;
    }
}

pub fn apply(kinematics: &mut Kinematics, intent: &Intent) {
    let dynamics = kinematics.stance.dynamics();

    match kinematics.stance {
        Stance::Grounded {
            normal,
            crouching: _,
        } => {
            update_velocity(&mut kinematics.velocity, normal, dynamics.friction, true);
        }
        Stance::Airborne { .. } => {
            update_velocity(&mut kinematics.velocity, Vec3::Y, dynamics.friction, false);
        }
        Stance::Floating { normal } => {
            let plane_normal = match normal {
                Some(normal) => normal,
                None => floating_plane_normal_get(intent.rotation),
            };
            update_velocity(
                &mut kinematics.velocity,
                plane_normal,
                dynamics.friction,
                true,
            );
        }
    }
}
