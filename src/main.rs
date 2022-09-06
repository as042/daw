#![allow(dead_code, unused_imports, unused_variables)]

mod project;

use project::*;

fn main() {
    let mut project = Project::new();

    project.new_track(TrackType::RawSamples);

    let track = project.tracks.iter_mut().find(|x| x.is_type(TrackType::RawSamples) == true).unwrap();

    let data = track.data.raw_samples().unwrap();

    data.push_sample(0xFF);
}