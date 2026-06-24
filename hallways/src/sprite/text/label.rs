use glam::Vec2;

use crate::hallways::gpu::buffer::vertex::overlay;
use crate::hallways::util;

use super::character::TEXT_SIZE;
use super::Character;

#[derive(Clone, Copy)]
pub enum Alignment {
    Right,
}

pub struct Label<'a> {
    position: Vec2,
    max_len: usize,
    visible_len: usize,
    color: util::Color,
    bold: bool,
    alignment: Alignment,
    text: &'a str,
}

impl<'a> Label<'a> {
    pub fn new(
        position: Vec2,
        max_len: Option<usize>,
        color: util::Color,
        bold: bool,
        alignment: Alignment,
        text: &'a str,
    ) -> Self {
        let text_len = text.chars().count();
        let max_len = max_len.unwrap_or(text_len);
        let visible_len = text_len.min(max_len);
        return Self {
            position,
            max_len,
            visible_len,
            color,
            bold,
            alignment,
            text,
        };
    }

    pub fn write(&self, buffer: &mut Vec<overlay::Data>) {
        let text = self.text;
        let position = self.position;
        let start_x = match self.alignment {
            Alignment::Right => position.x + (self.max_len - self.visible_len) as f32 * TEXT_SIZE.x,
        };
        let bold = self.bold;
        let color = self.color;
        for (i, c) in text.chars().take(self.visible_len).enumerate() {
            let char_position = Vec2::new(start_x + i as f32 * TEXT_SIZE.x, position.y);
            Character::new(c, bold, color)
                .quad(char_position)
                .write(buffer);
        }
    }
}
