#[derive(Debug, Clone, PartialEq)]
pub struct Wav {
    pub(in crate::project) ChunkID: i32,  // big-endian
    pub(in crate::project) ChunkSize: i32,
    pub(in crate::project) Format: i32,  // big-endian
    
    pub(in crate::project) Subchunk1ID: i32,  // big-endian
    pub(in crate::project) Subchunk1Size: i32,
    pub(in crate::project) AudioFormat: i32,
    pub(in crate::project) NumChannels: i32,
    pub(in crate::project) SampleRate: i32,
    pub(in crate::project) ByteRate: i32,
    pub(in crate::project) BlockAlign: i32,
    pub(in crate::project) BitsPerSample: i32,

    pub(in crate::project) Subchunk2ID: i32,  // big-endian
    pub(in crate::project) Subchunk2Size: i32,
    pub(in crate::project) Data: Vec<u8>
}

impl Wav {
    pub(in crate::project) fn create_wav(&self) -> Vec<u8> {
        Vec::default()
    }
}

impl Default for Wav {
    fn default() -> Self {
        Self {
            ChunkID: 0x52494646,
            ChunkSize: i32::default(),
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
            Subchunk2Size: i32::default(),
            Data: Vec::default(),
        }
    }
}