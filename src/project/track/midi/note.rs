#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Debug)]
pub struct Note {
    pitch: f64,
    velocity: f64,
    channel: usize,
    instrument: i64,
    time_start: f64,
    time_stop: f64
}