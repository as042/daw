use effect::EffectType;
use raw_samples::{Samples, RawSamples};
use crate::prelude::{Wave, Instrument};
use super::{*, sample_conversion::*, resample::*, format::match_num_channels};

// Handles the various conversions of the data needed to prepare it for writing
pub(super) fn raw_sample_data(data: &mut Vec<u8>, tracks: &Vec<Track>, export_settings: WavSettings, progress_updates: bool) {
    let mut raw_sample_tracks = tracks.to_vec();
    for track in tracks.iter().filter(|t| t.is_type(TrackType::MIDI)) {
        if progress_updates { println!("Converting MIDI track data to raw samples."); }
        let notes = track.midi().notes();
        let mut raw_sample_track = Track::default();
        raw_sample_track.data = Box::new(RawSamples::default());
        let data = raw_sample_track.raw_samples_mut();
        
        for note in notes {
            let wave = Wave { 
                freq: note.freq, 
                amp: (note.velocity as f64 / 128 as f64).powf(2.5), 
                phase_shift: 0.0
            };

            match note.instrument {
                Instrument::SubtractiveSynth => data.add_subtractive_synth_note(wave, note.channels, note.time)
            }
        }

        for effect in tracks.iter().filter(|t| t.is_type(TrackType::Effect)) {
            let effect_data = effect.data.effect();
            if effect_data.affected_tracks.contains(&tracks.iter().position(|t| t == track).uw()) {
                match effect_data.effect_type {
                    EffectType::Reverb(del, dec, mix) => data.reverb(effect_data.channels, del, dec, mix, effect_data.time),
                    EffectType::Fade(fade) => data.fade(fade, effect_data.channels, effect_data.time),
                }
            }
        }

        raw_sample_tracks.push(raw_sample_track);
    }
    for track in raw_sample_tracks.iter().filter(|t| t.is_type(TrackType::RawSamples)).map(|t| t) {
        let samples = track.raw_samples().samples();
        let settings = track.raw_samples().settings;
        
        if progress_updates { println!("Resampling."); }

        let resamples = resample(samples, settings, export_settings.sample_rate);

        if progress_updates { println!("Formatting raw samples."); }

        let one_vec = change_array_to_vec(&resamples, settings.num_channels);
        let binary_samples = change_f64_to_sample(&one_vec, export_settings.bytes_per_sample);
        let final_samples = match_num_channels(&binary_samples, settings.num_channels, export_settings);
        
        if progress_updates { println!("Writing raw samples to data."); }

        for i in (0..final_samples.len()).step_by(export_settings.bytes_per_sample) {
            let mut sample = [0; 8];
            for k in 0..export_settings.bytes_per_sample {
                sample[k] = final_samples[i + k];
            }

            write_raw_sample(data, sample, export_settings, i);
        }
    }
}

// Collapse the array of vecs into one vec
fn change_array_to_vec(samples: &Samples, num_channels: usize) -> Vec<f64> {
    let mut samples = samples.clone();

    let len = samples.iter().map(|v| v.len()).max().uw();
    // Pads zeros to prevent index out-of-bounds errors
    for k in 0..num_channels {
        for _ in 0..len - samples[k].len() {
            samples[k].push(0.0);
        }
    }

    let mut vec = vec![];
    for j in 0..len {
        for k in 0..num_channels {
            vec.push(samples[k][j]);
        }
    }

    vec
}

// Convert all of the floats into binary samples
fn change_f64_to_sample(samples: &Vec<f64>, bytes_per_sample: usize) -> Vec<u8> {
    let mut output = vec![];
    for k in 0..samples.len() {
        output.extend_from_slice(&f64_to_sample(samples[k], bytes_per_sample)[0..bytes_per_sample]);
    }

    output
}

// Write the raw samples to the data
fn write_raw_sample(data: &mut Vec<u8>, sample: [u8; 8], export_settings: WavSettings, idx: usize) {
    let mut sample2 = [0; 8];
    for k in 0..export_settings.bytes_per_sample {
        sample2[k] = data[idx + k];
    }
    
    let sum = add_samples(sample, sample2, export_settings.bytes_per_sample);
    
    let sample = f64_to_sample(sum, export_settings.bytes_per_sample);

    for k in idx..(idx + export_settings.bytes_per_sample) {
        data[k] = sample[k - idx];
    }
}

// Add two samples together
pub fn add_samples(sample1: [u8; 8], sample2: [u8; 8], bytes_per_sample: usize) -> f64 {
    let value1 = sample_to_f64(sample1, bytes_per_sample);
    let value2 = sample_to_f64(sample2, bytes_per_sample);

    value1 + value2
}