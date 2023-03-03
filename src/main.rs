use daw::prelude::*;
#[allow(unused_imports)]
use performance_tester::test_performance;

fn main() {
    // let mut project = Project::new();
    // let data = project.new_track(TrackType::MIDI).midi_mut();

    // data.add_note(Note { 
    //     freq: 440.0, 
    //     velocity: 60, 
    //     channels: Channels::All, 
    //     instrument: Instrument::SubtractiveSynth, 
    //     time: Time::new(0.0, 1.35) });

    // project.export_wav(WavSettings { 
    //     num_channels: 2, 
    //     sample_rate: 88200, 
    //     bytes_per_sample: 3 }, "test.wav", false).uw();

    Project::from_console_input();
}