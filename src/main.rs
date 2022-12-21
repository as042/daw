#![allow(dead_code, unused_imports, unused_variables)]
mod tests;
mod project;

use project::*;

fn main() {
    let mut project = Project::new();

    project.new_track(TrackType::RawSamples);
    let data = project.track(TrackType::RawSamples, 0).uw().data.raw_samples_mut().uw();

    data.add_sin_wave(Wave {
        freq: 100000.0,
        amp: 0.2,
        phase_shift: 0.0}, 0.0, 1.0);

    data.push_sin_wave(Wave {
        freq: 392.0,
        amp: 0.2,
        phase_shift: 0.0}, 1.0);

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 2}, "test.wav").uw();
}