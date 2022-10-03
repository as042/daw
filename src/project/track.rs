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
        self.data.is_type(track_type)
    }

    pub(crate) fn len(&self) -> usize {
        let data = &self.data;

        if let Ok(raw_samples) = data.raw_samples() {
            return raw_samples.samples().len();
        }
        else if let Ok(score) = data.score() {
            return score.samples().len();
        }
        else if let Ok(midi) = data.midi() {
            return midi.samples().len();
        }

        0
    }
}