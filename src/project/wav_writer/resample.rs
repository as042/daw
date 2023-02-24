use crate::{project::WavSettings, prelude::Samples};

pub fn resample(samples: &Samples, _settings: WavSettings, _export_settings: WavSettings) -> Samples {
    samples.clone()
    
    // let mut output = Samples::default();
    // for j in 0..settings.num_channels {

    // }

    // output
}