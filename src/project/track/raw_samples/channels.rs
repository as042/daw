use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Channels {
    #[default]
    All,
    None,
    Just(usize),
    AllBut(usize)
}