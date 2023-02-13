use daw::project::{wav_writer, Project, TrackType, UW, Wave, WavSettings};
use wav_writer::raw_sample_writer::*;

#[test]
fn multiple_tracks() {
    let mut project = Project::new();

    project.new_track(TrackType::RawSamples);
    let data = project.track(TrackType::RawSamples, 0).uw().data.raw_samples_mut().uw();

    data.push_sin_wave(Wave {
        freq: 261.63,
        amp: 0.2,
        phase_shift: 0.0}, 1.0);

    project.new_track(TrackType::RawSamples);
    let data = project.track(TrackType::RawSamples, 1).uw().data.raw_samples_mut().uw();

    data.push_sin_wave(Wave {
        freq: 329.63,
        amp: 0.1,
        phase_shift: 0.0}, 1.0);

    project.new_track(TrackType::RawSamples);
    let data = project.track(TrackType::RawSamples, 2).uw().data.raw_samples_mut().uw();

    data.push_sin_wave(Wave {
        freq: 392.0,
        amp: 0.1,
        phase_shift: 0.0}, 1.0);

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 3}, "test.wav").uw();
}