#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Fade {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) fade_type: FadeType,
    pub(super) fade_out: bool
}

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub enum FadeType {
    #[default]
    Linear,
    Power(f64),
    NegPower(f64)
}

impl Fade {
    pub fn new(offset: f64, duration: f64, fade_type: FadeType, fade_out: bool, sample_rate: i32) -> Fade {
        Fade { 
            start: (offset * sample_rate as f64) as usize,
            end: ((offset + duration) * sample_rate as f64) as usize, 
            fade_type: fade_type,
            fade_out: fade_out
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
}