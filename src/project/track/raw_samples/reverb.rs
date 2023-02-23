use std::mem::discriminant;

use crate::prelude::Channels;
use super::{RawSamples, fade::*};

impl RawSamples {
    pub fn reverb(&mut self, channels: Channels, delay: f64, decay_factor: f64, mix_percent: f64, offset: f64, duration: f64) {
        for j in 0..self.settings.num_channels {
            if channels == Channels::All || 
                channels == Channels::Just(j) ||
                (discriminant(&channels) == discriminant(&Channels::AllBut(1)) && channels != Channels::AllBut(j))
            {
                let mut vec: Vec<f64> = self.samples[
                    (offset * self.settings.sample_rate as f64) as usize..
                    ((offset + duration) * self.settings.sample_rate as f64 * self.settings.num_channels as f64) as usize].iter().enumerate()
                    .filter(|s| s.0 % self.settings.num_channels == j )
                    .map(|(_, s)| *s).collect();
                
                Self::reverb_vec(&mut vec, delay, decay_factor, mix_percent, self.settings.sample_rate);

                for k in 0..self.samples.len() {
                    if ((offset * self.settings.sample_rate as f64) as usize..
                        ((offset + duration) * self.settings.sample_rate as f64) as usize).contains(&k) &&
                        k % self.settings.num_channels == j
                    {
                        self.samples[2 * k + j] = vec[k];
                    }
                }
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

    #[deprecated]
    pub fn echo(&mut self, channels: Channels, offset: f64, duration: f64, reverb_len: f64) {
        for k in 0..(duration / reverb_len) as usize - 2 {
            println!("{k}, {}, {}", ((offset + k as f64 * reverb_len) * self.settings.sample_rate as f64) as usize, ((offset + (k + 1) as f64 * reverb_len) * self.settings.sample_rate as f64) as usize);
            let mut sample_vec = 
                self.samples[
                    ((offset + k as f64 * reverb_len) * self.settings.sample_rate as f64) as usize..
                    ((offset + (k + 1) as f64 * reverb_len) * self.settings.sample_rate as f64) as usize].to_vec();
            sample_vec = sample_vec.iter().map(|s| s * 0.8).collect();
            Self::fade(&mut sample_vec, vec![Fade::new(0.0, reverb_len, FadeType::Linear, true, self.settings.sample_rate)]);
            self.add(sample_vec, channels, offset + (k as f64 + 1.5) * reverb_len);
        }
    }
}