#[derive(Debug, Clone, Default, PartialEq)]
pub struct MIDI {
    pub(super) samples: Vec<u8>
}

impl MIDI {
    pub(crate) fn samples(&self) -> &Vec<u8> {
        &self.samples
    }
}