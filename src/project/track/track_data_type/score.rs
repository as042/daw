#[derive(Debug, Clone, Default, PartialEq)]
pub struct Score {
    pub(super) samples: Vec<u8>
}

impl Score {
    pub(in crate::project) fn new() -> Self {
        Self { samples: Vec::default() }
    }
}