use crate::prelude::{MIDI, TrackType, Time};
use super::TrackData;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Effect {
    pub(super) effect_type: EffectType,
    pub(super) tracks: Vec<usize>,
    pub(super) time: Time
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum EffectType {
    #[default]
    Reverb
}

impl TrackData for Effect {
    fn raw_samples(&self) -> &super::RawSamples {
        panic!("Incorrect type.")
    }
    fn midi(&self) -> &MIDI {
        panic!("Incorrect type.")
    }
    fn score(&self) -> &super::Score {
        panic!("Incorrect type.")
    }
    fn effect(&self) -> &Effect {
        self
    }
    fn raw_samples_mut(&mut self) -> &mut super::RawSamples {
        panic!("Incorrect type.")
    }
    fn midi_mut(&mut self) -> &mut MIDI {
        panic!("Incorrect type.")
    }
    fn score_mut(&mut self) -> &mut super::Score {
        panic!("Incorrect type.")
    }
    fn effect_mut(&mut self) -> &mut Effect {
        self
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

impl Effect {
    
}