#![allow(dead_code, non_snake_case)]

pub mod track;

use track::*;

use self::track::track_type::TrackType;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Project {
    tracks: Vec<Track>
}

impl Project {
    pub fn new() -> Project {
        Project { tracks: Vec::default() }
    }

    pub fn new_track(&mut self, track_type: TrackType) {
        self.tracks.push(Track {
            track_type: track_type,
            data: Vec::default()
        });
    }

    pub fn export_MIDI(&mut self) -> Result<(), String> {
        if self.tracks.iter().any(|x| x.track_type != TrackType::MIDI) { return Err("Not all tracks are MIDI type.".to_string()); }

        todo!();
    }
}