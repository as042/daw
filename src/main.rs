#![allow(dead_code, unused_imports, unused_variables)]
mod tests;

use daw::prelude::*;

fn main() {
    let mut project = Project::new();

    project.new_track(TrackType::RawSamples);
    let data = project.track(TrackType::RawSamples, 0).uw().data.raw_samples_mut().uw();

    data.add_timbre1(Wave {
        freq: C4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 0.0, 5.0);
    data.add_timbre1(Wave {
        freq: E4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 1.0, 5.0);
    data.add_timbre1(Wave {
        freq: G4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 2.0, 5.0);

    data.add_timbre1(Wave {
        freq: C4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 8.0, 0.5);
    data.add_timbre1(Wave {
        freq: D4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 8.5, 0.5);
    data.add_timbre1(Wave {
        freq: DS4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 9.0, 0.5);
    data.add_timbre1(Wave {
        freq: F4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 9.5, 0.5);
    data.add_timbre1(Wave {
        freq: D4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 10.0, 1.0);
    data.add_timbre1(Wave {
        freq: AS3,
        amp: 0.1,
        phase_shift: 0.0,
    }, 11.0, 0.25);
    data.add_timbre1(Wave {
        freq: C4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 11.25, 1.0);

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 2}, "test.wav").uw();
}