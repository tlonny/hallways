use glam::Vec3;

use crate::hallways::game::state::actor::Intent;
use crate::hallways::game::state::actor::Kinematics;
use crate::hallways::game::state::actor::Stance;
use crate::hallways::level::Cache;
use crate::hallways::util::parry3d::cylinder::Ext;

const STAND_DELTA: f32 = 0.7;

pub fn apply(
    kinematics: &mut Kinematics,
    intent: &Intent,
    cache: &mut Cache,
    previous_stance: Stance,
) {
    let stance = match kinematics.stance {
        Stance::Grounded {
            normal,
            crouching: old_crouching,
        } => {
            let clear_sweep = kinematics.sweep(cache, Vec3::Y, STAND_DELTA).is_none();
            let crouching = intent.crouching || (old_crouching && !clear_sweep);
            if intent.jumping {
                Stance::Airborne { crouching }
            } else {
                Stance::Grounded { normal, crouching }
            }
        }
        Stance::Airborne {
            crouching: old_crouching,
        } => {
            let clear_sweep = kinematics.sweep(cache, Vec3::NEG_Y, STAND_DELTA).is_none();
            let crouching = intent.crouching || (old_crouching && !clear_sweep);
            if intent.float {
                Stance::Floating { normal: None }
            } else {
                Stance::Airborne { crouching }
            }
        }
        Stance::Floating { normal } => {
            if intent.jumping {
                Stance::Airborne { crouching: true }
            } else {
                Stance::Floating { normal }
            }
        }
    };

    let old_height = previous_stance.collider().height();
    let new_height = stance.collider().height();
    let position_delta = (new_height - old_height) / 2.0;

    match stance {
        Stance::Grounded { .. } => {
            kinematics.position.y += position_delta;
        }
        Stance::Airborne { .. } | Stance::Floating { .. } => {
            kinematics.position.y -= position_delta;
        }
    }

    kinematics.stance = stance;
}
