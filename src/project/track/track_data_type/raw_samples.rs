use std::f32::consts::TAU;

use crate::project::{WavSettings, wave::Wave, sample_conversion::f64_to_sample, raw_sample_writer::add_samples};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RawSamples {
    samples: Vec<u8>,
    pub settings: WavSettings
}

impl RawSamples {
    pub fn samples(&self) -> &Vec<u8> {
        &self.samples
    }

    /// Pushes the given sample to the data.
    pub fn push_sample(&mut self, sample: [u8; 8]) {
        for k in 0..self.settings.num_channels {
            self.samples.extend_from_slice(&sample[0..self.settings.bytes_per_sample]);
        }
    }

    /// Pushes a calculated sine sample to the data.
    pub fn push_sin_sample(&mut self, wave: Wave, idx: usize) {
        let freq = wave.freq;
        let amp = wave.amp;
        let phase_shift = wave.amp;
        let time = idx as f64 / self.settings.sample_rate as f64;

        let value = amp * (TAU as f64 * freq * time + phase_shift).sin();
        let sample = f64_to_sample(value, self.settings.bytes_per_sample);

        self.push_sample(sample);
    }

    /// Pushes a calculated sine wave to the data.
    pub fn push_sin_wave(&mut self, wave: Wave, duration: f64) {
        for k in 0..(duration * self.settings.sample_rate as f64) as usize {
            self.push_sin_sample(wave, k);
        }
    }

    pub fn add_sample(&mut self, sample: [u8; 8], idx: usize) {
        let mut sample2 = [0; 8];
        for k in 0..8 {
            sample2[k] = self.samples[idx * self.settings.block_align() + k];
        }

        let sum = add_samples(sample, sample2, self.settings.bytes_per_sample);
    }

    pub fn add_sin_sample(&mut self, wave: Wave, idx: usize) {
        let freq = wave.freq;
        let amp = wave.amp;
        let phase_shift = wave.amp;
        let time = idx as f64 / self.settings.sample_rate as f64;

        let value = amp * (TAU as f64 * freq * time + phase_shift).sin();
        let sample1 = f64_to_sample(value, self.settings.bytes_per_sample);

        self.add_sample(sample1, idx);
    }

    pub fn add_sin_wav(&mut self, wave: Wave, offset: f64, duration: f64) {
        for k in 0..(duration * self.settings.sample_rate as f64) as usize {
            self.add_sin_sample(wave, k);
        }
    }
}