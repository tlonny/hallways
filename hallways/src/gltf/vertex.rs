use glam::{Vec2, Vec3};

use crate::hallways::util;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub diffuse_uv: Option<Vec2>,
    pub material_ix: Option<u32>,
    pub color: Option<util::Color>,
}
