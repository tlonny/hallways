use glam::Vec2;

pub struct Mouse {
    delta: Vec2,
    pending_delta: Vec2,
}

impl Mouse {
    pub fn new() -> Self {
        return Self {
            delta: Vec2::ZERO,
            pending_delta: Vec2::ZERO,
        };
    }

    pub fn push_motion(&mut self, delta: Vec2) {
        self.pending_delta += delta;
    }

    pub fn update(&mut self) {
        self.delta = self.pending_delta;
        self.pending_delta = Vec2::ZERO;
    }

    pub fn delta(&self) -> Vec2 {
        return self.delta;
    }
}

impl Default for Mouse {
    fn default() -> Self {
        return Self::new();
    }
}
