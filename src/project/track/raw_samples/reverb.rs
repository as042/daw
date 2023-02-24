use std::mem::discriminant;

use crate::prelude::{Channels, Time};
use super::RawSamples;

impl RawSamples {
    pub fn reverb(&mut self, channels: Channels, delay: f64, decay_factor: f64, mix_percent: f64, time: Time) {
        for j in 0..self.settings.num_channels {
            if channels == Channels::All || 
                channels == Channels::Just(j) ||
                (discriminant(&channels) == discriminant(&Channels::AllBut(1)) && channels != Channels::AllBut(j))
            {
                let range = (time.start * self.settings.sample_rate as f64) as usize..
                    (time.end * self.settings.sample_rate as f64) as usize;
                let mut vec = self.samples[j][range.clone()].to_vec();
                
                Self::reverb_vec(&mut vec, delay, decay_factor, mix_percent, self.settings.sample_rate);

                self.samples[j].splice(range, vec);
            }
        }
    }

    pub fn reverb_vec(buffer: &mut Vec<f64>, delay: f64, decay_factor: f64, mix_percent: f64, sample_rate: i32) {
        let comb_buffer = &mut buffer.clone();
        let mut buffer2 = buffer.clone();
        let mut buffer3 = buffer.clone();
        let mut buffer4 = buffer.clone();
        let mut buffer5 = buffer.clone();

        Self::comb_filter(&mut buffer2, delay, decay_factor, sample_rate);
		Self::comb_filter(&mut buffer3, delay - 0.01173, decay_factor - 0.1313, sample_rate);
		Self::comb_filter(&mut buffer4, delay + 0.01931, decay_factor - 0.2743, sample_rate);
		Self::comb_filter(&mut buffer5, delay - 0.00797, decay_factor - 0.31, sample_rate);

        for k in 0..buffer.len() {
            comb_buffer[k] = buffer2[k] + buffer3[k] + buffer4[k] + buffer5[k];
        }

        for k in 0..buffer.len() {
            buffer[k] = ((100.0 - mix_percent) / 100.0 * buffer[k]) + (mix_percent / 100.0 * comb_buffer[k]);
        }

        Self::all_pass_filter(buffer, sample_rate);
        Self::all_pass_filter(buffer, sample_rate);
    }

    pub fn comb_filter(buffer: &mut Vec<f64>, delay: f64, decay_factor: f64, sample_rate: i32) {
        let delay_samples = (delay * sample_rate as f64) as usize;
    
        // Applying algorithm for Comb Filter
        for k in 0..buffer.len() - delay_samples {
            buffer[k + delay_samples] += buffer[k] * decay_factor;
        }
    }

    pub fn all_pass_filter(buffer: &mut Vec<f64>, sample_rate: i32) {
        let delay_samples = (0.08927 * sample_rate as f64) as usize; // Number of delay samples. Calculated from number of samples per millisecond
        let decay_factor = 0.131;
    
        // Applying algorithm for All Pass Filter
        for k in 0..buffer.len() {
            if k >= delay_samples {
                buffer[k] += -decay_factor * buffer[k - delay_samples];
            }
            if k >= delay_samples + 1 {
                buffer[k] += decay_factor * buffer[k + 20 - delay_samples];
            }
        }
    
        // This is for smoothing out the samples and normalizing the audio. Without implementing this, the samples overflow causing clipping of audio
        let mut value = buffer[0];
        let mut max = 0.0;
    
        for k in 0..buffer.len() {
            if buffer[k].abs() > max {
                max = buffer[k].abs();
            }
        }
    
        for k in 0..buffer.len() {
            let current_value = buffer[k];
            value = (value + (current_value - value)) / max;
    
            buffer[k] = value;
        }
    }
}