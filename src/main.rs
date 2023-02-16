#![allow(dead_code, unused_imports, unused_variables)]
mod tests;

use daw::{prelude::*, project::raw_samples::RawSamples};

fn main() {
    let mut project = Project::new();

    project.new_track(TrackType::RawSamples);
    let data = project.track(TrackType::RawSamples, 0).uw().raw_samples_mut();

    data.add_timbre1(Wave {
        freq: C4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 0.0, 0.4);
    data.add_timbre1(Wave {
        freq: D4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 0.4, 0.4);
    data.add_timbre1(Wave {
        freq: DS4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 0.8, 0.4);
    data.add_timbre1(Wave {
        freq: F4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 1.2, 0.4);
    data.add_timbre1(Wave {
        freq: D4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 1.6, 0.6);
    data.add_timbre1(Wave {
        freq: AS3,
        amp: 0.1,
        phase_shift: 0.0,
    }, 2.2, 0.3);
    data.add_timbre1(Wave {
        freq: C4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 2.5, 0.6);

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 2}, "test.wav").uw();
}