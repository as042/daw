#[inline]
pub fn sample_to_int(sample: [u8; 8]) -> i64 {
    i64::from_le_bytes(sample)
}

#[inline]
pub fn int_to_sample(input: i64) -> [u8; 8] {
    input.to_le_bytes()
}

#[inline]
pub fn sample_to_f64(sample: [u8; 8], bytes_per_sample: usize) -> f64 {
    let int = sample_to_int(sample);

    let mut decimal = int as f64 / 2_f64.powf(bytes_per_sample as f64 * 8.0);
    if bytes_per_sample == 1 { decimal += 0.25; } // when bps=1, amplitudes are encoded differently for some reason
    let final_value = decimal * 2.0 - 1.0;

    final_value
}

#[inline]
pub fn f64_to_sample(input: f64, bytes_per_sample: usize) -> [u8; 8] {
    let sample_max = 2_f64.powf(bytes_per_sample as f64 * 8.0);
    let int = ((input + 1.0) * 0.5 * sample_max) as i64;

    int_to_sample(int)
}

#[inline]
pub fn change_bytes_per_sample(sample: [u8; 8], bytes_per_sample: usize, export_bytes_per_sample: usize) -> [u8; 8] {
    let double = sample_to_f64(sample, bytes_per_sample);

    f64_to_sample(double, export_bytes_per_sample)
}

#[cfg(test)]
#[test]
fn test_sample_to_f64() {
    let sample = [0, 0, 0, 0, 0, 0, 0, 0];
    let f64 = sample_to_f64(sample, 2);

    assert_eq!(f64, -1.0);

    let sample = [0, 0, 0, 0, 0, 0, 0, 0];
    let f64 = sample_to_f64(sample, 1);

    assert_eq!(f64, -0.5);
}

#[cfg(test)]
#[test]
fn test_f64_to_sample() {
    let f64 = 0.0;
    let sample = f64_to_sample(f64, 2);

    assert_eq!(sample, [0, 128, 0, 0, 0, 0, 0, 0]);

    let f64 = 0.0;
    let sample = f64_to_sample(f64, 1);

    assert_eq!(sample, [128, 0, 0, 0, 0, 0, 0, 0]);
}

#[cfg(test)]
#[test]
fn test_change_bytes_per_sample() {
    let sample = [0, 127, 0, 0, 0, 0, 0, 0];
    let output_sample = change_bytes_per_sample(sample, 2, 3);

    assert_eq!(output_sample, [0, 0, 127, 0, 0, 0, 0, 0]);
}