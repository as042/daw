use self::track_type::TrackType;

pub mod track_type;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Track {
    pub(super) track_type: TrackType,
    pub(super) data: Vec<u8>
}