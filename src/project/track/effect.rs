use std::{path::Path, fmt::Display, fs::OpenOptions, io::Read};

use method_shorthands::methods::UW;
use serde::{Deserialize, Serialize};

use crate::prelude::{TrackType, Time, Channels, Fade};
use super::TrackData;

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Effect {
    pub effect_type: EffectType,
    pub affected_tracks: Vec<usize>,
    pub channels: Channels,
    pub time: Time
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", content = "args")]
pub enum EffectType {
    Reverb(f64, f64, f64),
    Fade(Fade)
}

impl Default for EffectType {
    fn default() -> Self {
        EffectType::Reverb(0.1, 0.9, 50.0)
    }
}

impl Effect {
    pub fn set_effect(&mut self, effect_type: EffectType, affected_tracks: Vec<usize>, time: Time) {
        self.effect_type = effect_type;
        self.affected_tracks = affected_tracks;
        self.time = time;
    }

    pub fn add_from_toml(&mut self, path: impl AsRef<Path> + Display, progress_updates: bool) -> Result<(), &str> {
        if let Some(ext) = Path::new(&path.to_string()).extension() {
            if ext.to_ascii_uppercase() != "EFFECT" { return Err("Invalid type"); }
        } else { return Err("Invalid type"); }

        if progress_updates { println!("Opening effect file."); }

        let file = OpenOptions::new()
            .read(true)
            .open(path);

        if file.is_err() { return Err("Effect file not found"); }
        let mut file = file.uw();

        if progress_updates { println!("Reading effect file."); }

        let mut toml_data = String::default();
        if file.read_to_string(&mut toml_data).is_err() { return Err("Invalid effect file"); }

        let de: Result<Effect, toml::de::Error> = toml::from_str(&toml_data);
        if de.is_err() { return Err("Incorrect TOML data"); }
        let effect = de.uw();

        *self = effect;

        Ok(())
    }
}

impl TrackData for Effect {
    fn raw_samples(&self) -> &super::RawSamples {
        panic!("Incorrect type.")
    }
    fn midi(&self) -> &super::MIDI {
        panic!("Incorrect type.")
    }
    fn score(&self) -> &super::Score {
        panic!("Incorrect type.")
    }
    fn effect(&self) -> &Effect {
        self
    }
    fn raw_samples_mut(&mut self) -> &mut super::RawSamples {
        panic!("Incorrect type.")
    }
    fn midi_mut(&mut self) -> &mut super::MIDI {
        panic!("Incorrect type.")
    }
    fn score_mut(&mut self) -> &mut super::Score {
        panic!("Incorrect type.")
    }
    fn effect_mut(&mut self) -> &mut Effect {
        self
    }

    fn get_type(&self) -> TrackType {
        TrackType::Effect
    }
    fn is_type(&self, track_type: TrackType) -> bool {
        if track_type == TrackType::Effect {
            return true;
        }

        false
    }
}