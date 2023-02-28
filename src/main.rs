use daw::prelude::*;
#[allow(unused_imports)]
use performance_tester::test_performance;

fn main() {
    let mut project = Project::new();
    let data = project.new_track(TrackType::MIDI).midi_mut();

    data.add_note(Note { 
        freq: 440.0, 
        velocity: 60, 
        channels: Channels::All, 
        instrument: Instrument::SubtractiveSynth, 
        time: Time::new(0.0, 1.35) });

    project.export_wav(WavSettings { 
        num_channels: 2, 
        sample_rate: 88200, 
        bytes_per_sample: 3 }, "test.wav", false).uw();

    // project_to_wav()
}

#[allow(dead_code)]
fn project_to_wav() {
    let mut path = String::default();
    std::io::stdin().read_line(&mut path).uw();

    let from_toml = Project::from_toml(path.trim(), true);
    if let Ok(output) = from_toml {
        output.0.export_wav(output.1, output.2, true).uw();
    }  
    else {
        println!("Error: {}. \nOperation unsuccessful.", from_toml.unwrap_err());
    }

    std::io::stdin().read_line(&mut path).uw();
}