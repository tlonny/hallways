use winit::window::{CursorGrabMode, Window};

use crate::game::state::scene::Kind;
use crate::game::state::Scene;

pub fn update(handle: &Window, state_scene: &Scene) {
    if !state_scene.transitioned() {
        return;
    }

    match state_scene.scene() {
        Kind::Simulation => {
            let _ = handle
                .set_cursor_grab(CursorGrabMode::Locked)
                .or_else(|_| handle.set_cursor_grab(CursorGrabMode::Confined));
            handle.set_cursor_visible(false);
        }
        _ => {
            let _ = handle.set_cursor_grab(CursorGrabMode::None);
            handle.set_cursor_visible(true);
        }
    }
}
