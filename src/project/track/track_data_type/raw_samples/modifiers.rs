use std::f64::consts::TAU;

use rustfft::{num_complex::Complex, FftPlanner, num_traits::Zero};

use crate::prelude::Wave;
use super::RawSamples;

impl RawSamples {
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

    fn fft(buffer: &mut Vec<Complex<f64>>) {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(buffer.len());

        fft.process(buffer);
    }
    fn ifft(buffer: &mut Vec<Complex<f64>>) {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_inverse(buffer.len());

        fft.process(buffer);

        *buffer = buffer.iter().map(|s| Complex::new(s.re / buffer.len() as f64, s.im)).collect();
    }

    pub fn low_pass(&mut self, data: &mut Vec<f64>, cutoff: f64) {
        let mut buffer: Vec<Complex<f64>> = data.iter().map(|s| Complex::<f64>::from(s)).collect();

        Self::fft(&mut buffer);

        let quality = 0.999;
        for i in 0..buffer.len() {
            let freq = self.settings.sample_rate as f64 * i as f64 / buffer.len() as f64;
            if freq >= cutoff {
                let factor = quality / (quality - 1.0) * (freq - cutoff) + 1.0;
                if factor < 0.0 {
                    buffer[i] = Complex::zero();
                }
                else {
                    buffer[i] *= factor;
                }
            }
        }

        Self::ifft(&mut buffer);
        
        let buffer: Vec<f64> = buffer.iter().map(|s| s.re).collect();

        *data = buffer;
    }
}