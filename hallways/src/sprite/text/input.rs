use glam::Vec2;

use crate::hallways::gpu::buffer::vertex::overlay;
use crate::hallways::util;

use super::character::TEXT_SIZE;
use super::Character;

const TEXT_COLOR: util::Color = util::color::WHITE;
const BLINK_PERIOD: u64 = 30;

pub struct Input<'a> {
    position: Vec2,
    max_len: usize,
    text: &'a str,
    active: bool,
    clock: u64,
}

impl<'a> Input<'a> {
    pub fn new(position: Vec2, max_len: usize, text: &'a str, active: bool, clock: u64) -> Self {
        return Self {
            position,
            max_len,
            text,
            active,
            clock,
        };
    }

    pub fn write(&self, buffer: &mut Vec<overlay::Data>) {
        let color = TEXT_COLOR;
        let max_len_zero = self.max_len == 0;

        let visible: &str = if max_len_zero {
            ""
        } else if self.active {
            let start = self.text.len().saturating_sub(self.max_len - 1);
            &self.text[start..]
        } else {
            let end = self.text.len().min(self.max_len);
            &self.text[..end]
        };

        let start_x = self.position.x;
        let y = self.position.y;
        let mut visible_len = 0;
        for (i, c) in visible.chars().enumerate() {
            visible_len = i + 1;
            let position = Vec2::new(start_x + i as f32 * TEXT_SIZE.x, y);
            Character::new(c, false, color).quad(position).write(buffer);
        }

        let cursor_visible =
            !max_len_zero && self.active && (self.clock / BLINK_PERIOD).is_multiple_of(2);
        if cursor_visible {
            let cursor_x = start_x + visible_len as f32 * TEXT_SIZE.x;
            let cursor_pos = Vec2::new(cursor_x, y);
            Character::new('_', false, color)
                .quad(cursor_pos)
                .write(buffer);
        }
    }
}
