use super::{*, sample_conversion::*, resample::*, format::*};

pub(super) fn raw_sample_data(data: &mut Vec<u8>, tracks: &Vec<Track>, export_settings: WavSettings) {
    for track in tracks.iter().filter(|x| x.is_type(TrackType::RawSamples)) {
        let raw_samples = track.raw_samples();
        let samples = raw_samples.samples();
        let settings = raw_samples.settings;

        let formatted_samples = format_samples(samples, settings, export_settings);
        let final_samples = resample(&formatted_samples, settings.sample_rate, export_settings);

        for i in (0..final_samples.len()).step_by(export_settings.bytes_per_sample) {
            let mut sample = [0; 8];
            for k in 0..export_settings.bytes_per_sample {
                sample[k] = final_samples[i + k];
            }

            write_raw_sample(data, sample, export_settings, i);
        }
    }
}

fn write_raw_sample(data: &mut Vec<u8>, sample: [u8; 8], export_settings: WavSettings, i: usize) {
    let mut sample2 = [0; 8];
    for k in 0..export_settings.bytes_per_sample {
        sample2[k] = data[i + k];
    }
    
    let sum = add_samples(sample, sample2, export_settings.bytes_per_sample);
    
    let sample = f64_to_sample(sum, export_settings.bytes_per_sample);

    for k in i..(i + export_settings.bytes_per_sample) {
        data[k] = sample[k - i];
    }
}

pub fn add_samples(sample1: [u8; 8], sample2: [u8; 8], bytes_per_sample: usize) -> f64 {
    let value1 = sample_to_f64(sample1, bytes_per_sample);
    let value2 = sample_to_f64(sample2, bytes_per_sample);

    value1 + value2
}