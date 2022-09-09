#![allow(dead_code, unused_imports, unused_variables)]

mod project;

use project::*;

fn main() {
    let mut project = Project::new();

    project.new_track(TrackType::RawSamples);

    let track = project.tracks.iter_mut().find(|x| x.is_type(TrackType::RawSamples)).unwrap();

    let data = track.data.raw_samples_mut().unwrap();

    for _ in 0..44100 * 1 {
        data.push_sample(0x00);
        data.push_sample(0x80);
        data.push_sample(0x00);
        data.push_sample(0x80);
        data.push_sample(0x00);
        data.push_sample(0xC0);
        data.push_sample(0x00);
        data.push_sample(0xC0);
    }

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 2}, "test.wav".to_string()).unwrap();
}