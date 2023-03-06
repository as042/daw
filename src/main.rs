use daw::prelude::*;
#[allow(unused_imports)]
use performance_tester::test_performance;

fn main() {
    // let mut project = Project::new();
    // let data = project.new_track(TrackType::MIDI).midi_mut();

    // data.add_note(Note { 
    //     freq: C4, 
    //     velocity: 60, 
    //     channels: Channels::All, 
    //     instrument: Instrument::SubtractiveSynth, 
    //     time: Time::new(0.0, 2.0) });
    // data.add_note(Note { 
    //     freq: E4, 
    //     velocity: 60, 
    //     channels: Channels::All, 
    //     instrument: Instrument::SubtractiveSynth, 
    //     time: Time::new(2.0, 2.0) });
    // data.add_note(Note { 
    //     freq: G4, 
    //     velocity: 60, 
    //     channels: Channels::All, 
    //     instrument: Instrument::SubtractiveSynth, 
    //     time: Time::new(4.0, 2.0) });

    // let data = project.new_track(TrackType::Effect).effect_mut();

    // data.set_effect(EffectType::Fade(Fade::new(FadeType::Power(2.0), false, Time::new(0.0, 1.0))), vec![0], Time::new(0.0, 6.0));
    // data.set_effect(EffectType::Reverb(0.1, 0.9, 50.0), vec![0], Time::new(0.0, 1.0));
    // panic!("{:#?}", toml::to_string(data));

    // project.export_wav(WavSettings { 
    //     num_channels: 2, 
    //     sample_rate: 88200, 
    //     bytes_per_sample: 3 }, "test.wav", false).uw();

    Project::from_console_input();
}