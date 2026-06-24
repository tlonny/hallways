use parry3d::shape::Cylinder;

pub trait Ext {
    fn height(&self) -> f32;
}

impl Ext for Cylinder {
    fn height(&self) -> f32 {
        return self.half_height * 2.0;
    }
}
