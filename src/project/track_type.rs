#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TrackType {
    RawSamples,
    Score,
    #[default]
    MIDI
}