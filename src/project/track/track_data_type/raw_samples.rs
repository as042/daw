use std::f32::consts::TAU;

use crate::project::{WavSettings, wave::Wave};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RawSamples {
    samples: Vec<u8>,
    pub settings: WavSettings
}

impl RawSamples {
    pub fn samples(&self) -> &Vec<u8> {
        &self.samples
    }

    /// Pushes the given 2-byte sample to the data twice for stereo.
    pub fn push_sample(&mut self, sample: &[u8; 2]) {
        self.samples.extend_from_slice(sample);
        self.samples.extend_from_slice(sample);
    }

    pub fn push_sin_sample(&mut self, wave: Wave, time: f32) {
        let freq = wave.freq;
        let amp = wave.amp;
        let phase_shift = wave.amp;

        let value = amp * (TAU * freq * time + phase_shift).sin();
        let bytes = value.to_le_bytes();
    }
}