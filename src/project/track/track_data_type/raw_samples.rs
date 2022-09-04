#[derive(Debug, Clone, Default, PartialEq)]
pub struct RawSamples {
    pub(super) samples: Vec<u8>
}

impl RawSamples {
    pub(in crate::project) fn new() -> Self {
        Self { samples: Vec::default() }
    }

    pub(in crate::project) fn push_sample(&mut self, sample: u8) {
        self.samples.push(sample);
    }
}