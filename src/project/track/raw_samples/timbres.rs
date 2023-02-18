use std::f64::consts::TAU;

use crate::prelude::Wave;
use super::{RawSamples, channels::Channels};

impl RawSamples {
    /// Adds a triangletooth wave to the existing data.
    pub fn add_triangletooth_wave(&mut self, wave: Wave, channels: Channels, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            let sample = self.sawtooth_sample(wave, k);
            self.add_sample(sample, k, channels);
            let sample = self.triangle_sample(wave, k);
            self.add_sample(sample, k, channels);
        }
    }

    /// Creates a sine-to-a-power sample from wave data.
    pub fn sin_pow_sample(&self, wave: Wave, idx: usize, pow: f64) -> f64 {
        let freq = wave.freq;
        let amp = wave.amp;
        let phase_shift = wave.amp;
        let time = idx as f64 / self.settings.sample_rate as f64;

        return if pow as i64 % 2 == 0 {
            2.0 * amp * (0.5 * TAU as f64 * freq * time + phase_shift).sin().powf(pow) - amp
        }
        else {
            amp * (TAU as f64 * freq * time + phase_shift).sin().powf(pow)
        }
    }
    /// Adds a sine-to-the-power-of-2 wave to the existing data.
    pub fn add_sin_squared_wave(&mut self, wave: Wave, channels: Channels, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            let sample = self.sin_pow_sample(wave, k, 2.0);
            self.add_sample(sample, k, channels);
        }
    }
    /// Adds a sine-to-the-power-of-3 wave to the existing data.
    pub fn add_sin_cubed_wave(&mut self, wave: Wave, channels: Channels, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            let sample = self.sin_pow_sample(wave, k, 3.0);
            self.add_sample(sample, k, channels);
        }
    }
    /// Adds a sine-to-the-power-of-4 wave to the existing data.
    pub fn add_sin_hypercubed_wave(&mut self, wave: Wave, channels: Channels, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            let sample = self.sin_pow_sample(wave, k, 4.0);
            self.add_sample(sample, k, channels);
        }
    }

    /// Adds timbre1 to the existing data.
    pub fn add_timbre1(&mut self, wave: Wave, channels: Channels, offset: f64, duration: f64) {
        let mut vec = self.new_sawtooth_wav(wave, duration);
        self.low_pass(&mut vec, wave.freq * 3.1);
        self.add(vec, channels, offset);
    }
    /// Adds timbre2 to the existing data.
    pub fn add_timbre2(&mut self, wave: Wave, channels: Channels, offset: f64, duration: f64) {
        let mut vec = self.new_sawtooth_wav(Wave { freq: wave.freq, amp: 1.0, phase_shift: wave.phase_shift }, duration);
        self.low_pass(&mut vec, wave.freq * 2.1);
        Self::pow(&mut vec, 3.0);
        Self::set_max_amp(&mut vec, wave.amp);
        self.add(vec, channels, offset);
    }
    /// Adds timbre3 to the existing data.
    pub fn add_timbre3(&mut self, wave: Wave, channels: Channels, offset: f64, duration: f64) {
        let mut vec = self.new_sin_wav(Wave { freq: wave.freq, amp: 1.0, phase_shift: wave.phase_shift }, duration);
        Self::pow(&mut vec, 7.0);
        Self::set_max_amp(&mut vec, wave.amp);
        self.add(vec, channels, offset);

            // let mut vec2 = self.new_sawtooth_wav(Wave { freq: wave.freq, amp: 1.0, phase_shift: wave.phase_shift }, duration);
            // Self::pow(&mut vec2, 9.0);

            // let mut vec3 = vec.iter().enumerate().map(|s| s.1 + vec2[s.0]).collect();

            // self.low_pass(&mut vec3, wave.freq * 3.1);
            // Self::set_max_amp(&mut vec3, wave.amp);
            // self.add(vec3, offset)
    }
    /// Adds test to the existing data.
    pub fn add_test(&mut self, wave: Wave, channels: Channels, offset: f64, duration: f64) {
        let mut vec = self.new_sawtooth_wav(Wave { freq: wave.freq, amp: 0.2, phase_shift: wave.phase_shift }, duration);
        Self::set_max_amp(&mut vec, wave.amp);
        self.add(vec, channels, offset);
    }
}