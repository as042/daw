use std::vec;

use super::{*, sample_conversion::*, resample::*, format::match_num_channels};

// Handles the various conversions of the data needed to prepare it for writing
pub(super) fn raw_sample_data(data: &mut Vec<u8>, tracks: &Vec<Track>, export_settings: WavSettings) {
    for track in tracks.iter().filter(|x| x.is_type(TrackType::RawSamples)) {
        let raw_samples = track.raw_samples();
        let samples = raw_samples.samples();
        let settings = raw_samples.settings;

        let one_vec = change_array_to_vec(samples, settings.num_channels);
        let binary_samples = change_f64_to_sample(&one_vec, export_settings.bytes_per_sample);
        let resamples = resample(&binary_samples, settings.sample_rate, export_settings);
        let final_samples = match_num_channels(&resamples, settings.num_channels, export_settings);

        println!("{}, {}, {}, {}, {}", samples[0].len(), one_vec.len(), binary_samples.len(), resamples.len(), final_samples.len());
        for i in (0..final_samples.len()).step_by(export_settings.bytes_per_sample) {
            let mut sample = [0; 8];
            for k in 0..export_settings.bytes_per_sample {
                sample[k] = final_samples[i + k];
            }

            write_raw_sample(data, sample, export_settings, i);
        }
    }
}

// Collapse the array of vecs into one vec
fn change_array_to_vec(samples: &[Vec<f64>; 8], num_channels: usize) -> Vec<f64> {
    let mut samples = samples.clone();

    let len = samples.iter().map(|v| v.len()).max().uw();
    // Pads zeros to prevent index out-of-bounds errors
    for k in 0..num_channels {
        for _ in 0..len - samples[k].len() {
            samples[k].push(0.0);
        }
    }

    let mut vec = vec![];
    for j in 0..len {
        for k in 0..num_channels {
            vec.push(samples[k][j]);
        }
    }

    vec
}

// Convert all of the floats into binary samples
fn change_f64_to_sample(samples: &Vec<f64>, bytes_per_sample: usize) -> Vec<u8> {
    let mut output = vec![];
    for k in 0..samples.len() {
        output.extend_from_slice(&f64_to_sample(samples[k], bytes_per_sample)[0..bytes_per_sample]);
    }

    output
}

// Write the raw samples to the data
fn write_raw_sample(data: &mut Vec<u8>, sample: [u8; 8], export_settings: WavSettings, idx: usize) {
    let mut sample2 = [0; 8];
    for k in 0..export_settings.bytes_per_sample {
        sample2[k] = data[idx + k];
    }
    
    let sum = add_samples(sample, sample2, export_settings.bytes_per_sample);
    
    let sample = f64_to_sample(sum, export_settings.bytes_per_sample);

    for k in idx..(idx + export_settings.bytes_per_sample) {
        data[k] = sample[k - idx];
    }
}

// Add two samples together
pub fn add_samples(sample1: [u8; 8], sample2: [u8; 8], bytes_per_sample: usize) -> f64 {
    let value1 = sample_to_f64(sample1, bytes_per_sample);
    let value2 = sample_to_f64(sample2, bytes_per_sample);

    value1 + value2
}