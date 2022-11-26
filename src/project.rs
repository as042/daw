#![allow(dead_code, non_snake_case)]

use std::{fs::OpenOptions, io::Write, path::Path};
pub use method_shorthands::{better_unwrap::*, better_expect::*, better_to_string::*};

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
pub struct Project {
    pub tracks: Vec<Track>
}

impl Project {
    pub fn new() -> Self {
        Self { tracks: Vec::default() }
    }

    pub fn new_track(&mut self, track_type: TrackType) {
        let mut track = Track {
            data: TrackDataType::default()
        };

        track.data = match track_type {
            TrackType::RawSamples => TrackDataType::RawSamples(RawSamples::default()),
            TrackType::Score => TrackDataType::Score(Score::default()),
            TrackType::MIDI => TrackDataType::MIDI(MIDI::default())
        };

        self.tracks.push(track);
    }

    pub fn track(&mut self, track_type: TrackType, rank: usize) -> Result<&mut Track, String> {
        let mut count = 0;
        for k in 0..self.tracks.len() {
            if self.tracks[k].is_type(track_type) {
                if count == rank {
                    return Ok(&mut self.tracks[k]);
                }

                count += 1;
            }
        }

        Err("Cannot find specific track.".ts())
    }

    pub fn export_midi(&self) -> Result<(), String> {
        if self.tracks.len() == 0 { return Err("Project must have at least 1 track.".ts()); }
        if self.tracks.iter().any(|x| x.is_type(TrackType::RawSamples)) { return Err("Tracks cannot be RawSamples type.".ts()); }

        todo!();
    }

    pub fn export_wav(&self, wav_settings: WavSettings, path: impl AsRef<Path>) -> Result<(), String> {
        if self.tracks.len() == 0 { return Err("Project must have at least 1 track.".ts()); }

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
            .open(path).uw();

        file.write_all(&wav_vector).uw();

        Ok(())
    }
}