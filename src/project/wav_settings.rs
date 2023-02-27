use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Serialize)]
pub struct WavSettings {
    pub num_channels: usize, 
    pub sample_rate: i32, 
    pub bytes_per_sample: usize 
}

impl WavSettings {
    pub fn block_align(&self) -> usize {
        self.num_channels * self.bytes_per_sample
    }
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