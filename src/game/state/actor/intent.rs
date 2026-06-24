use glam::{Vec2, Vec3};
use std::time::Instant;

pub struct Intent {
    pub direction: Vec3,
    pub rotation: Vec2,
    pub jumping: bool,
    pub float: bool,
    pub crouching: bool,
    pub float_jump_time: Option<Instant>,
}

impl Intent {
    pub fn new() -> Self {
        return Self {
            direction: Vec3::ZERO,
            rotation: Vec2::ZERO,
            jumping: false,
            float: false,
            crouching: false,
            float_jump_time: None,
        };
    }
}
