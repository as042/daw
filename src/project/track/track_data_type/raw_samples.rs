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
        for _ in 0..self.settings.num_channels {
            self.samples.extend_from_slice(&sample[0..self.settings.bytes_per_sample]);
        }
    }
    /// Adds the given sample to the data.
    pub fn add_sample(&mut self, sample: [u8; 8], idx: usize) {
        let idx = idx * self.settings.block_align();
        for _ in self.samples.len()..(idx + 8) {
            self.samples.push(0);
        }

        let mut sample2 = [0; 8]; // prolly needs to account for offset
        for k in 0..self.settings.bytes_per_sample {
            sample2[k] = self.samples[idx + k];
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
    /// Creates a triangle sample from wave data.
    pub fn triangle_sample(&mut self, wave: Wave, idx: usize) -> [u8; 8] {
        let freq = wave.freq;
        let amp = wave.amp;
        let phase_shift = wave.amp;
        let time = idx as f64 / self.settings.sample_rate as f64;

        let value = 4.0 * amp * freq * ((time - phase_shift / freq).rem_euclid(1.0 / freq) - 1.0 / (freq * 2.0)).abs() - amp; // phase shift might be wrong
        let mut sample = f64_to_sample(value, self.settings.bytes_per_sample);
        for k in self.settings.bytes_per_sample..8 {
            sample[k] = 0;
        }

        sample
    }
    /// Creates a square sample from wave data.
    pub fn square_sample(&mut self, wave: Wave, idx: usize) -> [u8; 8] {
        let freq = wave.freq;
        let amp = wave.amp;
        let phase_shift = wave.amp;
        let time = idx as f64 / self.settings.sample_rate as f64;

        let value = amp * (TAU as f64 * freq * time + phase_shift).sin().signum();
        let mut sample = f64_to_sample(value, self.settings.bytes_per_sample);
        for k in self.settings.bytes_per_sample..8 {
            sample[k] = 0;
        }

        sample
    }
    /// Creates a sawtooth sample from wave data.
    pub fn sawtooth_sample(&mut self, wave: Wave, idx: usize) -> [u8; 8] {
        let freq = wave.freq;
        let amp = wave.amp;
        let phase_shift = wave.amp;
        let time = idx as f64 / self.settings.sample_rate as f64;

        let value = 2.0 * amp * ((time - phase_shift) * freq - f64::floor(0.5 + (time - phase_shift) * freq));
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
    /// Pushes a triangle wave to the data.
    pub fn push_triangle_wave(&mut self, wave: Wave, duration: f64) {
        for k in 0..(duration * self.settings.sample_rate as f64) as usize {
            let sample = self.triangle_sample(wave, k);
            self.push_sample(sample);
        }
    }
    /// Adds a triangle wave to the existing data.
    pub fn add_triangle_wave(&mut self, wave: Wave, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            let sample = self.triangle_sample(wave, k);
            self.add_sample(sample, k);
        }
    }
    /// Pushes a square wave to the data.
    pub fn push_square_wave(&mut self, wave: Wave, duration: f64) {
        for k in 0..(duration * self.settings.sample_rate as f64) as usize {
            let sample = self.square_sample(wave, k);
            self.push_sample(sample);
        }
    }
    /// Adds a square wave to the existing data.
    pub fn add_square_wave(&mut self, wave: Wave, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            let sample = self.square_sample(wave, k);
            self.add_sample(sample, k);
        }
    }
    /// Pushes a sawtooth wave to the data.
    pub fn push_sawtooth_wave(&mut self, wave: Wave, duration: f64) {
        for k in 0..(duration * self.settings.sample_rate as f64) as usize {
            let sample = self.sawtooth_sample(wave, k);
            self.push_sample(sample);
        }
    }
    /// Adds a sawtooth wave to the existing data.
    pub fn add_sawtooth_wave(&mut self, wave: Wave, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            let sample = self.sawtooth_sample(wave, k);
            self.add_sample(sample, k);
        }
    }


    /// Creates a sine-to-a-power sample from wave data.
    pub fn sinpow_sample(&mut self, wave: Wave, idx: usize, pow: f64) -> [u8; 8] {
        let freq = wave.freq;
        let amp = wave.amp;
        let phase_shift = wave.amp;
        let time = idx as f64 / self.settings.sample_rate as f64;

        let value;
        if pow as i64 % 2 == 0 {
            value = 2.0 * amp * (TAU as f64 * freq * time + phase_shift).sin().powf(pow) - amp;
        }
        else {
            value = amp * (TAU as f64 * freq * time + phase_shift).sin().powf(pow);
        }
        let mut sample = f64_to_sample(value, self.settings.bytes_per_sample);
        for k in self.settings.bytes_per_sample..8 {
            sample[k] = 0;
        }

        sample
    }

    /// Adds a triangletooth wave to the existing data.
    pub fn add_triangletooth_wave(&mut self, wave: Wave, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            let sample = self.sawtooth_sample(wave, k);
            self.add_sample(sample, k);
            let sample = self.triangle_sample(wave, k);
            self.add_sample(sample, k);
        }
    }
    /// Adds a sine-to-the-power-of-2 wave to the existing data.
    pub fn add_sinsquared_wave(&mut self, wave: Wave, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            let sample = self.sinpow_sample(wave, k, 2.0);
            self.add_sample(sample, k);
        }
    }
    /// Adds a sine-to-the-power-of-3 wave to the existing data.
    pub fn add_sincubed_wave(&mut self, wave: Wave, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            let sample = self.sinpow_sample(wave, k, 3.0);
            self.add_sample(sample, k);
        }
    }
    /// Adds a sine-to-the-power-of-4 wave to the existing data.
    pub fn add_sinhypercubed_wave(&mut self, wave: Wave, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            let sample = self.sinpow_sample(wave, k, 4.0);
            self.add_sample(sample, k);
        }
    }
}