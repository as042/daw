#![allow(dead_code, non_snake_case)]

use std::mem::discriminant;

pub mod track;
pub mod track_type;

pub use track::*;
pub use track_type::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Project {
    pub(crate) tracks: Vec<Track>
}

impl Project {
    pub fn new() -> Self {
        Self { tracks: Vec::default() }
    }

    pub fn tracks(&self) -> &Vec<Track> { &self.tracks }

    pub fn new_track(&mut self, track_type: TrackType) {
        let mut track = Track {
            data: TrackDataType::default()
        };

        if track_type == TrackType::RawSamples { track.data = TrackDataType::RawSamples(RawSamples::new()) }
        if track_type == TrackType::MIDI { track.data = TrackDataType::default() }
        if track_type == TrackType::Score { track.data = TrackDataType::Score(Score::default()) }

        self.tracks.push(track);
    }

    pub fn export_midi(&self) -> Result<(), String> {
        if self.tracks.len() == 0 { return Err("Project must have at least 1 track.".to_string()); }
        if self.tracks.iter().any(|x| discriminant(&x.data) == discriminant(&TrackDataType::RawSamples(Default::default()))) { return Err("Not all tracks are MIDI type.".to_string()); }

        todo!();
    }

    pub fn export_wav(&self) -> Result<(), String> {
        if self.tracks.len() == 0 { return Err("Project must have at least 1 track.".to_string()); }

        todo!();
    }
}