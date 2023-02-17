use crate::prelude::RawSamples;

impl RawSamples {
    /// Sets the carrier signal to the sum of both signal.
    pub fn add_mod(carrier: &mut Vec<f64>, other_signal: &mut Vec<f64>) {
        *carrier = carrier.iter().enumerate().map(|s| s.1 + other_signal[s.0]).collect();
    }
    /// Ring modulator. Sets the carrier signal to the product of both signal.
    pub fn ring_mod(carrier: &mut Vec<f64>, other_signal: &mut Vec<f64>) {
        *carrier = carrier.iter().enumerate().map(|s| s.1 + other_signal[s.0]).collect();
    }
    // /// Pulse width modulator. Modulates the carriers signal with inverted sawtooth waves.
    // pub fn pulse_width_mod(carrier: &mut Vec<f64>, pulse_freq: f64, pulse_intensity: f64) {
    //     *carrier = carrier.iter().map(|s| s * ()).collect();
    // }
}