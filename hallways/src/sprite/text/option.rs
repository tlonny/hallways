use glam::Vec2;

use super::character::TEXT_SIZE;
use super::Character;
use crate::hallways::gpu::buffer::vertex::overlay;
use crate::hallways::sprite::system::Selector;
use crate::hallways::util;

const INDENT: f32 = TEXT_SIZE.x + 2.0;

pub enum State {
    Disabled,
    Unselected,
    Selected,
}

pub struct Option<'a> {
    position: Vec2,
    max_len: usize,
    hovered: bool,
    state: State,
    text: &'a str,
}

impl<'a> Option<'a> {
    pub fn new(position: Vec2, max_len: usize, hovered: bool, state: State, text: &'a str) -> Self {
        return Self {
            position,
            max_len,
            hovered,
            state,
            text,
        };
    }

    pub fn write(&self, buffer: &mut Vec<overlay::Data>) {
        let color = match self.state {
            State::Disabled => util::color::GRAY,
            State::Unselected => util::color::WHITE,
            State::Selected => util::color::CYAN,
        };

        let selector_color = match self.state {
            State::Disabled => util::color::GRAY,
            State::Unselected => util::color::WHITE,
            State::Selected => util::color::CYAN,
        };

        if self.hovered {
            Selector::new(selector_color)
                .quad(self.position)
                .write(buffer);
        }

        let len = self.text.len().min(self.max_len);
        let visible = &self.text[..len];
        let text_position = Vec2::new(self.position.x + INDENT, self.position.y);
        for (i, c) in visible.chars().enumerate() {
            let char_position =
                Vec2::new(text_position.x + i as f32 * TEXT_SIZE.x, text_position.y);
            Character::new(c, false, color)
                .quad(char_position)
                .write(buffer);
        }
    }
}
