#![allow(dead_code)]

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Project {
    tracks: Vec<Track>
}

impl Project {
    pub fn new() -> Project {
        Project { tracks: Vec::default() }
    }
}