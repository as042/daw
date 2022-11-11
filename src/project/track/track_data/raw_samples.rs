use std::f32::consts::TAU;

use crate::project::{WavSettings, wave::Wave, sample_conversion::f64_to_sample, TrackType};

use super::TrackData;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RawSamples {
    samples: Vec<u8>,
    pub settings: WavSettings
}

impl RawSamples {
    pub fn samples(&self) -> &Vec<u8> {
        &self.samples
    }

    /// Pushes the given 2-byte sample to the data.
    pub fn push_sample(&mut self, sample: &[u8; 2]) {
        for k in 0..self.settings.num_channels {
            self.samples.extend_from_slice(sample);
        }
    }

    /// Calculates sin sample using given values and pushes it to self.
    pub fn push_sin_sample(&mut self, wave: Wave, idx: i64) {
        let freq = wave.freq;
        let amp = wave.amp;
        let phase_shift = wave.amp;
        let time = idx as f64 / self.settings.sample_rate as f64;

        let value = amp * (TAU as f64 * freq * time + phase_shift).sin();
        let sample = f64_to_sample(value, self.settings.bytes_per_sample);

        for k in 0..self.settings.num_channels {
            self.samples.extend_from_slice(&sample[0..self.settings.bytes_per_sample]);
        }
    }

    pub fn push_sin_wave(&mut self, wave: Wave, duration: f64) {
        for k in 0..(duration * self.settings.sample_rate as f64) as i64 {
            self.push_sin_sample(wave, k);
        }
    }
}

impl TrackData for RawSamples {
    fn get_type(&self) -> TrackType {
        TrackType::RawSamples
    }

    fn is_type(&self, track_type: TrackType) -> bool {
        if track_type == TrackType::RawSamples {
            return true;
        }

        false
    }

    fn raw_samples(&self) -> &RawSamples {
        &self
    }
    fn raw_samples_mut(&mut self) -> &mut RawSamples {
        &mut self
    }

    fn score(&self) -> &super::Score { panic!("Incorrect type."); }
    fn midi(&self) -> &super::MIDI { panic!("Incorrect type."); }
    fn score_mut(&mut self) -> &mut super::Score { panic!("Incorrect type."); }
    fn midi_mut(&mut self) -> &mut super::MIDI { panic!("Incorrect type."); }
}