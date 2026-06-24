use std::time::{Duration, Instant};

const DURATION_MILLIS: u64 = 8_333;
const DURATION: Duration = Duration::from_millis(DURATION_MILLIS);
pub const INTRO_DURATION_SECONDS: f32 = DURATION_MILLIS as f32 / 1_000.0;
pub const START: f32 = 0.15;

pub struct Intro {
    pub start_time: Instant,
    pub jingle_played: bool,
}

impl Intro {
    pub fn new() -> Self {
        return Self {
            start_time: Instant::now(),
            jingle_played: false,
        };
    }

    pub fn progress(&self) -> f32 {
        return (self.start_time.elapsed().as_secs_f32() / DURATION.as_secs_f32()).clamp(0.0, 1.0);
    }
}
