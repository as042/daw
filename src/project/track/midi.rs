pub mod note;

use note::*;
use crate::prelude::{TrackData, TrackType};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MIDI {
    pub(super) notes: Vec<Note>
}

impl TrackData for MIDI {
    fn raw_samples(&self) -> &super::RawSamples {
        panic!("Incorrect type.")
    }
    fn midi(&self) -> &MIDI {
        self
    }
    fn score(&self) -> &super::Score {
        panic!("Incorrect type.")
    }
    fn raw_samples_mut(&mut self) -> &mut super::RawSamples {
        panic!("Incorrect type.")
    }
    fn midi_mut(&mut self) -> &mut MIDI {
        self
    }
    fn score_mut(&mut self) -> &mut super::Score {
        panic!("Incorrect type.")
    }
    fn get_type(&self) -> TrackType {
        TrackType::MIDI
    }
    fn is_type(&self, track_type: TrackType) -> bool {
        if track_type == TrackType::MIDI {
            return true;
        }

        false
    }
}

impl MIDI {
    pub fn notes(&self) -> &Vec<Note> {
        &self.notes
    }
}