use byteorder::{LittleEndian, BigEndian, WriteBytesExt};

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
    pub(in crate::project) NumChannels: usize,
    pub(in crate::project) SampleRate: i32,
    pub(in crate::project) ByteRate: i32,
    pub(in crate::project) BlockAlign: i32,
    pub(in crate::project) BitsPerSample: i32,  // more accurately, bytes per channel * 8

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
        let len = tracks.iter().map(|x| x.len()).max().unwrap();
        let sample_len = len / self.BlockAlign as usize;

        self.Subchunk2Size = sample_len * self.BlockAlign as usize;
        self.ChunkSize = 36 + self.Subchunk2Size;
        self.BlockAlign = self.NumChannels as i32 * (self.BitsPerSample / 8);
        self.ByteRate = self.SampleRate * self.BlockAlign;
        
        let mut vec = Vec::default();

        self.create_header(&mut vec);

        let samples = tracks
            .iter()
            .find(|x| x.len() == len).unwrap()
            .data.raw_samples().unwrap()
            .samples();

        vec.append(&mut samples.clone());

        vec
    }

    fn create_header(&self, vec: &mut Vec<u8>) {
        vec.extend_from_slice(&self.ChunkID.to_be_bytes());
        vec.extend_from_slice(&self.ChunkSize.to_le_bytes()[0..4]);
        vec.extend_from_slice(&self.Format.to_be_bytes());

        vec.extend_from_slice(&self.Subchunk1ID.to_be_bytes()[4..8]);
        vec.extend_from_slice(&self.Subchunk1Size.to_le_bytes()[0..4]);
        vec.extend_from_slice(&self.AudioFormat.to_le_bytes()[0..2]);
        vec.extend_from_slice(&self.NumChannels.to_le_bytes()[0..2]);
        vec.extend_from_slice(&self.SampleRate.to_le_bytes()[0..4]);
        vec.extend_from_slice(&self.ByteRate.to_le_bytes()[0..4]);
        vec.extend_from_slice(&self.BlockAlign.to_le_bytes()[0..2]);
        vec.extend_from_slice(&self.BitsPerSample.to_le_bytes()[0..2]);

        vec.extend_from_slice(&self.Subchunk2ID.to_be_bytes());
        vec.extend_from_slice(&self.Subchunk2Size.to_le_bytes()[0..4]);
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