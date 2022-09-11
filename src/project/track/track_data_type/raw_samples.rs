use crate::project::WavSettings;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RawSamples {
    samples: Vec<u8>,
    pub(crate) settings: WavSettings
}

impl RawSamples {
    pub(in crate::project) fn new() -> Self {
        Self { 
            samples: Vec::default(), 
            settings: WavSettings::default()
        }
    }

    pub(crate) fn samples(&self) -> &Vec<u8> {
        &self.samples
    }

    /// Pushes the given 2-byte sample to the data twice for stereo.
    pub(crate) fn push_sample(&mut self, sample: &[u8; 2]) {
        self.samples.extend_from_slice(sample);
        self.samples.extend_from_slice(sample);
    }
}