use strum::{EnumCount, EnumIter};

#[derive(EnumCount, EnumIter, Clone, Copy)]
pub enum Item {
    Cancel,
    Exit,
}

impl Item {
    pub fn name(&self) -> &'static str {
        return match self {
            Item::Cancel => "CANCEL",
            Item::Exit => "EXIT",
        };
    }
}

pub struct Pause {
    pub selected: usize,
}

impl Pause {
    pub fn new() -> Self {
        return Self { selected: 0 };
    }

    pub fn clear(&mut self) {
        self.selected = 0;
    }
}
