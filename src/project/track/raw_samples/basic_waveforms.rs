use std::f64::consts::TAU;

use crate::prelude::{Wave, Time};
use super::{RawSamples, channels::Channels};

impl RawSamples {
    /// Creates a sine sample from wave data.
    pub fn sin_sample(&mut self, wave: Wave, idx: usize) -> f64 {
        let freq = wave.freq;
        let amp = wave.amp;
        let phase_shift = wave.amp;
        let time = idx as f64 / self.settings.sample_rate as f64;

        amp * (TAU as f64 * freq * time + phase_shift).sin()
    }
    /// Creates a triangle sample from wave data.
    pub fn triangle_sample(&mut self, wave: Wave, idx: usize) -> f64 {
        let freq = wave.freq;
        let amp = wave.amp;
        let phase_shift = wave.amp;
        let time = idx as f64 / self.settings.sample_rate as f64;

        4.0 * amp * freq * ((time - phase_shift / freq).rem_euclid(1.0 / freq) - 1.0 / (freq * 2.0)).abs() - amp // phase shift might be wrong
    }
    /// Creates a square sample from wave data.
    pub fn square_sample(&mut self, wave: Wave, idx: usize) -> f64 {
        let freq = wave.freq;
        let amp = wave.amp;
        let phase_shift = wave.amp;
        let time = idx as f64 / self.settings.sample_rate as f64;

        amp * (TAU * freq * time + phase_shift).sin().signum()
    }
    /// Creates a sawtooth sample from wave data.
    pub fn sawtooth_sample(&mut self, wave: Wave, idx: usize) -> f64 {
        let freq = wave.freq;
        let amp = wave.amp;
        let phase_shift = wave.amp;
        let time = idx as f64 / self.settings.sample_rate as f64;

        2.0 * amp * ((time - phase_shift) * freq - f64::floor(0.5 + (time - phase_shift) * freq))
    }

    /// Adds a sine wave to the existing data.
    pub fn add_sin_wave(&mut self, wave: Wave, channels: Channels, time: Time) {
        for k in (time.start * self.settings.sample_rate as f64) as usize..(time.end * self.settings.sample_rate as f64) as usize {
            let sample = self.sin_sample(wave, k);
            self.add_sample(sample, k, channels);
        }
    }
    /// Creates a sine wave buffer.
    pub fn new_sin_wav(&mut self, wave: Wave, duration: f64) -> Vec<f64> {
        let mut buffer = vec![];
        for k in 0..(duration * self.settings.sample_rate as f64) as usize {
            buffer.push(self.sin_sample(wave, k));
        }

        buffer
    }
    /// Adds a triangle wave to the existing data.
    pub fn add_triangle_wave(&mut self, wave: Wave, channels: Channels, time: Time) {
        for k in (time.start * self.settings.sample_rate as f64) as usize..(time.end * self.settings.sample_rate as f64) as usize {
            let sample = self.triangle_sample(wave, k);
            self.add_sample(sample, k, channels);
        }
    }
    /// Creates a triangle wave buffer.
    pub fn new_triangle_wav(&mut self, wave: Wave, duration: f64) -> Vec<f64> {
        let mut buffer = vec![];
        for k in 0..(duration * self.settings.sample_rate as f64) as usize {
            buffer.push(self.triangle_sample(wave, k));
        }

        buffer
    }
    /// Adds a square wave to the existing data.
    pub fn add_square_wave(&mut self, wave: Wave, channels: Channels, time: Time) {
        for k in (time.start * self.settings.sample_rate as f64) as usize..(time.end * self.settings.sample_rate as f64) as usize {
            let sample = self.square_sample(wave, k);
            self.add_sample(sample, k, channels);
        }
    }
    /// Creates a square wave buffer.
    pub fn new_square_wav(&mut self, wave: Wave, duration: f64) -> Vec<f64> {
        let mut buffer = vec![];
        for k in 0..(duration * self.settings.sample_rate as f64) as usize {
            buffer.push(self.square_sample(wave, k));
        }

        buffer
    }
    /// Adds a sawtooth wave to the existing data.
    pub fn add_sawtooth_wave(&mut self, wave: Wave, channels: Channels, time: Time) {
        for k in (time.start * self.settings.sample_rate as f64) as usize..(time.end * self.settings.sample_rate as f64) as usize {
            let sample = self.sawtooth_sample(wave, k);
            self.add_sample(sample, k, channels);
        }
    }
    /// Creates a sawtooth wave buffer.
    pub fn new_sawtooth_wav(&mut self, wave: Wave, duration: f64) -> Vec<f64> {
        let mut buffer = vec![];
        for k in 0..(duration * self.settings.sample_rate as f64) as usize {
            buffer.push(self.sawtooth_sample(wave, k));
        }

        buffer
    }
}