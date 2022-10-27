use wav_writer::raw_sample_writer::*;

use crate::project::*;

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
        bytes_per_sample: 2}, "test.wav").unwrap();
}

#[test]
fn test_raw_samples() {
    let mut project = Project::new();
    project.new_track(TrackType::RawSamples);
    project.new_track(TrackType::RawSamples);
    let track1 = &mut project.tracks[0];
    let data = track1.data.raw_samples_mut().unwrap();

    for _ in 0..22050 {
        data.push_sample(&[0, 0x08]);
        data.push_sample(&[0, 0xFF]);
        data.push_sample(&[0xA, 0x80]);
        data.push_sample(&[0, 0x7F]);
    }

    let track2 = &mut project.tracks[1];
    let data = track2.data.raw_samples_mut().unwrap();

    for _ in 0..22050 {
        data.push_sample(&[0, 0x08]);
        data.push_sample(&[0, 0xFF]);
        data.push_sample(&[0xA, 0x80]);
        data.push_sample(&[0, 0x7F]);
    }

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 2}, "test.wav").unwrap();
}

#[test]
fn test_push_sin_sample() {
    let mut project = Project::new();
    project.new_track(TrackType::RawSamples);
    let track = &mut project.tracks[0];
    let data = track.data.raw_samples_mut().unwrap();

    for i in 0..100 {
        data.push_sin_sample(Wave {
            freq: 440.0,
            amp: 0.1,
            phase_shift: 0.0}, i as f64);
    }

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 2}, "test.wav").unwrap();
}