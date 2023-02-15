use crate::prelude::Wave;
use super::RawSamples;

impl RawSamples {
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
    pub fn add_sin_squared_wave(&mut self, wave: Wave, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            let sample = self.sin_pow_sample(wave, k, 2.0);
            self.add_sample(sample, k);
        }
    }
    /// Adds a sine-to-the-power-of-3 wave to the existing data.
    pub fn add_sin_cubed_wave(&mut self, wave: Wave, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            let sample = self.sin_pow_sample(wave, k, 3.0);
            self.add_sample(sample, k);
        }
    }
    /// Adds a sine-to-the-power-of-4 wave to the existing data.
    pub fn add_sin_hypercubed_wave(&mut self, wave: Wave, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            let sample = self.sin_pow_sample(wave, k, 4.0);
            self.add_sample(sample, k);
        }
    }

    /// Adds a test wave to the existing data.
    pub fn add_timbre1(&mut self, wave: Wave, offset: f64, duration: f64) {
        let mut vec = self.new_sawtooth_wav(wave, duration);
        self.low_pass(&mut vec, wave.freq * 3.1);
        self.add(vec, offset);
    }
}