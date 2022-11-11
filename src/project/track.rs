use super::track_type::*;

pub mod track_data;

pub use track_data::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Track<T: TrackData + Default> {
    pub data: T
}

impl<T: TrackData + Default> Track<T> {
    pub fn is_type(&self, track_type: TrackType) -> bool {
        self.data.is_type(track_type)
    }

    pub fn len(&self) -> usize {
        let data = &self.data;

        return match self.data.get_type() {
            TrackType::RawSamples => self.data.raw_samples().samples().len(),
            TrackType::Score => 0,
            TrackType::MIDI => 0
        };
    }
}

// impl Track<RawSamples> {
//     pub fn new() -> Track<RawSamples> {
//         Track { data: RawSamples::default() }
//     }
// }
// impl Track<Score> {
//     pub fn new() -> Track<Score> {
//         Track { data: Score::default() }
//     }
// }
// impl Track<MIDI> {
//     pub fn new() -> Track<MIDI> {
//         Track { data: MIDI::default() }
//     }
// }