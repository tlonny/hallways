use glam::Vec2;

use crate::hallways::gpu::buffer::vertex::overlay;
use crate::hallways::sprite::{Quad, TextureKind};
use crate::hallways::util;

const BORDER: f32 = 3.0;

const BOX_TL: Vec2 = Vec2::new(16.0, 0.0);
const BOX_T: Vec2 = Vec2::new(19.0, 0.0);
const BOX_TR: Vec2 = Vec2::new(29.0, 0.0);
const BOX_L: Vec2 = Vec2::new(16.0, 3.0);
const BOX_C: Vec2 = Vec2::new(19.0, 3.0);
const BOX_R: Vec2 = Vec2::new(29.0, 3.0);
const BOX_BL: Vec2 = Vec2::new(16.0, 13.0);
const BOX_B: Vec2 = Vec2::new(19.0, 13.0);
const BOX_BR: Vec2 = Vec2::new(29.0, 13.0);

const BOX_CORNER_SIZE: Vec2 = Vec2::new(3.0, 3.0);
const BOX_EDGE_H_SIZE: Vec2 = Vec2::new(10.0, 3.0);
const BOX_EDGE_V_SIZE: Vec2 = Vec2::new(3.0, 10.0);
const BOX_CENTER_SIZE: Vec2 = Vec2::new(10.0, 10.0);
const COLOR: util::Color = util::color::WHITE;

#[derive(Clone, Copy)]
enum Cell {
    Start,
    Middle,
    End,
}

#[derive(Clone, Copy)]
struct Mask {
    x: Cell,
    y: Cell,
}

const MASKS: [Mask; 9] = [
    Mask {
        x: Cell::Start,
        y: Cell::Start,
    },
    Mask {
        x: Cell::Middle,
        y: Cell::Start,
    },
    Mask {
        x: Cell::End,
        y: Cell::Start,
    },
    Mask {
        x: Cell::Start,
        y: Cell::Middle,
    },
    Mask {
        x: Cell::Middle,
        y: Cell::Middle,
    },
    Mask {
        x: Cell::End,
        y: Cell::Middle,
    },
    Mask {
        x: Cell::Start,
        y: Cell::End,
    },
    Mask {
        x: Cell::Middle,
        y: Cell::End,
    },
    Mask {
        x: Cell::End,
        y: Cell::End,
    },
];

pub struct Border {
    position: Vec2,
    size: Vec2,
}

impl Cell {
    fn position(&self, position: f32, size: f32) -> f32 {
        return match self {
            Cell::Start => position,
            Cell::Middle => position + BORDER,
            Cell::End => position + size - BORDER,
        };
    }

    fn size(&self, size: f32) -> f32 {
        return match self {
            Cell::Middle => size - BORDER * 2.0,
            _ => BORDER,
        };
    }
}

impl Mask {
    fn position(&self, position: Vec2, size: Vec2) -> Vec2 {
        return Vec2::new(
            self.x.position(position.x, size.x),
            self.y.position(position.y, size.y),
        );
    }

    fn size(&self, size: Vec2) -> Vec2 {
        return Vec2::new(self.x.size(size.x), self.y.size(size.y));
    }

    fn uv_position(&self) -> Vec2 {
        return match (self.x, self.y) {
            (Cell::Start, Cell::Start) => BOX_TL,
            (Cell::Middle, Cell::Start) => BOX_T,
            (Cell::End, Cell::Start) => BOX_TR,
            (Cell::Start, Cell::Middle) => BOX_L,
            (Cell::Middle, Cell::Middle) => BOX_C,
            (Cell::End, Cell::Middle) => BOX_R,
            (Cell::Start, Cell::End) => BOX_BL,
            (Cell::Middle, Cell::End) => BOX_B,
            (Cell::End, Cell::End) => BOX_BR,
        };
    }

    fn uv_size(&self) -> Vec2 {
        return match (self.x, self.y) {
            (Cell::Middle, Cell::Middle) => BOX_CENTER_SIZE,
            (Cell::Middle, _) => BOX_EDGE_H_SIZE,
            (_, Cell::Middle) => BOX_EDGE_V_SIZE,
            _ => BOX_CORNER_SIZE,
        };
    }
}

impl Border {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        return Self { position, size };
    }

    pub fn write(&self, buffer: &mut Vec<overlay::Data>) {
        let position = self.position;
        let size = self.size;

        for mask in MASKS {
            Quad::new(
                mask.uv_position(),
                mask.uv_size(),
                TextureKind::System,
                mask.position(position, size),
                mask.size(size),
                COLOR,
            )
            .write(buffer);
        }
    }
}
