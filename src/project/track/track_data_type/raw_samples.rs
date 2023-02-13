use std::f32::consts::TAU;
pub use method_shorthands::methods::*;

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

    /// Adds the given sample to the data.
    pub fn add_sample(&mut self, sample: [u8; 8], idx: usize) {
        let idx = idx * self.settings.block_align();
        for j in self.samples.len()..(idx + 8) {
            self.samples.push(0);
        }

        let mut sample2 = [0; 8]; // prolly needs to account for offset
        for k in 0..self.settings.bytes_per_sample {
            sample2[k] = self.samples[idx];
        }

        let sum = add_samples(sample, sample2, self.settings.bytes_per_sample);
        let final_sample = f64_to_sample(sum, self.settings.bytes_per_sample);

        for j in 0..self.settings.num_channels {
            for k in (idx + j * self.settings.bytes_per_sample..idx + self.settings.bytes_per_sample + j * self.settings.bytes_per_sample).enumerate() {
                self.samples[k.1] = final_sample[k.0];
            }
        }
    }

    /// Creates a sine sample from wave data.
    pub fn sin_sample(&mut self, wave: Wave, idx: usize) -> [u8; 8] {
        let freq = wave.freq;
        let amp = wave.amp;
        let phase_shift = wave.amp;
        let time = idx as f64 / self.settings.sample_rate as f64;

        let value = amp * (TAU as f64 * freq * time + phase_shift).sin();
        let mut sample = f64_to_sample(value, self.settings.bytes_per_sample);
        for k in self.settings.bytes_per_sample..8 {
            sample[k] = 0;
        }

        sample
    }

    /// Pushes a sine wave to the data.
    pub fn push_sin_wave(&mut self, wave: Wave, duration: f64) {
        for k in 0..(duration * self.settings.sample_rate as f64) as usize {
            let sample = self.sin_sample(wave, k);
            self.push_sample(sample);
        }
    }

    /// Adds a sine wave to the existing data.
    pub fn add_sin_wave(&mut self, wave: Wave, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            let sample = self.sin_sample(wave, k);
            self.add_sample(sample, k);
        }
    }
}