#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Intro,
    MenuHome,
    MenuVisit,
    MenuLoad,
    MenuPause,
    MenuSettings,
    Simulation,
    Quit,
}

pub struct Scene {
    scene: Kind,
    transitioned: bool,
    pending_scene: Option<Kind>,
}

impl Scene {
    pub fn new() -> Self {
        return Self {
            scene: Kind::Intro,
            transitioned: false,
            pending_scene: None,
        };
    }

    pub fn scene(&self) -> Kind {
        return self.scene;
    }

    pub fn transitioned(&self) -> bool {
        return self.transitioned;
    }

    pub fn set_scene(&mut self, scene: Kind) {
        self.pending_scene = Some(scene);
    }

    pub fn advance(&mut self) {
        self.transitioned = false;
        if let Some(scene) = self.pending_scene.take() {
            self.transitioned = true;
            self.scene = scene;
        }
    }
}
