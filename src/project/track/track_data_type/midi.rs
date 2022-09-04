#[derive(Debug, Clone, Default, PartialEq)]
pub struct MIDI {
    pub(super) samples: Vec<u8>
}

impl MIDI {
    pub(in crate::project) fn new() -> Self {
        Self { samples: Vec::default() }
    }
}