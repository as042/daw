use crate::track::*;

use super::{Project, TrackType, WavSettings};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wav {
    pub(in crate::project) ChunkID: i32,  // big-endian
    pub(in crate::project) ChunkSize: usize,
    pub(in crate::project) Format: i32,  // big-endian
    
    pub(in crate::project) Subchunk1ID: i32,  // big-endian
    pub(in crate::project) Subchunk1Size: usize,
    pub(in crate::project) AudioFormat: i32,
    pub(in crate::project) NumChannels: usize,
    pub(in crate::project) SampleRate: i32,
    pub(in crate::project) ByteRate: i32,
    pub(in crate::project) BlockAlign: i32,
    pub(in crate::project) BitsPerSample: i32,  // more accurately, bytes per channel * 8

    pub(in crate::project) Subchunk2ID: i32,  // big-endian
    pub(in crate::project) Subchunk2Size: usize,
}

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

        let mut data = Vec::with_capacity(len);

        self.raw_sample_data(&mut data, tracks);

        vec
    }

    fn create_header(&self, vec: &mut Vec<u8>) {
        vec.extend_from_slice(&self.ChunkID.to_be_bytes());
        vec.extend_from_slice(&self.ChunkSize.to_le_bytes()[0..4]);
        vec.extend_from_slice(&self.Format.to_be_bytes());

        vec.extend_from_slice(&self.Subchunk1ID.to_be_bytes());
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

    fn raw_sample_data(&self, data: &mut Vec<u8>, tracks: &Vec<Track>) {
        let len = tracks.iter().map(|x| x.len()).max().unwrap();

        for track in tracks.iter().filter(|x| x.is_type(TrackType::RawSamples)) {
            let raw_samples = track.data.raw_samples().unwrap();
            let samples = raw_samples.samples();
            let settings = raw_samples.settings;

            let prev_channel = usize::default();
            for i in (0..len).step_by(settings.bytes_per_sample as usize) {
                let channel = i / settings.bytes_per_sample as usize % settings.num_channels; // idx of channel

                // channel idx is as high as it goes but less than export channels
                if settings.num_channels < self.NumChannels && channel + 1 == settings.num_channels {
                    self.compute_raw_sample(samples, settings, i);

                    for _ in 0..self.NumChannels - channel + 1 {
                        for _ in 0..self.NumChannels {
                            data.push(0);
                        }
                    }
                }
                // channel idx is within range of export channels
                else if channel < self.NumChannels {
                    self.compute_raw_sample(samples, settings, i);
                }
                // channel idx is outside range of export channels
                else if channel >= self.NumChannels {
                    continue;
                }
                // uh-oh, I forgot a case!
                else {
                    panic!("you dun messed up: {channel}");
                }
            }
        }
    }

    fn compute_raw_sample(&self, samples: &Vec<u8>, settings: WavSettings, i: usize) {
        let mut sample: [u8; 16] = [0; 16];
                    
        for k in 0..settings.bytes_per_sample as usize {
            sample[k] = samples[i + k];
        }

        let sample_int = u128::from_le_bytes(sample);
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
            Subchunk2Size: usize::default()
        }
    }
}