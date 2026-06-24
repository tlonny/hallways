use std::sync::Arc;

use glam::Vec3;
use parry3d::math::{Isometry, Vector};

use crate::audio::CrossFader;
use crate::game::state::actor::Intent;
use crate::game::state::actor::Kinematics;
use crate::game::state::actor::Stance;
use crate::level::cache::CacheEntry;
use crate::level::Cache;

pub fn teleport(
    kinematics: &mut Kinematics,
    intent: &mut Intent,
    cache: &mut Cache,
    cross_fader: &mut CrossFader,
    previous_position: Vec3,
    previous_stance: Stance,
) {
    let level_url = kinematics.level_url.as_ref().unwrap().clone();
    let Some(CacheEntry::Ready(level)) = cache.get(&level_url) else {
        return;
    };

    let previous_collider = previous_stance.collider();
    let start_eye = previous_position + Vec3::Y * previous_collider.half_height;
    let collider = kinematics.stance.collider();
    let end_eye = kinematics.eye_position();

    for src_portal in level.portals() {
        let Some(link) = src_portal.link(cache) else {
            continue;
        };

        let src_geometry = &src_portal.geometry;
        let src_normal = src_geometry.normal;
        let start_side = (start_eye - src_geometry.center).dot(src_normal);
        let end_side = (end_eye - src_geometry.center).dot(src_normal);
        let crossed = start_side * end_side <= 0.0;

        if !crossed {
            continue;
        }

        let move_delta = kinematics.position - previous_position;
        let shape_pos = Isometry::translation(
            previous_position.x,
            previous_position.y,
            previous_position.z,
        );
        let shape_vel = Vector::new(move_delta.x, move_delta.y, move_delta.z);
        let in_contact_prev = src_portal
            .sweep(&shape_pos, &shape_vel, &previous_collider, 1.0)
            .is_some();
        let in_contact_curr = src_portal
            .sweep(&shape_pos, &shape_vel, &collider, 1.0)
            .is_some();
        let in_contact = in_contact_prev || in_contact_curr;

        if in_contact {
            let target = src_portal.target.as_ref().unwrap();
            let delta_yaw = link.delta_yaw();
            let position = link.transform_position(kinematics.position);
            let velocity = link.transform_velocity(kinematics.velocity);
            let stance = match kinematics.stance {
                Stance::Grounded { normal, crouching } => Stance::Grounded {
                    normal: link.transform_velocity(normal).normalize_or_zero(),
                    crouching,
                },
                Stance::Airborne { crouching } => Stance::Airborne { crouching },
                Stance::Floating { normal } => {
                    let transformed_normal =
                        normal.map(|normal| link.transform_velocity(normal).normalize_or_zero());
                    Stance::Floating {
                        normal: transformed_normal,
                    }
                }
            };

            intent.rotation.y += delta_yaw;
            kinematics.position = position;
            kinematics.velocity = velocity;
            kinematics.stance = stance;
            kinematics.level_url = Some(Arc::new(target.url.clone()));

            let level_changed = level_url.as_str() != target.url.as_str();
            if level_changed {
                let Some(CacheEntry::Ready(dst_level)) = cache.get(&target.url) else {
                    panic!("linked level not ready")
                };
                match dst_level.track() {
                    Some(data) => cross_fader.fade_in(data.clone()),
                    None => cross_fader.fade_out(),
                }
            }
        }
    }
}
