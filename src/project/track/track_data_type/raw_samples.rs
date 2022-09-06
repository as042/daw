#[derive(Debug, Clone, Default, PartialEq)]
pub struct RawSamples {
    pub(super) samples: Vec<u8>,
    pub(in crate::project) num_channels: i32,
    pub(in crate::project) sample_rate: i32,
    pub(in crate::project) bytes_per_sample: i32
}

impl RawSamples {
    pub(in crate::project) fn new() -> Self {
        Self { samples: Vec::default(), 
            num_channels: 2, 
            sample_rate: 44100, 
            bytes_per_sample: 2 
        }
    }

    pub(crate) fn push_sample(&mut self, sample: u8) {
        self.samples.push(sample);
    }
}