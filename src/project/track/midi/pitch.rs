use std::str::FromStr;
use method_shorthands::methods::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pitch {
    note_letter: NoteLetter,
    octave: i8
}

#[derive(Deserialize)]
enum NoteLetter {
    A,
    AS,
    BF,
    B,
    BS,
    CF,
    C,
    CS,
    DF,
    D,
    DS,
    EF,
    E,
    ES,
    FF,
    F,
    FS,
    GF,
    G,
    GS,
    AF
}

impl FromStr for Pitch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.to_ascii_uppercase();

        if s.len() < 2 || s.len() > 3 { return Err(String::from("Invalid input")); }

        let octave = match s.chars().last().uw() {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            _ => return Err(String::from("Unsupported octave"))
        };

        s.remove(s.len() - 1);

        let note_letter = match s.as_str() {
            "A" => NoteLetter::A,
            "AS" => NoteLetter::AS,
            "BF" => NoteLetter::BF,
            "B" => NoteLetter::B,
            "BS" => NoteLetter::BS,
            "CF" => NoteLetter::CF,
            "C" => NoteLetter::C,
            "CS" => NoteLetter::CS,
            "DF" => NoteLetter::DF,
            "D" => NoteLetter::D,
            "DS" => NoteLetter::DS,
            "EF" => NoteLetter::EF,
            "E" => NoteLetter::E,
            "ES" => NoteLetter::ES,
            "FF" => NoteLetter::FF,
            "F" => NoteLetter::F,
            "FS" => NoteLetter::FS,
            "GF" => NoteLetter::GF,
            "G" => NoteLetter::G,
            "GS" => NoteLetter::GS,
            "AF" => NoteLetter::AF,
            _ => return Err(String::from("Unsupported note letter"))
        };

        Ok(Pitch { note_letter: note_letter, octave: octave })
    }
}

impl Pitch {
    pub fn to_freq(&mut self) -> f64 {
        2_f64.powf((self.note_num() - 49) as f64 / 12.0) * 440.0
    }

    fn note_num(&self) -> i8 {
        self.octave * 12 + match self.note_letter {
            NoteLetter::A => 1,
            NoteLetter::AS => 2,
            NoteLetter::BF => 2,
            NoteLetter::B => 3,
            NoteLetter::BS => -8,
            NoteLetter::CF => 3,
            NoteLetter::C => -8,
            NoteLetter::CS => -7,
            NoteLetter::DF => -7,
            NoteLetter::D => -6,
            NoteLetter::DS => -5,
            NoteLetter::EF => -5,
            NoteLetter::E => -4,
            NoteLetter::ES => -3,
            NoteLetter::FF => -4,
            NoteLetter::F => -3,
            NoteLetter::FS => -2,
            NoteLetter::GF => -2,
            NoteLetter::G => -1,
            NoteLetter::GS => 0,
            NoteLetter::AF => 0
        }
    }
}