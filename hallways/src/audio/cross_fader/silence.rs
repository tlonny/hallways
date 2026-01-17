use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use rodio::Source;

pub struct Silence {
    alive: Arc<AtomicBool>,
    channels: u16,
    sample_rate: u32,
}

struct SilenceSource {
    alive: Arc<AtomicBool>,
    channels: u16,
    sample_rate: u32,
}

impl Silence {
    pub fn create(channels: u16, sample_rate: u32) -> Self {
        return Self {
            alive: Arc::new(AtomicBool::new(true)),
            channels,
            sample_rate,
        };
    }

    pub fn source(&self) -> impl Source<Item = f32> {
        return SilenceSource {
            alive: Arc::clone(&self.alive),
            channels: self.channels,
            sample_rate: self.sample_rate,
        };
    }
}

impl Drop for Silence {
    fn drop(&mut self) {
        self.alive.store(false, Ordering::Release);
    }
}

impl Iterator for SilenceSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.alive.load(Ordering::Acquire) {
            return Some(0.0);
        }

        return None;
    }
}

impl Source for SilenceSource {
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
