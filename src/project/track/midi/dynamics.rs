use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dynamic {
    PPPP,
    PPP,
    PP,
    P,
    MP,
    #[default]
    MF,
    F,
    FF,
    FFF,
    FFFF
}

impl Dynamic {
    pub fn to_vel(&self) -> u8 {
        match self {
            Self::PPPP => 8,
            Self::PPP => 20,
            Self::PP => 31,
            Self::P => 42,
            Self::MP => 53,
            Self::MF => 64,
            Self::F => 80,
            Self::FF => 96,
            Self::FFF => 112,
            Self::FFFF => 127,
        }
    }
}