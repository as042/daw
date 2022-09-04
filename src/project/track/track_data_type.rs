pub mod raw_samples;
pub mod midi;
pub mod score;

use raw_samples::*;
use midi::*;
use score::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TrackDataType {
    RawSamples(RawSamples),
    Score(Score),
    MIDI(MIDI)
}

impl Default for TrackDataType {
    fn default() -> Self { Self::MIDI(MIDI::default()) }
}