use crate::{project::{wav_writer, Project, TrackType, UW, Wave, WavSettings}, prelude::channels::Channels};
use wav_writer::raw_sample_writer::*;

#[test]
fn test_construct_project() {
    assert_eq!(Project::new(), Project::default());
}

#[test]
fn test_new_track() {
    let mut project = Project::new();
    project.new_track(TrackType::RawSamples);
    project.new_track(TrackType::Score);
    project.new_track(TrackType::MIDI);

    assert_eq!(project.tracks.len(), 3);
}

#[test]
fn test_export_wav() {
    let mut project = Project::new();
    project.new_track(TrackType::RawSamples);

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 2}, "test.wav", false).uw();
}

#[test]
pub fn test_raw_samples() {
    let mut project = Project::new();
    project.new_track(TrackType::RawSamples);
    let track1 = &mut project.tracks[0];
    let data = track1.raw_samples_mut();

    data.add_sample(1.0, 0, Channels::None);
    data.add_sample(1.0, 1, Channels::None);
    data.add_sample(1.0, 2, Channels::None);
    data.add_sample(1.0, 3, Channels::None);

    project.export_wav(WavSettings { 
        num_channels: 1, 
        sample_rate: 44100, 
        bytes_per_sample: 2}, "test.wav", false).uw();
}