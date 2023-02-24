#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Debug)]
pub struct Time {
    pub start: f64,
    pub end: f64
}

impl Time {
    pub fn new(offset: f64, duration: f64) -> Time {
        Time { start: offset, end: offset + duration }
    }

    pub fn duration(&self) -> f64 {
        self.end - self.start
    }
}