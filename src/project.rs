#![allow(dead_code, non_snake_case)]
pub mod track;
pub mod track_type;
pub mod wav_settings;
pub mod wav_writer;
pub mod wave;
pub mod time;

use std::{fs::OpenOptions, path::Path, io::{Write, Read}, fmt::Display};
pub use method_shorthands::methods::*;

use serde::Deserialize;
pub use track::*;
pub use track_type::*;
pub use wav_settings::*;
pub use wav_writer::*;
pub use wave::*;
use self::{track::{raw_samples::RawSamples, score::Score, midi::MIDI}, effect::Effect};

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Project {
    pub tracks: Vec<Track>,
    progress_updates: bool
}

impl Project {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_track(&mut self, track_type: TrackType) -> &mut Track {
        let mut track = Track::default();

        track.data = match track_type {
            TrackType::RawSamples => Box::new(RawSamples::default()),
            TrackType::MIDI => Box::new(MIDI::default()),
            TrackType::Score => Box::new(Score::default()),
            TrackType::Filter => Box::new(Effect::default())
        };

        self.tracks.push(track);

        let len = self.tracks.len();
        &mut self.tracks[len - 1]
    }

    pub fn track(&mut self, track_type: TrackType, rank: usize) -> Result<&mut Track, String> {
        let mut count = 0;
        for k in 0..self.tracks.len() {
            if self.tracks[k].is_type(track_type) {
                if count == rank {
                    return Ok(&mut self.tracks[k]);
                }

                count += 1;
            }
        }

        Err("Cannot find specific track.".ts())
    }

    pub fn export_midi(&self) -> Result<(), String> {
        if self.tracks.len() == 0 { return Err("Project must have at least 1 track.".ts()); }
        if self.tracks.iter().any(|x| x.is_type(TrackType::RawSamples)) { return Err("Tracks cannot be RawSamples type.".ts()); }

        todo!();
    }

    /// Export Project to .wav file
    pub fn export_wav(&self, wav_settings: WavSettings, path: impl AsRef<Path>, progress_updates: bool) -> Result<(), String> {
        if self.tracks.len() == 0 { return Err("Project must have at least 1 track.".ts()); }

        if progress_updates { println!("Initializing wav data.") };

        let mut wav = Wav {
            num_channels: wav_settings.num_channels,
            sample_rate: wav_settings.sample_rate,
            bits_per_sample: wav_settings.bytes_per_sample * 8,
            ..Default::default()
        };

        let wav_vector = wav.create_wav(self, progress_updates);

        if progress_updates { println!("Creating or opening wav file.") };

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path).uw();

        if progress_updates { println!("Writing data to wav file.") };

        file.write_all(&wav_vector).uw();

        if progress_updates { println!("Operation successful.") };

        Ok(())
    }

    /// Create Project from .project file
    pub fn from_toml(path: impl AsRef<Path> + Display, progress_updates: bool) -> Result<(Self, WavSettings, String), String> {
        let p = &path.to_string();
        if let Some(path) = Path::new(p).extension() {
            if path != "project" { return Err("Invalid type".ts()); }
        } else { return Err("Invalid type".ts()); }

        if progress_updates { println!("Opening project file."); }

        let file = OpenOptions::new()
            .read(true)
            .open(path);

        if file.is_err() { return Err("Project file not found".ts()); }
        let mut file = file.uw();

        let mut file_location = p.clone();
        file_location.replace_range(p.find(Path::new(p).file_name().uw().to_str().uw()).uw()..p.len(), "");
        let export_file_name = Path::new(p).file_stem().uw().to_str().uw().ts();

        if progress_updates { println!("Reading project file."); }

        let mut toml_data = String::default();
        if file.read_to_string(&mut toml_data).is_err() { return Err("Invalid project file".ts()); }

        if progress_updates { println!("Deserializing project file."); }

        let de: Result<TomlProject, toml::de::Error> = toml::from_str(toml_data.as_str());
        if de.is_err() { return Err("Incorrect TOML data in project file".ts()); }

        if progress_updates { println!("Interpreting project data."); }

        let toml_project = de.uw();
        let project = toml_project.to_project(file_location, progress_updates);
        if project.is_err() { return Err(project.unwrap_err()); }

        Ok((project.uw(), toml_project.settings, export_file_name + ".wav"))
    }

    /// Create project from files inputted through console and export to wav.
    pub fn from_console_input() {
        let mut path = String::default();
        std::io::stdin().read_line(&mut path).uw();
    
        let from_toml = Project::from_toml(path.trim(), true);
        if let Ok(output) = from_toml {
            output.0.export_wav(output.1, output.2, true).uw();
        }  
        else {
            println!("Error: {}. \nOperation unsuccessful.", from_toml.unwrap_err());
        }
    
        std::io::stdin().read_line(&mut path).uw();
    }
}

#[derive(Deserialize, Clone)]
struct TomlProject {
    settings: WavSettings,
    tracks: Vec<String>
}

impl TomlProject {
    pub fn to_project(&self, project_location: String, progress_updates: bool) -> Result<Project, String> {
        let mut project = Project { progress_updates: true, ..Default::default() };
        for track_path in &self.tracks {
            let track = project.new_track(TrackType::MIDI);

            let res = track.midi_mut().add_from_toml(project_location.clone() + track_path, progress_updates);
            if res.is_err() { return Err(res.unwrap_err().ts()); }
        }

        Ok(project)
    }
}