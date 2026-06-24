mod stance;
mod sweep;

use std::sync::Arc;

use glam::Vec3;
use url::Url;

pub use stance::Stance;

#[derive(Clone, Debug)]
pub struct Kinematics {
    pub position: Vec3,
    pub velocity: Vec3,
    pub stance: Stance,
    pub level_url: Option<Arc<Url>>,
}

impl Kinematics {
    pub fn new(position: Vec3) -> Self {
        let stance = Stance::Airborne { crouching: false };
        return Self {
            position,
            velocity: Vec3::ZERO,
            stance,
            level_url: None,
        };
    }

    pub fn eye_position(&self) -> Vec3 {
        let collider = self.stance.collider();
        return self.position + Vec3::Y * collider.half_height;
    }
}
