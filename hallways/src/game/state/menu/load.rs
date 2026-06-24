use std::time::Instant;

use crate::hallways::sprite::text::Character;

pub struct Load {
    start_time: Instant,
    result_start_time: Option<Instant>,
    result_message: Vec<Character>,
}

impl Load {
    pub fn new() -> Self {
        return Self {
            start_time: Instant::now(),
            result_start_time: None,
            result_message: Vec::new(),
        };
    }

    pub fn clear(&mut self) {
        self.start_time = Instant::now();
        self.result_start_time = None;
        self.result_message.clear();
    }

    pub fn elapsed_seconds(&self) -> f32 {
        return self.start_time.elapsed().as_secs_f32();
    }

    pub fn started_result(&self) -> bool {
        return self.result_start_time.is_some();
    }

    pub fn start_result(&mut self, message: Vec<Character>) {
        self.result_start_time = Some(Instant::now());
        self.result_message = message;
    }

    pub fn result_elapsed_seconds(&self) -> f32 {
        return self.result_start_time.unwrap().elapsed().as_secs_f32();
    }

    pub fn result_message(&self) -> &[Character] {
        return &self.result_message;
    }
}
