use std::mem::discriminant;

use super::track_type::*;

pub mod track_data_type;

pub use track_data_type::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Track {
    pub(crate) data: TrackDataType
}

impl Track {
    pub(crate) fn is_type(&self, track_type: TrackType) -> bool {
        let mut data_type = TrackDataType::default();

        if track_type == TrackType::RawSamples { data_type = TrackDataType::RawSamples(RawSamples::default()) }
        if track_type == TrackType::Score { data_type = TrackDataType::Score(Score::default()) }
        if track_type == TrackType::MIDI { data_type = TrackDataType::default() }


        if discriminant(&self.data) == discriminant(&data_type) {
            return true;
        }

        false
    }

    pub(crate) fn len(&self) -> usize {
        let data = self.data;

        if let Ok(raw_samples) = data.raw_samples() {
            return raw_samples.samples().len();
        }
        if let Ok(score) = data.score() {
            return score.samples().len();
        }
        if let Ok(midi) = data.midi() {
            return midi.samples().len();
        }

        0
    }
}