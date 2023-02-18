use rustfft::{num_complex::Complex, FftPlanner, num_traits::Zero};

use super::RawSamples;

impl RawSamples {
    pub fn fade(buffer: &mut Vec<f64>, fade_out: bool) {
        *buffer = buffer.iter().enumerate().map(|s| s.1 * (fade_out as i32 as f64 - s.0 as f64 / buffer.len() as f64)).collect();
    }

    /// Takes each sample to the power given. Be careful with even powers as they tend to double the frequency and mess up the y displacement.
    pub fn pow(buffer: &mut Vec<f64>, pow: f64) {
        *buffer = buffer.iter().map(|s| s.powf(pow)).collect();
    }
    /// Scales the data so that the highest magnitude amplitude is equal to the given amplitude.
    pub fn set_max_amp(buffer: &mut Vec<f64>, amp: f64) {
        let max_amp = buffer.iter().fold(0_f64, |a, s| a.abs().max(s.abs()));
        *buffer = buffer.iter().map(|s| s * amp / max_amp).collect();
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

        *buffer = buffer.iter().map(|s| Complex::new(2.0 * s.re / buffer.len() as f64, s.im)).collect();
    }

    /// Low-pass filter. Filters out high frequencies, letting the low ones pass.
    pub fn low_pass(&mut self, data: &mut Vec<f64>, cutoff: f64) {
        let mut buffer: Vec<Complex<f64>> = data.iter().map(|s| Complex::<f64>::from(s)).collect();

        Self::fft(&mut buffer);

        for i in 0..buffer.len() {
            let freq = self.settings.sample_rate as f64 * i as f64 / buffer.len() as f64;
            if freq >= cutoff {
                buffer[i] = Complex::zero();
            }
        }

        Self::ifft(&mut buffer);
        
        let buffer: Vec<f64> = buffer.iter().map(|s| s.re).collect();

        *data = buffer;
    }
    /// High-pass filter. Filters out low frequencies, letting the high ones pass.
    pub fn high_pass(&mut self, data: &mut Vec<f64>, cutoff: f64) {
        let mut buffer: Vec<Complex<f64>> = data.iter().map(|s| Complex::<f64>::from(s)).collect();

        Self::fft(&mut buffer);

        for i in 0..buffer.len() {
            let freq = self.settings.sample_rate as f64 * i as f64 / buffer.len() as f64;
            if freq <= cutoff {
                buffer[i] = Complex::zero();
            }
        }

        Self::ifft(&mut buffer);
        
        let buffer: Vec<f64> = buffer.iter().map(|s| s.re).collect();

        *data = buffer;
    }
    /// Range-pass filter. Filters out frequencies outside the range, letting the ones inside it pass.
    pub fn range_pass(&mut self, data: &mut Vec<f64>, min: f64, max: f64) {
        let mut buffer: Vec<Complex<f64>> = data.iter().map(|s| Complex::<f64>::from(s)).collect();

        Self::fft(&mut buffer);

        for i in 0..buffer.len() {
            let freq = self.settings.sample_rate as f64 * i as f64 / buffer.len() as f64;
            if freq <= min || freq >= max {
                buffer[i] = Complex::zero();
            }
        }

        Self::ifft(&mut buffer);
        
        let buffer: Vec<f64> = buffer.iter().map(|s| s.re).collect();

        *data = buffer;
    }
}