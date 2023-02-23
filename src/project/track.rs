pub mod raw_samples;
pub mod midi;
pub mod score;

use std::fmt::Debug;

use super::track_type::*;
use raw_samples::*;
use midi::*;
use score::*;

pub trait TrackData {
    fn raw_samples(&self) -> &RawSamples;
    fn midi(&self) -> &MIDI;
    fn score(&self) -> &Score; 
    fn raw_samples_mut(&mut self) -> &mut RawSamples;
    fn midi_mut(&mut self) -> &mut MIDI;
    fn score_mut(&mut self) -> &mut Score; 
    fn get_type(&self) -> TrackType;
    fn is_type(&self, track_type: TrackType) -> bool;
}

pub struct Track {
    pub(super) data: Box<dyn TrackData>
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Debug for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Track").field("data", &self).finish()
    }
}

impl Default for Track {
    fn default() -> Self {
        Self { data: Box::new(RawSamples::default()) }
    }
}

impl Track {
    pub fn raw_samples(&self) -> &RawSamples {
        self.data.raw_samples()
    }
    pub fn midi(&self) -> &MIDI {
        self.data.midi()
    }
    pub fn score(&self) -> &Score {
        self.data.score()
    }
    pub fn raw_samples_mut(&mut self) -> &mut RawSamples {
        self.data.raw_samples_mut()
    }
    pub fn midi_mut(&mut self) -> &mut MIDI {
        self.data.midi_mut()
    }
    pub fn score_mut(&mut self) -> &mut Score {
        self.data.score_mut()
    }

    pub fn is_type(&self, track_type: TrackType) -> bool {
        self.data.is_type(track_type)
    }
    pub fn get_type(&self) -> TrackType {
        self.data.get_type()
    }

    pub fn len(&self) -> usize {
        if self.is_type(TrackType::RawSamples) {
            return self.data.raw_samples().samples()[0].len();
        }
        else if self.is_type(TrackType::MIDI) {
            return self.data.midi().notes().len();
        }
        else if self.is_type(TrackType::Score) {
            return self.data.score().samples().len();
        }

        0
    }
}