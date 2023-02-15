use crate::project::WavSettings;
use super::sample_conversion::*;

pub fn format_samples(samples: &Vec<u8>, settings: WavSettings, export_settings: WavSettings) -> Vec<u8> {
    match_num_channels(&match_bytes_per_sample(
        samples, 
        settings, 
        export_settings),
    settings, 
    export_settings)
}

fn match_bytes_per_sample(samples: &Vec<u8>, settings: WavSettings, export_settings: WavSettings) -> Vec<u8> {
    if settings.bytes_per_sample == export_settings.bytes_per_sample { return samples.to_vec(); }

    let mut output = Vec::with_capacity(samples.len() * export_settings.bytes_per_sample / settings.bytes_per_sample);

    for k in (0..samples.len()).step_by(settings.bytes_per_sample) {
        let mut sample = [0; 8];
        for l in (k..(k + settings.bytes_per_sample)).enumerate() {
            sample[l.0] = samples[l.1];
        }

        let changed_sample = &change_bytes_per_sample(sample, settings.bytes_per_sample, export_settings.bytes_per_sample)[0..export_settings.bytes_per_sample];
        output.extend_from_slice(changed_sample);
    }

    output
}

fn match_num_channels(samples: &Vec<u8>, settings: WavSettings, export_settings: WavSettings) -> Vec<u8> {
    if settings.num_channels == export_settings.num_channels { return samples.to_vec(); }

    let mut output = Vec::with_capacity(samples.len() * export_settings.num_channels / settings.num_channels);

    for k in (0..samples.len()).step_by(export_settings.bytes_per_sample) {
        let channel_idx = k / export_settings.bytes_per_sample % settings.num_channels;

        if channel_idx < export_settings.num_channels {
            let mut sample = [0; 8];
            for l in (k..(k + export_settings.bytes_per_sample)).enumerate() {
                sample[l.0] = samples[l.1];
            }

            output.extend_from_slice(&sample[0..export_settings.bytes_per_sample]);
        }
        if channel_idx + 1 == settings.num_channels && settings.num_channels < export_settings.num_channels {
            for _ in 0..((export_settings.num_channels - settings.num_channels) * export_settings.bytes_per_sample) {
                output.push(0);
            }
        }
    }

    output
}

#[cfg(test)]
#[test]
fn test_match_bytes_per_sample() {
    let vec = vec![0, 128, 0, 0, 0, 128, 0, 0];

    let output = match_bytes_per_sample(&vec, WavSettings { 
        num_channels: 2,  
        bytes_per_sample: 2,
        ..Default::default()
    },
    WavSettings { 
        num_channels: 2,  
        bytes_per_sample: 3,
        ..Default::default()
    });

    assert_eq!(output, vec![0, 0, 128, 0, 0, 0, 0, 0, 128, 0, 0, 0]);

    let vec = vec![0, 128, 0, 0, 0, 128, 0, 128, 128];

    let output = match_bytes_per_sample(&vec, WavSettings { 
        num_channels: 2,  
        bytes_per_sample: 3,
        ..Default::default()
    },
    WavSettings { 
        num_channels: 2,  
        bytes_per_sample: 2,
        ..Default::default()
    });

    assert_eq!(output, vec![128, 0, 0, 128, 128, 128]);

    let vec = vec![0, 128, 0, 0, 0, 128, 255, 129];

    let output = match_bytes_per_sample(&vec, WavSettings { 
        num_channels: 2,  
        bytes_per_sample: 2,
        ..Default::default()
    },
    WavSettings { 
        num_channels: 2,  
        bytes_per_sample: 1,
        ..Default::default()
    });

    assert_eq!(output, vec![128, 0, 128, 129]);
}

#[cfg(test)]
#[test]
fn test_match_num_channels() {
    let vec = vec![0, 128, 0, 0, 0, 128, 0, 0];

    let output = match_num_channels(&vec, WavSettings { 
        num_channels: 2,  
        bytes_per_sample: 2,
        ..Default::default()
    },
    WavSettings { 
        num_channels: 3,  
        bytes_per_sample: 2,
        ..Default::default()
    });

    assert_eq!(output, vec![0, 128, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0]);

    let vec = vec![0, 128, 0, 0, 0, 255, 0, 128, 0, 0, 255, 255];

    let output = match_num_channels(&vec, WavSettings { 
        num_channels: 3,  
        bytes_per_sample: 2,
        ..Default::default()
    },
    WavSettings { 
        num_channels: 2,  
        bytes_per_sample: 2,
        ..Default::default()
    });

    assert_eq!(output, vec![0, 128, 0, 0, 0, 128, 0, 0]);
}

#[cfg(test)]
#[test]
fn test_format_samples() {
    let vec = vec![0, 18, 46, 0, 0, 128, 23, 1];

    let output = format_samples(&vec, WavSettings { 
        num_channels: 2,  
        bytes_per_sample: 4,
        ..Default::default()
    },
    WavSettings { 
        num_channels: 4,  
        bytes_per_sample: 2,
        ..Default::default()
    });

    assert_eq!(output, vec![46, 0, 23, 1, 0, 0, 0, 0]);

    let vec = vec![46, 0, 23, 1, 0, 0, 0, 0];

    let output = format_samples(&vec, WavSettings { 
        num_channels: 4,  
        bytes_per_sample: 2,
        ..Default::default()
    },
    WavSettings { 
        num_channels: 2,  
        bytes_per_sample: 4,
        ..Default::default()
    });

    assert_eq!(output, vec![0, 0, 46, 0, 0, 0, 23, 1]);
}