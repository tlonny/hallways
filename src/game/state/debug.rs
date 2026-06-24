use std::collections::VecDeque;

use crate::sprite::text::Character;

pub const CHARACTER_DIMENSIONS: glam::UVec2 = glam::UVec2::new(96, 32);

pub struct Debug {
    pub visible: bool,
    pub lines: VecDeque<Line>,
}

pub struct Line {
    pub characters: Vec<Character>,
}

impl Line {
    pub fn new() -> Self {
        return Self {
            characters: Vec::new(),
        };
    }
}

impl Debug {
    pub fn new() -> Self {
        return Self {
            visible: false,
            lines: VecDeque::new(),
        };
    }

    pub fn push(&mut self, characters: &[Character]) {
        let max_characters = CHARACTER_DIMENSIONS.x as usize;
        let max_lines = CHARACTER_DIMENSIONS.y as usize;
        let line_count = characters.len().max(1).div_ceil(max_characters);
        let mut character_ix = 0;

        for _ in 0..line_count {
            let mut line = Line::new();
            let character_end = (character_ix + max_characters).min(characters.len());
            for character in characters[character_ix..character_end].iter().cloned() {
                line.characters.push(character);
            }
            character_ix = character_end;
            if self.lines.len() >= max_lines {
                self.lines.pop_front();
            }
            self.lines.push_back(line);
        }
    }
}
