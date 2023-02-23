use daw::prelude::*;

fn main() {
    let mut project = Project::new();

    project.new_track(TrackType::RawSamples);
    let data = project.track(TrackType::RawSamples, 0).uw().raw_samples_mut();

    data.add_note1(Wave {
        freq: C4,
        amp: 0.1,
        phase_shift: 0.0,
    }, Channels::All, 0.0, 0.5);
    data.add_note1(Wave {
        freq: E4,
        amp: 0.1,
        phase_shift: 0.0,
    }, Channels::All, 0.5, 0.5);
    data.add_note1(Wave {
        freq: G4,
        amp: 0.1,
        phase_shift: 0.0,
    }, Channels::All, 1.0, 0.5);
    data.add_note1(Wave {
        freq: B4,
        amp: 0.1,
        phase_shift: 0.0,
    }, Channels::All, 1.5, 0.5);
    data.add_note1(Wave {
        freq: D4,
        amp: 0.1,
        phase_shift: 0.0,
    }, Channels::All, 2.0, 0.5);
    data.add_note1(Wave {
        freq: F4,
        amp: 0.1,
        phase_shift: 0.0,
    }, Channels::All, 2.5, 0.5);

    data.reverb(Channels::All, 0.15, 0.9, 72.5, 0.0, 3.0);

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 2}, "test.wav").uw();
}