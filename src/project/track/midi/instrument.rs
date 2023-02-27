use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Instrument {
    #[default]
    SubtractiveSynth
}