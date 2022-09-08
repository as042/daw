use byteorder::{LittleEndian, WriteBytesExt};

use crate::track::*;

use super::Project;

#[derive(Debug, Clone, PartialEq)]
pub struct Wav {
    pub(in crate::project) ChunkID: i32,  // big-endian
    pub(in crate::project) ChunkSize: usize,
    pub(in crate::project) Format: i32,  // big-endian
    
    pub(in crate::project) Subchunk1ID: usize,  // big-endian
    pub(in crate::project) Subchunk1Size: i32,
    pub(in crate::project) AudioFormat: i32,
    pub(in crate::project) NumChannels: i32,
    pub(in crate::project) SampleRate: i32,
    pub(in crate::project) ByteRate: i32,
    pub(in crate::project) BlockAlign: i32,
    pub(in crate::project) BitsPerSample: i32,

    pub(in crate::project) Subchunk2ID: i32,  // big-endian
    pub(in crate::project) Subchunk2Size: usize,
    pub(in crate::project) Data: Vec<u8>
}

// let mut wtr = vec![];
// wtr.write_u16::<LittleEndian>(517).unwrap();
// wtr.write_u16::<LittleEndian>(768).unwrap();
// assert_eq!(wtr, vec![5, 2, 0, 3]);

impl Wav {
    // only works for WAV tracks!
    pub(in crate::project) fn create_wav(&mut self, project: &Project) -> Vec<u8> {
        let tracks = &project.tracks;
        let sample_len = tracks.iter().map(|x| x.len()).max().unwrap() / self.BlockAlign as usize;

        self.Subchunk2Size = sample_len * self.BlockAlign as usize;
        self.ChunkSize = 36 + self.Subchunk2Size;

        let vec = Vec::<u8>::default();

        

        vec
    }
}

impl Default for Wav {
    fn default() -> Self {
        Self {
            ChunkID: 0x52494646,
            ChunkSize: usize::default(),
            Format: 0x57415645,
            Subchunk1ID: 0x666d7420,
            Subchunk1Size: 0x10,
            AudioFormat: 0x1,
            NumChannels: 0x2,
            SampleRate: 0xAC44,
            ByteRate: 0x2B110,
            BlockAlign: 0x4,
            BitsPerSample: 0x10,
            Subchunk2ID: 0x64617461,
            Subchunk2Size: usize::default(),
            Data: Vec::default(),
        }
    }
}