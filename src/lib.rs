pub mod project;
pub mod note_freqs;
#[allow(dead_code, unused_imports, unused_variables)]
mod tests;

pub mod prelude {
    pub use crate::{project::{*, time::*, raw_samples::{*, channels::*, modifiers::*, fade::*, reverb::*}, midi::{*, instrument::*, note::*, dynamics::*, toml::*}, score::*, effect::*}, note_freqs::*};
}