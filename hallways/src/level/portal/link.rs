use glam::{Mat3, Vec3};

use super::Geometry;

pub struct Link {
    pub portal_ix: usize,
    pub src: Geometry,
    pub dst: Geometry,
}

impl Link {
    pub fn delta_yaw(&self) -> f32 {
        return self.dst.yaw - self.src.yaw;
    }

    pub fn transform_position(&self, pos: Vec3) -> Vec3 {
        let local = pos - self.src.center;
        let rot = Mat3::from_rotation_y(self.delta_yaw());
        return self.dst.center + rot * local;
    }

    pub fn transform_velocity(&self, vel: Vec3) -> Vec3 {
        let rot = Mat3::from_rotation_y(self.delta_yaw());
        return rot * vel;
    }
}
