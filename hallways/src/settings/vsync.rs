use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VsyncStatus {
    Enabled,
    Disabled,
}

impl VsyncStatus {
    pub fn present_mode(self) -> wgpu::PresentMode {
        return match self {
            VsyncStatus::Enabled => wgpu::PresentMode::Fifo,
            VsyncStatus::Disabled => wgpu::PresentMode::AutoNoVsync,
        };
    }
}
