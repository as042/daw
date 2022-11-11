#![allow(dead_code, unused_imports, unused_variables)]

#[cfg(test)]
mod unit_tests;
#[cfg(test)]
mod tests;
mod project;

use project::*;

fn main() {
    let mut project = Project::new();

    project.new_track(TrackType::RawSamples);
    let track = &mut project.tracks[0];
    let data = track.data.raw_samples_mut().unwrap();

    data.push_sin_wave(Wave {
        freq: 261.63,
        amp: 0.2,
        phase_shift: 0.0}, 1.0);

    // project.new_track(TrackType::RawSamples);
    // let track2 = &mut project.tracks[1];
    // let data2 = track2.data.raw_samples_mut().unwrap();

    // data2.push_sin_wave(Wave {
    //     freq: 392.0,
    //     amp: 0.1,
    //     phase_shift: 0.0}, 1.0);

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 2}, "test.wav").unwrap();
}