use glam::Vec2;

use crate::hallways::sprite::{Quad, TextureKind};
use crate::hallways::util;

const UV_POSITION: Vec2 = Vec2::new(32.0, 0.0);
const UV_SIZE: Vec2 = Vec2::splat(16.0);

pub struct Solid {
    color: util::Color,
}

impl Solid {
    pub fn new(color: util::Color) -> Self {
        return Self { color };
    }

    pub fn quad(&self, position: Vec2, size: Vec2) -> Quad {
        return Quad::new(
            UV_POSITION,
            UV_SIZE,
            TextureKind::System,
            position,
            size,
            self.color,
        );
    }
}
