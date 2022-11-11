use super::{*, sample_conversion::*, resample::*, format::*};

pub(super) fn raw_sample_data<T: TrackData + Default>(data: &mut Vec<u8>, tracks: &Vec<Track<T>>, export_settings: WavSettings) {
    for track in tracks.iter().filter(|x| x.is_type(TrackType::RawSamples)) {
        let raw_samples = track.data.raw_samples();
        let samples = raw_samples.samples();
        let settings = raw_samples.settings;

        // println!("{:?}", samples);

        let formatted_samples = format_samples(samples, settings, export_settings);
        let final_samples = resample(&formatted_samples, settings.sample_rate, export_settings);
        // println!("{:?}", final_samples);

        for i in (0..final_samples.len()).step_by(export_settings.bytes_per_sample) {
            //println!("{o}");
            let mut sample = [0; 8];
            for k in 0..export_settings.bytes_per_sample {
                sample[k] = final_samples[i + k];
            }

            write_raw_sample(data, sample, settings, export_settings, i);
        }
    }
}

pub fn write_raw_sample(data: &mut Vec<u8>, sample: [u8; 8], settings: WavSettings, export_settings: WavSettings, i: usize) {
    let value1 = sample_to_f64(sample, export_settings.bytes_per_sample);

    let mut sample2 = [0; 8];
    for k in 0..export_settings.bytes_per_sample {
        sample2[k] = data[i + k];
    }
    //println!("idx: {idx}, sample2: {:?}", sample2);
    let value2 = sample_to_f64(sample2, export_settings.bytes_per_sample);

    let sum = value1 + value2;
    // println!("sample1: {:?}, {value1}, {value2}, {sum}", sample);

    let sample = f64_to_sample(sum, export_settings.bytes_per_sample);

    for k in i..(i + export_settings.bytes_per_sample) {
        //println!("k: {}, data: {}, sample: {}", k, data[k], sample[k - idx]);
        data[k] = sample[k - i];
    }
}