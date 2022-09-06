use std::mem::discriminant;
use enum_as_inner::EnumAsInner;
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
    pub fn raw_samples(&mut self) -> Result<&mut RawSamples, String> {
        if self.is_type(TrackType::RawSamples) {
            return Ok(self.as_raw_samples_mut().unwrap())
        }

        Err("Track is not of type RawSamples.".to_string())
    }
    pub fn score(&mut self) -> Result<&mut Score, String> {
        if self.is_type(TrackType::Score) {
            return Ok(self.as_score_mut().unwrap())
        }

        Err("Track is not of type Score.".to_string())
    }
    pub fn midi(&mut self) -> Result<&mut MIDI, String> {
        if self.is_type(TrackType::MIDI) {
            return Ok(self.as_midi_mut().unwrap())
        }

        Err("Track is not of type MIDI.".to_string())
    }

    pub(crate) fn is_type(&self, track_type: TrackType) -> bool {
        let mut data_type = TrackDataType::default();

        if track_type == TrackType::RawSamples { data_type = TrackDataType::RawSamples(RawSamples::default()) }
        if track_type == TrackType::MIDI { data_type = TrackDataType::default() }
        if track_type == TrackType::Score { data_type = TrackDataType::Score(Score::default()) }

        if discriminant(self) == discriminant(&data_type) {
            return true;
        }

        false
    }
}