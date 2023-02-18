use daw::prelude::{*, channels::Channels};

fn main() {
    let mut project = Project::new();

    project.new_track(TrackType::RawSamples);
    let data = project.track(TrackType::RawSamples, 0).uw().raw_samples_mut();

    let mut vec = data.new_triangle_wav(Wave {
        freq: FS4,
        amp: 0.2,
        phase_shift: 0.0,
    }, 2.0);
    RawSamples::fade(&mut vec, false);
    data.add(vec, Channels::All, 0.0);

    let mut vec = data.new_triangle_wav(Wave {
        freq: FS4,
        amp: 0.2,
        phase_shift: 0.0,
    }, 2.0);
    RawSamples::fade(&mut vec, true);
    data.add(vec, Channels::All, 2.0);

    let mut vec = data.new_triangle_wav(Wave {
        freq: FS4,
        amp: 0.2,
        phase_shift: 0.0,
    }, 2.0);
    RawSamples::fade(&mut vec, false);
    data.add(vec, Channels::Just(0), 5.0);

    let mut vec = data.new_triangle_wav(Wave {
        freq: FS4,
        amp: 0.2,
        phase_shift: 0.0,
    }, 2.0);
    RawSamples::fade(&mut vec, true);
    data.add(vec, Channels::Just(0), 7.0);
    let mut vec = data.new_triangle_wav(Wave {
        freq: FS4,
        amp: 0.2,
        phase_shift: 0.0,
    }, 2.0);
    RawSamples::fade(&mut vec, false);
    data.add(vec, Channels::Just(1), 7.0);

    let mut vec = data.new_triangle_wav(Wave {
        freq: FS4,
        amp: 0.2,
        phase_shift: 0.0,
    }, 2.0);
    RawSamples::fade(&mut vec, true);
    data.add(vec, Channels::Just(1), 9.0);

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 44100, 
        bytes_per_sample: 2}, "test.wav").uw();
}