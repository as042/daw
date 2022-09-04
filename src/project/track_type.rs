#[derive(Debug, Clone, Default, PartialEq)]
pub enum TrackType {
    RawSamples,
    Score,
    #[default]
    MIDI
}