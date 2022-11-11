use crate::project::track_type::{*, self};

pub mod raw_samples;
pub mod midi;
pub mod score;

pub use raw_samples::*;
pub use midi::*;
pub use score::*;

pub trait TrackData {
    fn get_type(&self) -> TrackType;

    fn is_type(&self, track_type: TrackType) -> bool;

    fn raw_samples(&self) -> &RawSamples;
    fn score(&self) -> &Score;
    fn midi(&self) -> &MIDI;

    fn raw_samples_mut(&mut self) -> &mut RawSamples;
    fn score_mut(&mut self) -> &mut Score;
    fn midi_mut(&mut self) -> &mut MIDI;
}

// impl TrackDataType {
//     pub fn raw_samples(&self) -> Result<&RawSamples, String> {
//         if self.is_type(TrackType::RawSamples) {
//             return Ok(self.as_raw_samples().unwrap())
//         }

//         Err("Track is not of type RawSamples.".to_string())
//     }
//     pub fn score(&self) -> Result<&Score, String> {
//         if self.is_type(TrackType::Score) {
//             return Ok(self.as_score().unwrap())
//         }

//         Err("Track is not of type Score.".to_string())
//     }
//     pub fn midi(&self) -> Result<&MIDI, String> {
//         if self.is_type(TrackType::MIDI) {
//             return Ok(self.as_midi().unwrap())
//         }

//         Err("Track is not of type MIDI.".to_string())
//     }

//     pub fn raw_samples_mut(&mut self) -> Result<&mut RawSamples, String> {
//         if self.is_type(TrackType::RawSamples) {
//             return Ok(self.as_raw_samples_mut().unwrap())
//         }

//         Err("Track is not of type RawSamples.".to_string())
//     }
//     pub fn score_mut(&mut self) -> Result<&mut Score, String> {
//         if self.is_type(TrackType::Score) {
//             return Ok(self.as_score_mut().unwrap())
//         }

//         Err("Track is not of type Score.".to_string())
//     }
//     pub fn midi_mut(&mut self) -> Result<&mut MIDI, String> {
//         if self.is_type(TrackType::MIDI) {
//             return Ok(self.as_midi_mut().unwrap())
//         }

//         Err("Track is not of type MIDI.".to_string())
//     }

//     pub(crate) fn is_type(&self, track_type: TrackType) -> bool {
//         let data_type = match track_type {
//             TrackType::RawSamples => TrackDataType::RawSamples(RawSamples::default()),
//             TrackType::Score => TrackDataType::Score(Score::default()),
//             TrackType::MIDI => TrackDataType::MIDI(MIDI::default())
//         };

//         if discriminant(self) == discriminant(&data_type) {
//             return true;
//         }

//         false
//     }
// }