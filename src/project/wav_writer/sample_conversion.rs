pub fn sample_to_f64(sample: [u8; 8], bytes_per_sample: i32) -> f64 {
    let sample_int = i64::from_le_bytes(sample);

    let unsigned = sample_int as f64 / 2_f64.powf(bytes_per_sample as f64 * 8.0);
    let final_value = unsigned * 2.0;

    final_value
}

pub fn f64_to_sample(value: f64, bytes_per_sample: i32) -> [u8; 8] {
    let sample_max = 2_f64.powf(bytes_per_sample as f64 * 8.0);

    let int = (value * 0.5 * sample_max) as i64;

    int.to_le_bytes()
}

#[cfg(test)]
#[test]
fn test_sample_to_f64() {
    let sample = [0, 0, 0, 0, 0, 0, 0, 0];

    let f64 = sample_to_f64(sample, 2);

    assert_eq!(f64, 0.0);
}

#[cfg(test)]
#[test]
fn test_f64_to_sample() {
    let f64 = 0.0;

    let sample = f64_to_sample(f64, 2);

    assert_eq!(sample, [0, 0, 0, 0, 0, 0, 0, 0]);
}