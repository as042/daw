#![allow(dead_code, unused_imports)]

mod project;

use project::{*, track::{*, track_data_type::*}};
use project::track_type::*;

fn main() {
    let mut project = Project::new();

    project.new_track(TrackType::MIDI);
    project.export_wav().unwrap();
}