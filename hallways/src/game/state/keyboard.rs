use std::collections::HashSet;

use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

enum KeyboardEvent {
    Typed(char),
    Pressed(KeyCode, bool),
    Released(KeyCode),
}

pub struct Keyboard {
    held: HashSet<KeyCode>,
    pressed: HashSet<KeyCode>,
    repeated: HashSet<KeyCode>,
    released: HashSet<KeyCode>,
    last_pressed: Option<KeyCode>,
    typed: Vec<char>,
    pending: Vec<KeyboardEvent>,
}

impl Keyboard {
    pub fn new() -> Self {
        return Self {
            held: HashSet::new(),
            pressed: HashSet::new(),
            repeated: HashSet::new(),
            released: HashSet::new(),
            last_pressed: None,
            typed: Vec::new(),
            pending: Vec::new(),
        };
    }

    pub fn push(&mut self, event: &KeyEvent) {
        if matches!(event.state, ElementState::Pressed) {
            if let Some(text) = event.text.as_deref() {
                for c in text.chars().filter(|c| !c.is_control()) {
                    self.pending.push(KeyboardEvent::Typed(c));
                }
            }
        }

        let PhysicalKey::Code(key) = event.physical_key else {
            return;
        };

        match event.state {
            ElementState::Pressed => self.pending.push(KeyboardEvent::Pressed(key, event.repeat)),
            ElementState::Released => self.pending.push(KeyboardEvent::Released(key)),
        }
    }

    pub fn update(&mut self) {
        self.pressed.clear();
        self.repeated.clear();
        self.released.clear();
        self.last_pressed = None;
        self.typed.clear();

        for event in self.pending.drain(..) {
            match event {
                KeyboardEvent::Typed(c) => self.typed.push(c),
                KeyboardEvent::Pressed(key, repeated) => {
                    self.pressed.insert(key);
                    if repeated {
                        self.repeated.insert(key);
                    }
                    self.held.insert(key);
                    self.last_pressed = Some(key);
                }
                KeyboardEvent::Released(key) => {
                    self.held.remove(&key);
                    self.released.insert(key);
                }
            }
        }
    }

    pub fn held(&self, key: KeyCode) -> bool {
        return self.held.contains(&key);
    }

    pub fn pressed(&self, key: KeyCode, exclude_repeat: bool) -> bool {
        return self.pressed.contains(&key) && !(exclude_repeat && self.repeated.contains(&key));
    }

    pub fn last_pressed(&self) -> Option<KeyCode> {
        return self.last_pressed;
    }

    pub fn typed(&self) -> &[char] {
        return self.typed.as_slice();
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        return Self::new();
    }
}
