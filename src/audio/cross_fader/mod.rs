use rodio::dynamic_mixer::{DynamicMixer, DynamicMixerController};

mod silence;

use self::silence::Silence;
use super::data::Data;
use super::speaker::Speaker;

const RAMP_SPEED: f32 = 0.01;
const MIXER_CHANNELS: u16 = 2;
const MIXER_SAMPLE_RATE: u32 = 48_000;

pub struct CrossFader {
    mixer: std::sync::Arc<DynamicMixerController<f32>>,
    source: Option<DynamicMixer<f32>>,
    _silence: Silence,
    incoming_speaker: Option<Speaker>,
    incoming_volume: f32,
    outgoing_speaker: Option<Speaker>,
    outgoing_volume: f32,
}

impl CrossFader {
    pub fn create() -> Self {
        let (local_mixer, local_source) =
            rodio::dynamic_mixer::mixer::<f32>(MIXER_CHANNELS, MIXER_SAMPLE_RATE);
        let silence = Silence::create(MIXER_CHANNELS, MIXER_SAMPLE_RATE);
        local_mixer.add(silence.source());

        return Self {
            mixer: local_mixer,
            source: Some(local_source),
            _silence: silence,
            incoming_speaker: None,
            incoming_volume: 0.0,
            outgoing_speaker: None,
            outgoing_volume: 0.0,
        };
    }

    pub fn source(&mut self) -> DynamicMixer<f32> {
        return self.source.take().unwrap();
    }

    pub fn fade_in(&mut self, data: Data) {
        self.outgoing_speaker = self.incoming_speaker.take();
        self.outgoing_volume = self.incoming_volume;

        let speaker = Speaker::create(data);
        speaker.volume_set(0.0);
        speaker.play();
        self.mixer.add(speaker.source());

        self.incoming_speaker = Some(speaker);
        self.incoming_volume = 0.0;
    }

    pub fn fade_out(&mut self) {
        self.outgoing_speaker = self.incoming_speaker.take();
        self.outgoing_volume = self.incoming_volume;
        self.incoming_volume = 0.0;
    }

    pub fn update(&mut self) {
        if let Some(speaker) = self.incoming_speaker.as_ref() {
            let delta = (1.0 - self.incoming_volume).clamp(-RAMP_SPEED, RAMP_SPEED);
            self.incoming_volume += delta;
            speaker.volume_set(self.incoming_volume);
        }

        if let Some(speaker) = self.outgoing_speaker.as_ref() {
            let delta = (0.0 - self.outgoing_volume).clamp(-RAMP_SPEED, RAMP_SPEED);
            self.outgoing_volume += delta;
            speaker.volume_set(self.outgoing_volume);
        }

        if self.outgoing_volume <= 0.0 {
            self.outgoing_speaker = None;
        }
    }
}
