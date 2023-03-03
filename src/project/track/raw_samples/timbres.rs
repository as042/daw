use std::f64::consts::TAU;

use crate::prelude::{Wave, fade::*, Time};
use super::{RawSamples, channels::Channels};

impl RawSamples {
    /// Adds a triangletooth wave to the existing data.
    pub fn add_triangletooth_wave(&mut self, wave: Wave, channels: Channels, time: Time) {
        for k in (time.start * self.settings.sample_rate as f64) as usize..(time.end * self.settings.sample_rate as f64) as usize {
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
    pub fn add_sin_squared_wave(&mut self, wave: Wave, channels: Channels, time: Time) {
        for k in (time.start * self.settings.sample_rate as f64) as usize..(time.end * self.settings.sample_rate as f64) as usize {
            let sample = self.sin_pow_sample(wave, k, 2.0);
            self.add_sample(sample, k, channels);
        }
    }
    /// Adds a sine-to-the-power-of-3 wave to the existing data.
    pub fn add_sin_cubed_wave(&mut self, wave: Wave, channels: Channels, time: Time) {
        for k in (time.start * self.settings.sample_rate as f64) as usize..(time.end * self.settings.sample_rate as f64) as usize {
            let sample = self.sin_pow_sample(wave, k, 3.0);
            self.add_sample(sample, k, channels);
        }
    }
    /// Adds a sine-to-the-power-of-4 wave to the existing data.
    pub fn add_sin_hypercubed_wave(&mut self, wave: Wave, channels: Channels, time: Time) {
        for k in (time.start * self.settings.sample_rate as f64) as usize..(time.end * self.settings.sample_rate as f64) as usize {
            let sample = self.sin_pow_sample(wave, k, 4.0);
            self.add_sample(sample, k, channels);
        }
    }

    /// Adds subtractive synth to the existing data.
    pub fn add_subtractive_synth(&mut self, wave: Wave, channels: Channels, time: Time) {
        let mut vec = self.new_sawtooth_wav(wave, time.duration());
        self.low_pass(&mut vec, wave.freq * 3.1);
        self.add(vec, channels, time.start);
    }

    /// Adds note1 to the existing data.
    pub fn add_subtractive_synth_note(&mut self, wave: Wave, channels: Channels, time: Time) {
        let rest_duration = -1.0 / (100.0 * time.duration() + 20.0) + 0.05;  
        let note_duration = time.duration() - rest_duration;

        let mut vec = self.new_sawtooth_wav(wave, note_duration);
        self.low_pass(&mut vec, wave.freq * 3.1);
        RawSamples::fade_vec(&mut vec, &vec![
            Fade { fade_type: FadeType::Power(2.0), fade_out: false, time: Time::new(0.0, 0.005) },
            Fade { fade_type: FadeType::Power(2.0), fade_out: true, time: Time::new(note_duration - 0.005, 0.005) }], self.settings.sample_rate);
        
        self.add(vec, channels, time.start);
        self.add_const(0.0, channels, Time::new(time.start + note_duration, rest_duration))
    }
}