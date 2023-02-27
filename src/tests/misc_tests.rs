use crate::{project::{wav_writer, Project, TrackType, UW, Wave, WavSettings}, prelude::{channels::Channels, Time, C4}};
use wav_writer::raw_sample_writer::*;

fn test() {
    let mut project = Project::new();

    project.new_track(TrackType::RawSamples);
    let data = project.track(TrackType::RawSamples, 0).uw().raw_samples_mut();

    data.add_subtractive_synth_note(Wave {
        freq: C4,
        amp: 0.1,
        phase_shift: 0.0,
    }, Channels::All, Time::new(0.0, 60.0));

    data.reverb(Channels::All, 0.1, 0.5, 50.0, Time::new(0.0, 60.0));

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 2}, "test.wav", false).uw();
}

#[test]
fn multiple_tracks() {
    let mut project = Project::new();

    project.new_track(TrackType::RawSamples);
    let data = project.track(TrackType::RawSamples, 0).uw().raw_samples_mut();

    data.add_sin_wave(Wave {
        freq: 261.63,
        amp: 0.2,
        phase_shift: 0.0}, Channels::None, Time::new(0.0, 0.1));

    project.new_track(TrackType::RawSamples);
    let data = project.track(TrackType::RawSamples, 1).uw().raw_samples_mut();

    data.add_sin_wave(Wave {
        freq: 329.63,
        amp: 0.1,
        phase_shift: 0.0}, Channels::None, Time::new(0.0, 0.1));

    project.new_track(TrackType::RawSamples);
    let data = project.track(TrackType::RawSamples, 2).uw().raw_samples_mut();

    data.add_sin_wave(Wave {
        freq: 392.0,
        amp: 0.1,
        phase_shift: 0.0}, Channels::None, Time::new(0.0, 0.1));

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 3}, "test.wav", false).uw();
}