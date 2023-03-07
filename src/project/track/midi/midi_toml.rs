use std::{path::Path, fs::OpenOptions, io::Read, str::FromStr, vec, fmt::Display};
use method_shorthands::methods::*;
use serde::Deserialize;

use crate::prelude::{Note, Dynamic, Channels, Instrument, Time};
use super::{MIDI, pitch::Pitch};

#[derive(Deserialize, Clone)]
pub struct TomlNote {
    start: f64,
    duration: f64,
    note: Option<String>,
    notes: Option<Vec<String>>,
    dynamic: Option<Dynamic>,
    instrument: Option<Instrument>,
    channels: Option<TomlChannels>
}

#[derive(Deserialize, Clone, Copy)]
pub enum TomlChannels {
    All,
    Left,
    Right,
    None
}

impl TomlNote {
    fn to_notes(&self) -> Result<Vec<Note>, ()> {
        if self.note.is_some() {
            if self.notes.is_some() { return Err(()) };

            Ok(vec![Note { 
                freq: Pitch::from_str(self.note.as_ref().uw()).uw().to_freq(), 
                velocity: self.dynamic.uw().to_vel(), 
                channels: match self.channels.uw() {
                    TomlChannels::All => Channels::All,
                    TomlChannels::Left => Channels::Just(0),
                    TomlChannels::Right => Channels::Just(1),
                    TomlChannels::None => Channels::None
                }, 
                instrument: self.instrument.uw(), 
                time: Time::new(self.start, self.duration) 
            }])
        }
        else if self.notes.is_some() {
            if self.note.is_some() { return Err(()) }

            let mut vec = Vec::default();
            for note in self.notes.as_ref().uw() {
                vec.push(Note { 
                    freq: Pitch::from_str(&note).uw().to_freq(), 
                    velocity: self.dynamic.uw().to_vel(), 
                    channels: match self.channels.uw() {
                        TomlChannels::All => Channels::All,
                        TomlChannels::Left => Channels::Just(0),
                        TomlChannels::Right => Channels::Just(1),
                        TomlChannels::None => Channels::None
                    }, 
                    instrument: self.instrument.uw(), 
                    time: Time::new(self.start, self.duration) 
                });
            }

            Ok(vec)
        }
        else {
            return Err(())
        }
    }
}

impl MIDI {
    #[deprecated]
    pub fn add_note_from_toml(&mut self, path: impl AsRef<Path> + Display) -> Result<(), &str> {
        if Path::new(&path.to_string()).extension().uw() != "note" { return Err("Invalid type"); }

        let mut file = OpenOptions::new()
            .read(true)
            .open(path).uw();

        let mut toml_data = String::default();
        if file.read_to_string(&mut toml_data).is_err() {
            return Err("Invalid file");
        }

        let toml_note: TomlNote = toml::from_str(&toml_data.ts()).uw();

        self.add_note(toml_note.to_notes().uw()[0]);

        Ok(())
    }

    pub fn add_from_toml(&mut self, path: impl AsRef<Path> + Display, progress_updates: bool) -> Result<(), &str> {
        if let Some(ext) = Path::new(&path.to_string()).extension() {
            if ext.to_ascii_uppercase() != "TRACK" { return Err("Invalid type"); }
        } else { return Err("Invalid type"); }

        if progress_updates { println!("Opening track file."); }

        let file = OpenOptions::new()
            .read(true)
            .open(path);

        if file.is_err() { return Err("Track file not found"); }
        let mut file = file.uw();

        if progress_updates { println!("Reading track file."); }

        let mut toml_data = String::default();
        if file.read_to_string(&mut toml_data).is_err() { return Err("Invalid track file"); }
        toml_data = toml_data.chars().filter(|c| c != &' ').collect();

        if progress_updates { println!("Lexing track file."); }

        if toml_data.find("start=").is_none() { return Err("No TOMLs found in track file") }

        let mut tomls = vec![];
        loop {
            let mut toml_copy = toml_data.clone();
            toml_copy.remove(toml_data.find("start=").uw());
            let find = toml_copy.find("start=");
            if find.is_none() { 
                tomls.push(toml_data);
                break; 
            }

            let idx = find.uw();
            tomls.push(toml_data.clone().split_at(idx).0.ts());
            toml_data.replace_range(0..idx, "");
        }

        if progress_updates { println!("Deserializing track file."); }

        let mut dynamic = Dynamic::MF;
        let mut instrument = Instrument::SubtractiveSynth;
        let mut channels = TomlChannels::All;
        for toml in tomls {
            let de: Result<TomlNote, toml::de::Error> = toml::from_str(toml.as_str());
            // if de.is_err() { return Err("Incorrect TOML data"); }
            let mut toml_note = de.uw();

            if toml_note.dynamic.is_some() { dynamic = toml_note.dynamic.uw() } else { toml_note.dynamic = Some(dynamic) }
            if toml_note.instrument.is_some() { instrument = toml_note.instrument.uw() } else { toml_note.instrument = Some(instrument) }
            if toml_note.channels.is_some() { channels = toml_note.channels.uw() } else { toml_note.channels = Some(channels) }

            let notes = toml_note.to_notes();
            if notes.is_err() { return Err("Incorrect TOML data"); }
            self.notes.append(&mut notes.uw());
        }

        Ok(())
    }
}