#![allow(dead_code, non_snake_case)]

use std::{fs::OpenOptions, io::Write, path::Path, default};

pub mod track;
pub mod track_type;
pub mod wav_settings;
pub mod wav_writer;
pub mod wave;

pub use track::*;
pub use track_type::*;
pub use wav_settings::*;
pub use wav_writer::*;
pub use wave::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Project<T: TrackData + Default> {
    pub tracks: Vec<Track<T>>
}

impl<T: TrackData + Default> Project<T> {
    pub fn new() -> Self {
        Self { tracks: Vec::default() }
    }

    pub fn new_track(&mut self, track_type: TrackType) {
        if track_type == TrackType::RawSamples {
            self.tracks.push(Track::<T>::default());
        }
        else if track_type == TrackType::Score {
            self.tracks.push(track);
        }
        else if track_type == TrackType::MIDI {
            self.tracks.push(track);
        }        
    }

    pub fn export_midi(&self) -> Result<(), String> {
        if self.tracks.len() == 0 { return Err("Project must have at least 1 track.".to_string()); }
        if self.tracks.iter().any(|x| x.is_type(TrackType::RawSamples)) { return Err("Tracks cannot be RawSamples type.".to_string()); }

        todo!();
    }

    pub fn export_wav(&self, wav_settings: WavSettings, path: impl AsRef<Path>) -> Result<(), String> {
        if self.tracks.len() == 0 { return Err("Project must have at least 1 track.".to_string()); }

        let mut wav = Wav {
            num_channels: wav_settings.num_channels,
            sample_rate: wav_settings.sample_rate,
            bits_per_sample: wav_settings.bytes_per_sample * 8,
            ..Default::default()
        };

        let wav_vector = wav.create_wav(self);

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path).unwrap();

        file.write_all(&wav_vector).unwrap();

        Ok(())
    }
}