pub struct Data {
    pub ix: u32,
    pub path: &'static str,
}

#[derive(Debug, Clone, Copy, strum::EnumCount, strum::EnumIter)]
pub enum TextureKind {
    Text,
    System,
}

impl TextureKind {
    pub fn data(&self) -> Data {
        return match self {
            TextureKind::Text => Data {
                ix: 0,
                path: "text.png",
            },
            TextureKind::System => Data {
                ix: 1,
                path: "system.png",
            },
        };
    }
}
