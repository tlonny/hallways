use winit::keyboard::KeyCode;

use crate::audio::Speaker;
use crate::game::state::scene::Kind;
use crate::game::state::Keyboard;
use crate::game::state::Scene;

pub fn update(state_scene: &mut Scene, keyboard: &Keyboard, move_speaker: &Speaker) {
    if !matches!(state_scene.scene(), Kind::Simulation) {
        return;
    }

    if keyboard.pressed(KeyCode::Escape, false) {
        move_speaker.reset();
        move_speaker.play();
        state_scene.set_scene(Kind::MenuPause);
    }
}
