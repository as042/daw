use std::{path::Path, fmt::{Display, Debug, Formatter}, fs::OpenOptions, io::Read};
use method_shorthands::methods::UW;
use serde::Deserialize;

use crate::prelude::{TrackType, Time, Channels, Fade, FadeType};
use super::TrackData;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Effect {
    pub effect_type: EffectType,
    pub affected_tracks: Vec<usize>,
    pub channels: Channels,
    pub time: Time
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EffectType {
    Reverb(f64, f64, f64),
    Fade(Fade)
}

impl Display for EffectType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            EffectType::Reverb(del, dec, mix) => Debug::fmt(&EffectType::Reverb(*del, *dec, *mix), f),
            EffectType::Fade(_) => write!(f, "Fade")
        }
    }
}

impl Default for EffectType {
    fn default() -> Self {
        EffectType::Reverb(0.1, 0.9, 50.0)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
struct TomlEffect {
    effect_type: TomlEffectType,
    reverb_settings: Option<TomlReverbArgs>,
    fade_settings: Option<TomlFadeArgs>,
    affected_tracks: Vec<usize>,
    start: f64,
    duration: f64,
    channels: Channels
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Deserialize)]
struct TomlReverbArgs {
    delay: f64,
    decay_factor: f64,
    mix_percentage: f64
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Deserialize)]
struct TomlFadeArgs {
    fade_type: TomlFadeType,
    fade_direction: TomlFadeDirection
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Deserialize)]
enum TomlFadeType {
    #[default]
    Linear,
    Quadratic,
    NegativeQuadratic
}

impl TomlFadeType {
    fn to_fade_type(&self) -> FadeType {
        return match self {
            TomlFadeType::Linear => FadeType::Linear,
            TomlFadeType::Quadratic => FadeType::Power(2.0),
            TomlFadeType::NegativeQuadratic => FadeType::NegPower(2.0),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Deserialize)]
enum TomlFadeDirection {
    #[default]
    In,
    Out
}

impl TomlFadeDirection {
    fn to_fade_out(&self) -> bool {
        if self == &TomlFadeDirection::Out { 
            return true;
        }

        false
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq)]
enum TomlEffectType {
    #[default]
    Reverb,
    Fade
}

impl TomlEffect {
    fn to_effect(&self) -> Result<Effect, ()> {
        let effect_type = if self.effect_type == TomlEffectType::Reverb {
            if self.reverb_settings.is_none() { return Err(()) }
            if self.fade_settings.is_some() { return Err(()) }

            let settings = self.reverb_settings.uw();
            EffectType::Reverb(settings.delay, settings.decay_factor, settings.mix_percentage)
        }
        else if self.effect_type == TomlEffectType::Fade {
            if self.reverb_settings.is_some() { return Err(()) }
            if self.fade_settings.is_none() { return Err(()) }
            
            let settings = self.fade_settings.uw();
            EffectType::Fade(Fade::new(
                settings.fade_type.to_fade_type(), 
                settings.fade_direction.to_fade_out(), 
                Time::new(self.start, self.duration)))
        }
        else {
            return Err(())
        };
        Ok(Effect { 
            effect_type: effect_type, 
            affected_tracks: self.affected_tracks.clone(), 
            channels: self.channels, 
            time: Time::new(self.start, self.duration)
        })
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

        let de: Result<TomlEffect, toml::de::Error> = toml::from_str(&toml_data);
        if de.is_err() { return Err("Incorrect TOML data"); }
        let toml_effect = de.uw();

        let effect = toml_effect.to_effect();
        if effect.is_err() { return Err("Incorrect TOML data"); }
        *self = effect.uw();

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