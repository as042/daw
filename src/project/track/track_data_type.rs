use std::mem::discriminant;
use enum_as_inner::EnumAsInner;
use method_shorthands::*;
use crate::project::track_type::*;

pub mod raw_samples;
pub mod midi;
pub mod score;

pub use raw_samples::*;
pub use midi::*;
pub use score::*;

#[derive(Debug, Clone, PartialEq, EnumAsInner)]
pub enum TrackDataType {
    RawSamples(RawSamples),
    Score(Score),
    MIDI(MIDI)
}

impl Default for TrackDataType {
    fn default() -> Self { Self::MIDI(MIDI::default()) }
}

impl TrackDataType {
    pub fn raw_samples(&self) -> Result<&RawSamples, String> {
        if self.is_type(TrackType::RawSamples) {
            return Ok(self.as_raw_samples().uw())
        }

        Err("Track is not of type RawSamples.".ts())
    }
    pub fn score(&self) -> Result<&Score, String> {
        if self.is_type(TrackType::Score) {
            return Ok(self.as_score().uw())
        }

        Err("Track is not of type Score.".ts())
    }
    pub fn midi(&self) -> Result<&MIDI, String> {
        if self.is_type(TrackType::MIDI) {
            return Ok(self.as_midi().uw())
        }

        Err("Track is not of type MIDI.".ts())
    }

    pub fn raw_samples_mut(&mut self) -> Result<&mut RawSamples, String> {
        if self.is_type(TrackType::RawSamples) {
            return Ok(self.as_raw_samples_mut().uw())
        }

        Err("Track is not of type RawSamples.".ts())
    }
    pub fn score_mut(&mut self) -> Result<&mut Score, String> {
        if self.is_type(TrackType::Score) {
            return Ok(self.as_score_mut().uw())
        }

        Err("Track is not of type Score.".ts())
    }
    pub fn midi_mut(&mut self) -> Result<&mut MIDI, String> {
        if self.is_type(TrackType::MIDI) {
            return Ok(self.as_midi_mut().uw())
        }

        Err("Track is not of type MIDI.".ts())
    }

    pub(crate) fn is_type(&self, track_type: TrackType) -> bool {
        let data_type = match track_type {
            TrackType::RawSamples => TrackDataType::RawSamples(RawSamples::default()),
            TrackType::Score => TrackDataType::Score(Score::default()),
            TrackType::MIDI => TrackDataType::MIDI(MIDI::default())
        };

        if discriminant(self) == discriminant(&data_type) {
            return true;
        }

        false
    }
}