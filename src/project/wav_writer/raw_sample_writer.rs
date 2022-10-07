use std::collections::btree_map::Iter;

use super::*;
use crate::project::resample::*;

impl Wav {
    pub(super) fn raw_sample_data(&self, data: &mut Vec<u8>, tracks: &Vec<Track>) {
        let len = tracks.iter().map(|x| x.len()).max().unwrap();

        for track in tracks.iter().filter(|x| x.is_type(TrackType::RawSamples)) {
            let raw_samples = track.data.raw_samples().unwrap();
            let samples = raw_samples.samples();
            let settings = raw_samples.settings;

            let prev_channel = usize::default();
            for i in (0..len).step_by(settings.bytes_per_sample as usize) {
                let channel = i / settings.bytes_per_sample as usize % settings.num_channels; // idx of channel

                // channel idx is as high as it goes but less than export channels
                if settings.num_channels < self.NumChannels && channel + 1 == settings.num_channels {
                    self.compute_raw_sample(data, samples, settings, i);

                    for _ in 0..self.NumChannels - channel + 1 {
                        for _ in 0..self.NumChannels {
                            data.push(0);
                        }
                    }
                }
                // channel idx is within range of export channels
                else if channel < self.NumChannels {
                    self.compute_raw_sample(data, samples, settings, i);
                }

                // channel idx is outside range of export channels
            }
        }
    }

    fn compute_raw_sample(&self, data: &mut Vec<u8>, samples: &Vec<u8>, settings: WavSettings, i: usize) {
        let mut sample = [0; 8];
                    
        for k in 0..settings.bytes_per_sample as usize {
            sample[k] = samples[i + k];
        }

        // the "easy" case
        if settings.sample_rate == self.SampleRate {
            self.write_raw_sample(data, sample, settings, i);
        }
        // ruh-roh
        else {
            let time = i as i32 / settings.bytes_per_sample * settings.num_channels as i32;

            resample(sample);
        }
    }

    fn write_raw_sample(&self, data: &mut Vec<u8>, sample: [u8; 8], settings: WavSettings, i: usize) {
        let value1 = self.sample_to_value(sample, settings);

        let idx = i / settings.bytes_per_sample as usize;
        let mut sample2 = [0; 8];
        for k in 0..8 {
            sample2[k] = data[idx + k];
        }
        let value2 = self.sample_to_value(sample2, settings);
        
        let mut sum = value1 + value2;
        let sample_max = 2_f64.powf(self.BitsPerSample as f64);

        sum *= sample_max;

        if sum.abs() >= sample_max {
            sum = sum.signum() * (sample_max - 1.0);
        }

        let bytes = sum.to_le_bytes();

        for k in idx..(idx + 8) {
            data[k] = bytes[k - idx];
        }
    }

    fn sample_to_value(&self, sample: [u8; 8], settings: WavSettings) -> f64 {
        let sample_int = u64::from_le_bytes(sample);
        let double = sample_int as f64 / 2_f64.powf(settings.bytes_per_sample as f64 * 8_f64);
        let unsigned_value = double * 2_f64.powf(self.BitsPerSample as f64);
        let final_value = unsigned_value * 2.0 - 1.0;
    
        final_value
    }
}