use crate::project::WavSettings;

pub fn resample(samples: &Vec<u8>, _init_sample_rate: i32, _export_settings: WavSettings) -> Vec<u8> {
    samples.to_vec()
}