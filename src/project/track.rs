pub mod raw_samples;
pub mod midi;
pub mod score;
pub mod effect;

use std::fmt::Debug;

use self::effect::Effect;
use super::track_type::*;
use raw_samples::*;
use midi::*;
use score::*;

pub trait TrackData: TrackDataClone + Debug {
    fn raw_samples(&self) -> &RawSamples;
    fn midi(&self) -> &MIDI;
    fn score(&self) -> &Score; 
    fn effect(&self) -> &Effect; 
    fn raw_samples_mut(&mut self) -> &mut RawSamples;
    fn midi_mut(&mut self) -> &mut MIDI;
    fn score_mut(&mut self) -> &mut Score; 
    fn effect_mut(&mut self) -> &mut Effect; 
    fn get_type(&self) -> TrackType;
    fn is_type(&self, track_type: TrackType) -> bool;
}

pub trait TrackDataClone {
    fn clone_box(&self) -> Box<dyn TrackData>;
}

impl<T> TrackDataClone for T
where
    T: 'static + TrackData + Clone,
{
    fn clone_box(&self) -> Box<dyn TrackData> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn TrackData> {
    fn clone(&self) -> Box<dyn TrackData> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Track {
    pub(super) data: Box<dyn TrackData>
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        if self.get_type() == other.get_type() {
            return match self.get_type() {
                TrackType::RawSamples => self.data.raw_samples() == other.data.raw_samples(),
                TrackType::Score => self.data.score() == other.data.score(),
                TrackType::MIDI => self.data.midi() == other.data.midi(),
                TrackType::Effect => self.data.effect() == other.data.effect(),
            }
        }

        false
    }
}

impl Debug for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Track").field("data", &self.data).finish()
    }
}

impl Default for Track {
    fn default() -> Self {
        Self { data: Box::new(RawSamples::default()) }
    }
}

impl Track {
    #[inline]
    pub fn raw_samples(&self) -> &RawSamples {
        self.data.raw_samples()
    }
    #[inline]
    pub fn midi(&self) -> &MIDI {
        self.data.midi()
    }
    #[inline]
    pub fn score(&self) -> &Score {
        self.data.score()
    }
    #[inline]
    pub fn effect(&self) -> &Effect {
        self.data.effect()
    }
    #[inline]
    pub fn raw_samples_mut(&mut self) -> &mut RawSamples {
        self.data.raw_samples_mut()
    }
    #[inline]
    pub fn midi_mut(&mut self) -> &mut MIDI {
        self.data.midi_mut()
    }
    #[inline]
    pub fn score_mut(&mut self) -> &mut Score {
        self.data.score_mut()
    }
    #[inline]
    pub fn effect_mut(&mut self) -> &mut Effect {
        self.data.effect_mut()
    }

    #[inline]
    pub fn is_type(&self, track_type: TrackType) -> bool {
        self.data.is_type(track_type)
    }
    #[inline]
    pub fn get_type(&self) -> TrackType {
        self.data.get_type()
    }

    #[inline]
    pub fn len(&self) -> usize {
        if self.is_type(TrackType::RawSamples) {
            self.data.raw_samples().samples().len()
        }
        else if self.is_type(TrackType::MIDI) {
            self.data.midi().notes().len()
        }
        else if self.is_type(TrackType::Score) {
            self.data.score().samples().len()
        }
        else {
            1
        }
    }
    #[inline]
    pub fn size(&self, block_align: usize, sample_rate: i32) -> usize {
        if self.is_type(TrackType::RawSamples) {
            self.data.raw_samples().samples().iter().map(|x| x.len()).max().uw() * block_align
        }
        else if self.is_type(TrackType::MIDI) {
            (self.data.midi().notes().iter().map(|n| n.time.end).fold(0_f64, |a, b| a.max(b)) * sample_rate as f64) as usize * block_align
        }
        else if self.is_type(TrackType::Score) {
            self.len()
        }
        else {
            self.len()
        }
    }
}