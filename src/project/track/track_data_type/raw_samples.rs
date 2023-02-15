pub mod basic_waveforms;
pub mod modifiers;
pub mod timbres;

pub use method_shorthands::methods::*;

use crate::{project::{WavSettings, sample_conversion::f64_to_sample}, prelude::sample_conversion::sample_to_f64};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RawSamples {
    samples: Vec<u8>,
    pub settings: WavSettings
}

impl RawSamples {
    pub fn samples(&self) -> &Vec<u8> {
        &self.samples
    }

    /// Pushes the given sample to the data.
    pub fn push_sample(&mut self, sample: f64) {
        for _ in 0..self.settings.num_channels {
            self.samples.extend_from_slice(&f64_to_sample(sample, self.settings.bytes_per_sample)[0..self.settings.bytes_per_sample]);
        }
    }
    /// Adds the given sample to the data.
    pub fn add_sample(&mut self, sample: f64, idx: usize) {
        let idx = idx * self.settings.block_align();

        for _ in 0..idx as i32 + 8 - self.samples.len() as i32 {
            let zero_sample = f64_to_sample(0.0, self.settings.bytes_per_sample);
            for k in 0..self.settings.bytes_per_sample {
                self.samples.push(zero_sample[k]); // spam 0s to prevent index errors
            }
        }


        let mut sample2 = [0; 8];
        for k in 0..self.settings.bytes_per_sample {
            sample2[k] = self.samples[idx + k];
        }

        let sum = sample + sample_to_f64(sample2, self.settings.bytes_per_sample);
        //println!("sample: {:?}, sample2: {:?}, sum: {}", sample, sample2, sum);
        let final_sample = f64_to_sample(sum, self.settings.bytes_per_sample);

        for j in 0..self.settings.num_channels {
            for k in (idx + j * self.settings.bytes_per_sample..idx + self.settings.bytes_per_sample + j * self.settings.bytes_per_sample).enumerate() {
                self.samples[k.1] = final_sample[k.0];
            }
        }
    }
    // Adds the input to the data.
    pub fn add(&mut self, input: Vec<f64>, offset: f64) {
        for k in 0..input.len() {
            self.add_sample(input[k], k + (offset * self.settings.sample_rate as f64) as usize);
        }
    }

    /// Pushes a constant to the data.
    pub fn push_const(&mut self, amp: f64, duration: f64) {
        for _ in 0..(duration * self.settings.sample_rate as f64) as usize {
            self.push_sample(amp);
        }
    }
    /// Adds a constant to the existing data.
    pub fn add_const(&mut self, amp: f64, offset: f64, duration: f64) {
        for k in (offset * self.settings.sample_rate as f64) as usize..((offset + duration) * self.settings.sample_rate as f64) as usize {
            self.add_sample(amp, k);
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