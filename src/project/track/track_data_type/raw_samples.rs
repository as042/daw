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

    pub(crate) fn push_sample(&mut self, sample: u8) {
        self.samples.push(sample);
    }
}