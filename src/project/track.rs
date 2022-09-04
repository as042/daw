pub mod track_data_type;

use track_data_type::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Track {
    pub(super) data: TrackDataType
}