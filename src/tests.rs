use wav_writer::raw_sample_writer::*;

use crate::project::*;

#[test]
fn multiple_tracks() {
    let mut project = Project::new();

    project.new_track(TrackType::RawSamples);
    let track = &mut project.tracks[0];
    let data = track.data.raw_samples_mut().unwrap();

    data.push_sin_wave(Wave {
        freq: 261.63,
        amp: 0.2,
        phase_shift: 0.0}, 1.0);

    project.new_track(TrackType::RawSamples);
    let track2 = &mut project.tracks[1];
    let data2 = track2.data.raw_samples_mut().unwrap();

    data2.push_sin_wave(Wave {
        freq: 329.63,
        amp: 0.1,
        phase_shift: 0.0}, 1.0);

    project.new_track(TrackType::RawSamples);
    let track3 = &mut project.tracks[2];
    let data3 = track3.data.raw_samples_mut().unwrap();

    data3.push_sin_wave(Wave {
        freq: 392.0,
        amp: 0.1,
        phase_shift: 0.0}, 1.0);

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 3}, "test.wav").unwrap();
}