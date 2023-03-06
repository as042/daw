use std::mem::discriminant;

use crate::prelude::{Time, Channels};
use super::RawSamples;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Fade {
    pub fade_type: FadeType,
    pub fade_out: bool,
    pub time: Time
}

impl Fade {
    pub fn new(fade_type: FadeType, fade_out: bool, time: Time) -> Fade {
        Fade { 
            fade_type, 
            fade_out, 
            time
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub enum FadeType {
    #[default]
    Linear,
    Power(f64),
    NegPower(f64)
}

impl RawSamples {
    pub fn fade(&mut self, fade: Fade, channels: Channels) {
        for j in 0..self.settings.num_channels {
            if channels == Channels::All || 
                channels == Channels::Just(j) ||
                (discriminant(&channels) == discriminant(&Channels::AllBut(1)) && channels != Channels::AllBut(j))
            {
                let mut vec = self.samples[j].clone();

                Self::fade_vec(&mut vec, &vec![fade], self.settings.sample_rate);

                let range = (fade.time.start * self.settings.sample_rate as f64) as usize..
                    (fade.time.end * self.settings.sample_rate as f64) as usize;
                self.samples[j].splice(range.clone(), vec[range].to_vec());
            }
        }
    }
    /// Applies the given fades to the inputted vector.
    pub fn fade_vec(buffer: &mut Vec<f64>, fades: &Vec<Fade>, sample_rate: i32) {
        for fade in fades {
            let mut pow = 1.0;
            if let FadeType::Power(exp) = fade.fade_type {
                pow = exp;
            }
            else if let FadeType::NegPower(exp) = fade.fade_type {
                pow = exp;
            }

            let start_idx = (fade.time.start * sample_rate as f64) as usize;
            let end_idx = (fade.time.end * sample_rate as f64) as usize;
            let len = end_idx - start_idx;
            *buffer = buffer.iter().enumerate()
                .map(|s| 
                    if (start_idx..end_idx).contains(&s.0) { 
                        // NegPower
                        if discriminant(&fade.fade_type) == discriminant(&FadeType::NegPower(0.0)) {
                            if fade.fade_out { 
                                s.1 * (1.0 - ((s.0 as f64 - start_idx as f64) / len as f64).powf(pow))
                            } 
                            else { s.1 * (1.0 - ((s.0 as f64 - start_idx as f64 - len as f64) / len as f64).powf(pow)) } 
                        }
                        // Power
                        else {
                            if fade.fade_out { 
                                s.1 * ((s.0 as f64 - start_idx as f64 - len as f64) / len as f64).powf(pow)
                            } 
                            else { s.1 * ((s.0 as f64 - start_idx as f64) / len as f64).powf(pow) } 
                        }
                    } 
                    else { *s.1 }).collect();
        }
    }
}