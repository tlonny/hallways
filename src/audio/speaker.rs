use std::sync::{Arc, Mutex};
use std::time::Duration;

use rodio::Source;

use super::data::Data;

struct SpeakerState {
    paused: bool,
    volume: f32,
    dropped: bool,
    generation: u64,
}

pub struct Speaker {
    data: Data,
    state: Arc<Mutex<SpeakerState>>,
}

// Rodio's mixer owns sources after they are added, so Speaker stays as the
// control handle while SpeakerSource is moved into the mixer.
struct SpeakerSource {
    state: Arc<Mutex<SpeakerState>>,
    samples: Arc<[f32]>,
    pos: usize,
    repeat: bool,
    generation: u64,
    channels: u16,
    sample_rate: u32,
}

impl Speaker {
    pub fn create(data: Data) -> Self {
        return Self {
            data,
            state: Arc::new(Mutex::new(SpeakerState {
                paused: true,
                volume: 1.0,
                dropped: false,
                generation: 0,
            })),
        };
    }

    pub fn source(&self) -> impl Source<Item = f32> {
        let generation = self.state.lock().unwrap().generation;
        return SpeakerSource {
            state: Arc::clone(&self.state),
            samples: self.data.samples(),
            pos: 0,
            repeat: self.data.repeat(),
            generation,
            channels: self.data.channels(),
            sample_rate: self.data.sample_rate(),
        };
    }

    pub fn volume_set(&self, volume: f32) {
        self.state.lock().unwrap().volume = volume;
    }

    pub fn play(&self) {
        self.state.lock().unwrap().paused = false;
    }

    pub fn reset(&self) {
        let mut state = self.state.lock().unwrap();
        state.generation = state.generation.wrapping_add(1);
        state.paused = true;
    }
}

impl Drop for Speaker {
    fn drop(&mut self) {
        self.state.lock().unwrap().dropped = true;
    }
}

impl Iterator for SpeakerSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let state = self.state.lock().unwrap();

        if state.dropped {
            return None;
        }

        if state.generation != self.generation {
            self.pos = 0;
            self.generation = state.generation;
        }

        if state.paused {
            return Some(0.0);
        }

        if self.pos >= self.samples.len() {
            if !self.repeat {
                return Some(0.0);
            }
            self.pos = 0;
        }

        let sample = self.samples[self.pos];
        self.pos += 1;
        return Some(sample * state.volume);
    }
}

impl Source for SpeakerSource {
    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn channels(&self) -> u16 {
        return self.channels;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}
