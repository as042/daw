#![allow(dead_code, non_snake_case)]

use std::{mem::discriminant, fs::OpenOptions, io::Write, path::Path};

pub mod track;
pub mod track_type;
pub mod wav_settings;
pub mod wav_writer;
pub mod wave;
mod resample;

pub use track::*;
pub use track_type::*;
pub use wav_settings::*;
pub use wav_writer::*;

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

    pub fn export_midi(&self) -> Result<(), String> {
        if self.tracks.len() == 0 { return Err("Project must have at least 1 track.".to_string()); }
        if self.tracks.iter().any(|x| x.is_type(TrackType::MIDI) == false) { return Err("Not all tracks are MIDI type.".to_string()); }

        todo!();
    }

    pub fn export_wav(&self, wav_settings: WavSettings, path: impl AsRef<Path>) -> Result<(), String> {
        if self.tracks.len() == 0 { return Err("Project must have at least 1 track.".to_string()); }

        let mut wav = Wav {
            NumChannels: wav_settings.num_channels,
            SampleRate: wav_settings.sample_rate,
            BitsPerSample: wav_settings.bytes_per_sample * 8,
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