pub mod basic_waveforms;
pub mod modifiers;
pub mod timbres;
pub mod modulators;
pub mod channels;
pub mod fade;
pub mod reverb;
pub mod pitch_shift;

use std::mem::discriminant;
pub use method_shorthands::methods::*;

use crate::{project::WavSettings, prelude::{TrackData, TrackType, Time}};
use self::channels::Channels;

pub type Samples = [Vec<f64>; 8];

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RawSamples {
    samples: Samples,
    settings: WavSettings
}

impl RawSamples {
    pub fn new(settings: WavSettings) -> Self {
        RawSamples { settings: settings, ..Default::default() }
    }

    pub fn samples(&self) -> &Samples {
        &self.samples
    }
    pub fn settings(&self) -> WavSettings {
        self.settings
    }

    /// Adds the given sample to the data.
    pub fn add_sample(&mut self, sample: f64, idx: usize, channels: Channels) {
        // Pads zeros to prevent indexing out-of-bounds errors
        for _ in 0..idx as i32 + 1 - self.samples[0].len() as i32 {
            for k in 0..self.settings.num_channels {
                self.samples[k].push(0.0);
            }
        }

        for j in 0..self.settings.num_channels {
            if channels == Channels::All || 
                channels == Channels::Just(j) ||
                (discriminant(&channels) == discriminant(&Channels::AllBut(1)) && channels != Channels::AllBut(j))
            {
                let sample2 = self.samples[j][idx];
                let sum = sample + sample2;
                self.samples[j][idx] = sum;
            }
        }
    }
    // Adds the input to the data.
    pub fn add(&mut self, input: Vec<f64>, channels: Channels, offset: f64) {
        for k in 0..input.len() {
            self.add_sample(input[k], k + (offset * self.settings.sample_rate as f64) as usize, channels);
        }
    }

    #[deprecated]
    /// Pushes a constant to the data. Not recommended.
    pub fn push_const(&mut self, amp: f64, duration: f64) {
        for _ in 0..(duration * self.settings.sample_rate as f64) as usize {
            for k in 0..self.settings.num_channels {
                self.samples[k].push(amp);
            }
        }
    }
    /// Adds a constant to the existing data.
    pub fn add_const(&mut self, amp: f64, channels: Channels, time: Time) {
        for k in (time.start * self.settings.sample_rate as f64) as usize..(time.end * self.settings.sample_rate as f64) as usize {
            self.add_sample(amp, k, channels);
        }
    }
    /// Creates a constant buffer.
    pub fn new_const(&mut self, amp: f64, duration: f64) -> Vec<f64> {
        let mut buffer = vec![];
        for _ in 0..(duration * self.settings.sample_rate as f64) as usize {
            buffer.push(amp);
        }

        buffer
    }
}

impl TrackData for RawSamples {
    fn raw_samples(&self) -> &RawSamples {
        self
    }
    fn midi(&self) -> &super::MIDI {
        panic!("Incorrect type.")
    }
    fn score(&self) -> &super::Score {
        panic!("Incorrect type.")
    }
    fn effect(&self) -> &crate::prelude::Effect {
        panic!("Incorrect type.")
    }
    fn raw_samples_mut(&mut self) -> &mut RawSamples {
        self
    }
    fn midi_mut(&mut self) -> &mut super::MIDI {
        panic!("Incorrect type.")
    }
    fn score_mut(&mut self) -> &mut super::Score {
        panic!("Incorrect type.")
    }
    fn effect_mut(&mut self) -> &mut crate::prelude::Effect {
        panic!("Incorrect type.")
    }
    
    fn get_type(&self) -> TrackType {
        TrackType::RawSamples
    }
    fn is_type(&self, track_type: TrackType) -> bool {
        if track_type == TrackType::RawSamples {
            return true;
        }

        false
    }
}