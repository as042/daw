use super::*;

impl Wav {
    pub(super) fn raw_sample_data(&self, data: &mut Vec<u8>, tracks: &Vec<Track>) {
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
                    self.compute_raw_sample(data, samples, settings, i);

                    for _ in 0..self.NumChannels - channel + 1 {
                        for _ in 0..self.NumChannels {
                            data.push(0);
                        }
                    }
                }
                // channel idx is within range of export channels
                else if channel < self.NumChannels {
                    self.compute_raw_sample(data, samples, settings, i);
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

    fn compute_raw_sample(&self, data: &mut Vec<u8>, samples: &Vec<u8>, settings: WavSettings, i: usize) {
        let mut sample = [0; 8];
                    
        for k in 0..settings.bytes_per_sample as usize {
            sample[k] = samples[i + k];
        }

        // the "easy" case
        if settings.sample_rate == self.SampleRate {
            let sample_int = u64::from_le_bytes(sample);

            let double = sample_int as f64 / 2_f64.powf(settings.bytes_per_sample as f64 * 8_f64);

            let final_value = double * 2_f64.powf(self.BitsPerSample as f64);

            data.extend_from_slice(&final_value.to_le_bytes());
        }
        // ruh-roh
        else {
            
        }
    }
}