use daw::prelude::*;

fn main() {
    let mut project = Project::new();

    project.new_track(TrackType::RawSamples);
    let data = project.track(TrackType::RawSamples, 0).uw().raw_samples_mut();

    data.add_timbre1(Wave {
        freq: C4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 0.0, 1.0);
    data.add_timbre2(Wave {
        freq: C4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 1.0, 1.0);
    data.add_timbre3(Wave {
        freq: C4,
        amp: 0.1,
        phase_shift: 0.0,
    }, 2.0, 1.0);

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 2}, "test.wav").uw();
}