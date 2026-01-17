use strum::{EnumCount, EnumIter};

#[derive(EnumCount, EnumIter, Clone, Copy)]
pub enum Item {
    Visit,
    Settings,
    Quit,
}

impl Item {
    pub fn name(&self) -> &'static str {
        return match self {
            Item::Visit => "VISIT",
            Item::Settings => "SETTINGS",
            Item::Quit => "QUIT",
        };
    }
}

pub struct Home {
    pub selected: usize,
}

impl Home {
    pub fn new() -> Self {
        return Self { selected: 0 };
    }

    pub fn clear(&mut self) {
        self.selected = 0;
    }
}
