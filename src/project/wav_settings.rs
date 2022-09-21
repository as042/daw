#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WavSettings {
    pub(crate) num_channels: usize, 
    pub(crate) sample_rate: i32, 
    pub(crate) bytes_per_sample: i32 
}

impl Default for WavSettings {
    fn default() -> Self {
        Self { 
            num_channels: 2, 
            sample_rate: 44100, 
            bytes_per_sample: 2
        }
    }
}