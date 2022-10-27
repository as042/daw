#![allow(dead_code, unused_imports, unused_variables)]

#[cfg(test)]
mod tests;
mod project;

use project::*;

fn main() {
    let mut project = Project::new();
    project.new_track(TrackType::RawSamples);
    let track = &mut project.tracks[0];
    let data = track.data.raw_samples_mut().unwrap();

    for i in 0..10 {
        data.push_sin_sample(Wave {
            freq: 44000.0,
            amp: 0.1,
            phase_shift: 0.0}, i as f64);
    }

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 2}, "test.wav").unwrap();
}