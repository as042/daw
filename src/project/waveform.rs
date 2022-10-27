use enum_as_inner::EnumAsInner;

pub mod wave;
use wave::*;

#[derive(Debug, Clone, PartialEq, EnumAsInner)]
pub enum Waveform {
    Sine(Wave),
    Square(Wave),
    Triangle(Wave),
    Sawtooth(Wave)    
}

impl Default for Waveform {
    fn default() -> Self { Self::Sine(Wave::default()) }
}