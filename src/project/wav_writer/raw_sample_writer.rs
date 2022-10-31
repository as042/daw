use super::{*, sample_conversion::*};
use crate::project::resample::*;

pub(super) fn raw_sample_data(data: &mut Vec<u8>, tracks: &Vec<Track>, export_settings: WavSettings) {
    let len = tracks.iter().map(|x| x.len()).max().unwrap();

    for track in tracks.iter().filter(|x| x.is_type(TrackType::RawSamples)) {
        let raw_samples = track.data.raw_samples().unwrap();
        let samples = raw_samples.samples();
        let settings = raw_samples.settings;

        if settings.num_channels < export_settings.num_channels {
            for j in (0..len).step_by(settings.bytes_per_sample * )
        }

        for i in (0..len).step_by(settings.bytes_per_sample as usize) {
            let channel = i / settings.bytes_per_sample as usize % settings.num_channels; // idx of channel

            // channel idx is as high as it goes but less than export channels
            if settings.num_channels < export_settings.num_channels && channel + 1 == settings.num_channels {
                compute_raw_sample(data, samples, settings, export_settings, i);

                // for _ in 0..export_settings.num_channels - channel + 1 {
                //     for _ in 0..export_settings.num_channels {
                //         data.push(0);
                //     }
                // }
            }
            // channel idx is within range of export channels
            else if channel < export_settings.num_channels {
                compute_raw_sample(data, samples, settings, export_settings, i);
            }

            // channel idx is outside range of export channels
        }
    }
}

pub fn compute_raw_sample(data: &mut Vec<u8>, samples: &Vec<u8>, settings: WavSettings, export_settings: WavSettings, i: usize) {
    let mut sample = [0; 8];
            
    // println!("samples: {:?}", samples);
    for k in 0..settings.bytes_per_sample as usize {
        sample[k] = samples[i + k];
        // println!("i: {i}, k: {k}");
    }

    // println!("{:?}", sample);

    // the "easy" case
    if settings.sample_rate == export_settings.sample_rate {
        write_raw_sample(data, sample, settings, export_settings, i);
    }
    // ruh-roh
    else {
        let time = i as i32 / settings.bytes_per_sample * settings.num_channels as i32;

        resample(sample);
    }
}

pub fn write_raw_sample(data: &mut Vec<u8>, sample: [u8; 8], settings: WavSettings, export_settings: WavSettings, i: usize) {
    let value1 = sample_to_f64(sample, settings.bytes_per_sample);

    let idx = i / settings.bytes_per_sample as usize * export_settings.bytes_per_sample as usize;
    let mut sample2 = [0; 8];
    for k in 0..export_settings.bytes_per_sample as usize {
        sample2[k] = data[idx + k];
    }
    //println!("idx: {idx}, sample2: {:?}", sample2);
    let value2 = sample_to_f64(sample2, export_settings.bytes_per_sample);

    let sum = value1 + value2 + 0.0;
    // println!("sample1: {:?}, {value1}, {value2}, {sum}", sample);

    let sample = f64_to_sample(sum, export_settings.bytes_per_sample);

    for k in idx..(idx + export_settings.bytes_per_sample as usize) {
        //println!("k: {}, data: {}, sample: {}", k, data[k], sample[k - idx]);
        data[k] = sample[k - idx];
    }
}