use winit::keyboard::KeyCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, strum::EnumIter, strum::EnumCount)]
pub enum Action {
    Forward,
    Back,
    StrafeLeft,
    StrafeRight,
    Jump,
    Crouch,
}

impl Action {
    pub fn key_default(self) -> KeyCode {
        return match self {
            Action::Forward => KeyCode::KeyW,
            Action::Back => KeyCode::KeyS,
            Action::StrafeLeft => KeyCode::KeyA,
            Action::StrafeRight => KeyCode::KeyD,
            Action::Jump => KeyCode::Space,
            Action::Crouch => KeyCode::ControlLeft,
        };
    }
}
