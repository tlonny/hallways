use crate::hallways::settings::Settings;
use url::Url;

#[derive(strum::EnumIter, strum::EnumCount, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    LevelUrl,
    Visit,
    GoBack,
}

impl Item {
    pub fn name(&self) -> &'static str {
        return match self {
            Item::LevelUrl => "LEVEL URL",
            Item::Visit => "VISIT",
            Item::GoBack => "BACK",
        };
    }
}

pub struct Visit {
    pub hovered: usize,
    pub selected: bool,
    level_url: String,
    level_url_value: Option<Url>,
}

pub struct VisitLevelUrl<'a> {
    pub text: &'a str,
    pub value: Option<&'a Url>,
}

impl Visit {
    pub fn new(settings: &Settings) -> Self {
        return Self {
            hovered: 0,
            selected: false,
            level_url: settings.default_url.to_string(),
            level_url_value: Some(settings.default_url.clone()),
        };
    }

    pub fn level_url(&self) -> VisitLevelUrl<'_> {
        return VisitLevelUrl {
            text: &self.level_url,
            value: self.level_url_value.as_ref(),
        };
    }

    pub fn level_url_pop(&mut self) {
        self.level_url.pop();
        self.level_url_value_refresh();
    }

    pub fn level_url_push(&mut self, c: char) {
        self.level_url.push(c);
        self.level_url_value_refresh();
    }

    pub fn clear(&mut self) {
        self.hovered = 0;
        self.selected = false;
    }

    fn level_url_value_refresh(&mut self) {
        self.level_url_value = Url::parse(&self.level_url).ok();
    }
}
