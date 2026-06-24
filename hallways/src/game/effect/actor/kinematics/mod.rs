mod acceleration;
mod friction;
mod intent;
mod slide;
mod teleport;

use crate::hallways::audio::CrossFader;
use crate::hallways::game::state::actor::Intent;
use crate::hallways::game::state::actor::Kinematics;
use crate::hallways::game::state::scene::Kind;
use crate::hallways::game::state::Scene;
use crate::hallways::level::Cache;

pub fn update(
    state_scene: &Scene,
    kinematics: &mut Kinematics,
    intent: &mut Intent,
    cache: &mut Cache,
    cross_fader: &mut CrossFader,
) {
    if !matches!(state_scene.scene(), Kind::Simulation) {
        return;
    }

    if kinematics.level_url.is_none() {
        return;
    }

    let previous_position = kinematics.position;
    let previous_stance = kinematics.stance;

    intent::apply(kinematics, intent, cache, previous_stance);
    friction::apply(kinematics, intent);
    acceleration::apply(kinematics, intent, previous_stance);
    slide::slide(kinematics, intent, cache);
    teleport::teleport(
        kinematics,
        intent,
        cache,
        cross_fader,
        previous_position,
        previous_stance,
    );
}
