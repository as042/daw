use crate::{project::WavSettings, prelude::Samples};

pub fn resample(samples: &Samples, settings: WavSettings, export_sample_rate: i32) -> Samples {
    let mut resamples = samples.clone();
    
    for j in 0..settings.num_channels {
        let new_len = (samples[j].len() as f64 * export_sample_rate as f64 / settings.sample_rate as f64).round() as usize;
        let mut resampled = vec![0.0; new_len];
        let step = settings.sample_rate as f64 / export_sample_rate as f64;
        let mut i = 0_f64;
        for k in 0..new_len {
            let sample_index = i.floor() as usize;
            let frac = i - sample_index as f64;
            if sample_index < samples[j].len() - 1 {
                resampled[k] = samples[j][sample_index] * (1.0 - frac) + samples[j][sample_index + 1] * frac;
            } else {
                resampled[k] = samples[j][sample_index];
            }
            i += step;
        }

        resamples[j] = resampled;
    }

    resamples
}