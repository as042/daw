use std::f64::consts::TAU;

use crate::prelude::{Wave, fade::*};
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

    /// Adds note1 to the existing data.
    pub fn add_note1(&mut self, wave: Wave, channels: Channels, offset: f64, duration: f64) {
        let note_duration = duration - 0.03;
        let mut vec = self.new_sawtooth_wav(wave, note_duration);
        self.low_pass(&mut vec, wave.freq * 3.1);
        RawSamples::fade(&mut vec, vec![Fade::new(0.0, 0.01, FadeType::Power(2.0), false, 44100), Fade::new(note_duration - 0.01, 0.01, FadeType::Power(2.0), true, 44100)]);
        self.add(vec, channels, offset);
        self.add_const(0.0, channels, offset + duration - 0.01, 0.01)
    }
}