use crate::project::TrackType;
use super::TrackData;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Score {
    pub(super) samples: Vec<u8>
}

impl Score {
    pub(crate) fn samples(&self) -> &Vec<u8> {
        &self.samples
    }
}

impl TrackData for Score {
    fn get_type(&self) -> TrackType {
        TrackType::Score
    }

    fn is_type(&self, track_type: TrackType) -> bool {
        if track_type == TrackType::Score {
            return true;
        }

        false
    }

    fn score(&self) -> &Score {
        &self
    }
    fn score_mut(&mut self) -> &mut Score {
        &mut self
    }

    fn raw_samples(&self) -> &super::RawSamples { panic!("Incorrect type."); }
    fn midi(&self) -> &super::MIDI { panic!("Incorrect type."); }
    fn raw_samples_mut(&mut self) -> &mut super::RawSamples { panic!("Incorrect type."); }
    fn midi_mut(&mut self) -> &mut super::MIDI { panic!("Incorrect type."); }
}