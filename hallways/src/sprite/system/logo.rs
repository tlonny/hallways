use glam::Vec2;

use crate::gpu::buffer::vertex::overlay;
use crate::sprite::{Quad, TextureKind};
use crate::util;

const UV_POSITION: Vec2 = Vec2::new(0.0, 16.0);
const UV_SIZE: Vec2 = Vec2::splat(480.0);

pub struct Logo {
    pub center: Vec2,
}

impl Logo {
    pub fn new(center: Vec2) -> Self {
        return Self { center };
    }

    pub fn write(&self, buffer: &mut Vec<overlay::Data>) {
        let position = self.center - UV_SIZE / 2.0;
        Quad::new(
            UV_POSITION,
            UV_SIZE,
            TextureKind::System,
            position,
            UV_SIZE,
            util::color::WHITE,
        )
        .write(buffer);
    }
}
