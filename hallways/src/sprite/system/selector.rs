use glam::Vec2;

use crate::hallways::sprite::{Quad, TextureKind};
use crate::hallways::util;

const UV_POSITION: Vec2 = Vec2::new(0.0, 0.0);
const SIZE: Vec2 = Vec2::new(8.0, 16.0);

pub struct Selector {
    color: util::Color,
}

impl Selector {
    pub fn new(color: util::Color) -> Self {
        return Self { color };
    }

    pub fn quad(&self, position: Vec2) -> Quad {
        return Quad::new(
            UV_POSITION,
            SIZE,
            TextureKind::System,
            position,
            SIZE,
            self.color,
        );
    }
}
