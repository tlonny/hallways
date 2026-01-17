use glam::Vec3;
use parry3d::math::Isometry;
use parry3d::query::PointQuery;

use crate::app::SIM_STEP;
use crate::game::state::actor::Intent;
use crate::game::state::actor::Kinematics;
use crate::game::state::actor::Stance;
use crate::level::Cache;

const EPSILON: f32 = 0.001;
const GROUND_NORMAL_Y_MIN: f32 = 0.7;
const SIM_STEP_SECS: f32 = SIM_STEP.as_secs_f32();
const MAX_ITERATIONS: usize = 3;
const SKIN_THICKNESS: f32 = 0.01;
const GROUND_CHECK_DISTANCE: f32 = 0.04;
const GROUND_SNAP_DISTANCE: f32 = 0.1;
const OVERCLIP: f32 = 1.001;

fn clip_velocity(velocity: Vec3, planes: &[Vec3; MAX_ITERATIONS], plane_count: usize) -> Vec3 {
    if plane_count == 0 {
        return velocity;
    }
    if plane_count == 1 {
        let backoff = velocity.dot(planes[0]) * OVERCLIP;
        return velocity - planes[0] * backoff;
    }
    if plane_count == 2 {
        let dir = planes[0].cross(planes[1]).normalize_or_zero();
        let crease = dir * dir.dot(velocity);
        return crease * OVERCLIP;
    }
    return Vec3::ZERO;
}

pub fn slide(kinematics: &mut Kinematics, intent: &Intent, cache: &mut Cache) {
    let intent_direction = intent.direction;
    let start_stance = kinematics.stance;
    let mut velocity = kinematics.velocity;

    let primal_velocity = velocity;
    let mut remaining_time = SIM_STEP_SECS;
    let mut planes = [Vec3::ZERO; MAX_ITERATIONS];
    let mut plane_count = 0;

    for _ in 0..MAX_ITERATIONS {
        let resolved_velocity = velocity;
        if let Some(hit) = kinematics.sweep(cache, resolved_velocity, remaining_time) {
            let approach_speed = (-resolved_velocity.dot(hit.normal)).max(0.0);
            let epsilon_time = SKIN_THICKNESS / approach_speed.max(EPSILON);
            let safe_time = (hit.time - epsilon_time).max(0.0);

            let travel = resolved_velocity * safe_time;
            kinematics.position += travel;
            remaining_time -= safe_time;
            if travel.length() > EPSILON {
                plane_count = 0;
            }

            let shape_pos = Isometry::translation(
                kinematics.position.x,
                kinematics.position.y,
                kinematics.position.z,
            );
            let hit_point = parry3d::math::Point::new(hit.point.x, hit.point.y, hit.point.z);
            let collider = kinematics.stance.collider();
            let dist = collider.distance_to_point(&shape_pos, &hit_point, true);
            if dist < EPSILON {
                kinematics.position += hit.normal * EPSILON;
            }
            if resolved_velocity.dot(hit.normal) >= 0.0 {
                continue;
            }

            planes[plane_count] = hit.normal;
            plane_count += 1;
            velocity = clip_velocity(primal_velocity, &planes, plane_count);

            if velocity.dot(primal_velocity) <= 0.0 {
                velocity = Vec3::ZERO;
                break;
            }
            continue;
        }

        kinematics.position += resolved_velocity * remaining_time;
        break;
    }

    kinematics.velocity = velocity;

    match start_stance {
        Stance::Airborne { crouching } => {
            let hit = kinematics.sweep(cache, Vec3::NEG_Y, GROUND_CHECK_DISTANCE);
            if let Some(hit) = hit {
                if hit.normal.y >= GROUND_NORMAL_Y_MIN {
                    let snap_dist = (hit.time - SKIN_THICKNESS).max(0.0);
                    kinematics.position.y -= snap_dist;
                    kinematics.stance = Stance::Grounded {
                        normal: hit.normal.normalize_or_zero(),
                        crouching,
                    };
                }
            }
        }
        Stance::Grounded { crouching, .. } => {
            let hit = kinematics.sweep(cache, Vec3::NEG_Y, GROUND_SNAP_DISTANCE);
            if let Some(hit) = hit {
                if hit.normal.y >= GROUND_NORMAL_Y_MIN {
                    let snap_dist = (hit.time - SKIN_THICKNESS).max(0.0);
                    kinematics.position.y -= snap_dist;
                    kinematics.stance = Stance::Grounded {
                        normal: hit.normal.normalize_or_zero(),
                        crouching,
                    };
                } else {
                    kinematics.stance = Stance::Airborne { crouching };
                }
            } else {
                kinematics.stance = Stance::Airborne { crouching };
            }
        }
        Stance::Floating { .. } => {
            let hit = kinematics.sweep(cache, intent_direction, GROUND_CHECK_DISTANCE);
            kinematics.stance = match hit {
                Some(hit)
                    if (hit.normal.y >= GROUND_NORMAL_Y_MIN
                        || hit.normal.y <= -GROUND_NORMAL_Y_MIN) =>
                {
                    Stance::Floating {
                        normal: Some(hit.normal.normalize_or_zero()),
                    }
                }
                _ => Stance::Floating { normal: None },
            };
        }
    }
}
