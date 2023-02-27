use serde::{Deserialize, Serialize};

use crate::prelude::{Time, Instrument, Channels};

#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct Note {
    pub freq: f64,
    pub velocity: u8,
    pub channels: Channels,
    pub instrument: Instrument,
    pub time: Time
}