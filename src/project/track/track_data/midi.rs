use crate::project::TrackType;
use super::TrackData;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MIDI {
    pub(super) samples: Vec<u8>
}

impl MIDI {
    pub(crate) fn samples(&self) -> &Vec<u8> {
        &self.samples
    }
}

impl TrackData for MIDI {
    fn get_type(&self) -> TrackType {
        TrackType::MIDI
    }

    fn is_type(&self, track_type: TrackType) -> bool {
        if track_type == TrackType::MIDI {
            return true;
        }

        false
    }

    fn midi(&self) -> &MIDI {
        self
    }
    fn midi_mut(&mut self) -> &mut MIDI {
        &mut self
    }

    fn score(&self) -> &super::Score { panic!("Incorrect type."); }
    fn raw_samples(&self) -> &super::RawSamples { panic!("Incorrect type."); }
    fn score_mut(&mut self) -> &mut super::Score { panic!("Incorrect type."); }
    fn raw_samples_mut(&mut self) -> &mut super::RawSamples { panic!("Incorrect type."); }
}