#[derive(Debug, Clone, Default, PartialEq)]
pub struct Score {
    pub(super) samples: Vec<u8>
}

impl Score {
    pub(crate) fn samples(&self) -> &Vec<u8> {
        &self.samples
    }
}