pub mod raw_sample_writer;
pub mod sample_conversion;
mod format;
mod resample;

pub use method_shorthands::methods::UW;

use crate::project::track::*;
use super::{Project, TrackType, WavSettings};
use raw_sample_writer::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wav {
    pub(in crate::project) chunk_id: i32,  // big-endian
    pub(in crate::project) chunk_size: usize,
    pub(in crate::project) format: i32,  // big-endian
    
    pub(in crate::project) subchunk1_id: i32,  // big-endian
    pub(in crate::project) subchunk1_size: usize,
    pub(in crate::project) audio_format: i32,
    pub(in crate::project) num_channels: usize,
    pub(in crate::project) sample_rate: i32,
    pub(in crate::project) byte_rate: i32,
    pub(in crate::project) block_align: usize,
    pub(in crate::project) bits_per_sample: usize,  // more accurately, bytes per channel * 8

    pub(in crate::project) subchunk2_id: i32,  // big-endian
    pub(in crate::project) subchunk2_size: usize,
}

impl Wav {
    // only works for WAV tracks!
    pub(in crate::project) fn create_wav(&mut self, project: &Project) -> Vec<u8> {
        let tracks = &project.tracks;

        self.block_align = self.num_channels * (self.bits_per_sample / 8);
        let len = tracks.iter().map(|x| x.size(self.bits_per_sample / 8 * self.num_channels, self.sample_rate)).max().uw();

        self.subchunk2_size = len;
        self.chunk_size = 36 + self.subchunk2_size;
        self.byte_rate = self.sample_rate * self.block_align as i32;
        
        let mut vec = Vec::default();

        self.create_header(&mut vec);

        let mut data = vec![0; len];
        raw_sample_data(&mut data, tracks, WavSettings { 
            num_channels: self.num_channels, 
            sample_rate: self.sample_rate, 
            bytes_per_sample: self.bits_per_sample as usize / 8
        });

        vec.extend_from_slice(&data);

        vec
    }

    fn create_header(&self, vec: &mut Vec<u8>) {
        vec.extend_from_slice(&self.chunk_id.to_be_bytes());
        vec.extend_from_slice(&self.chunk_size.to_le_bytes()[0..4]);
        vec.extend_from_slice(&self.format.to_be_bytes());

        vec.extend_from_slice(&self.subchunk1_id.to_be_bytes());
        vec.extend_from_slice(&self.subchunk1_size.to_le_bytes()[0..4]);
        vec.extend_from_slice(&self.audio_format.to_le_bytes()[0..2]);
        vec.extend_from_slice(&self.num_channels.to_le_bytes()[0..2]);
        vec.extend_from_slice(&self.sample_rate.to_le_bytes()[0..4]);
        vec.extend_from_slice(&self.byte_rate.to_le_bytes()[0..4]);
        vec.extend_from_slice(&self.block_align.to_le_bytes()[0..2]);
        vec.extend_from_slice(&self.bits_per_sample.to_le_bytes()[0..2]);

        vec.extend_from_slice(&self.subchunk2_id.to_be_bytes());
        vec.extend_from_slice(&self.subchunk2_size.to_le_bytes()[0..4]);
    }
}

impl Default for Wav {
    fn default() -> Self {
        Self {
            chunk_id: 0x52494646,
            chunk_size: usize::default(),
            format: 0x57415645,
            subchunk1_id: 0x666d7420,
            subchunk1_size: 0x10,
            audio_format: 0x1,
            num_channels: 0x2,
            sample_rate: 0xAC44,
            byte_rate: 0x2B110,
            block_align: 0x4,
            bits_per_sample: 0x10,
            subchunk2_id: 0x64617461,
            subchunk2_size: usize::default()
        }
    }
}