// use method_shorthands::methods::UW;
// use rustfft::num_complex::Complex;

// use super::RawSamples;

// impl RawSamples {
//     pub fn pitch_test(buffer: &mut Vec<f64>) {
//         let mut complex_buffer: Vec<Complex<f64>> = buffer.iter().map(|s| Complex::<f64>::from(s)).collect();

//         Self::fft(&mut complex_buffer);

//         let min_phase = complex_buffer.iter().map(|s| s.im).reduce(|acc, s| {
//             if acc <= s { acc } else { s }
//         }).uw();       
//         let max_phase = complex_buffer.iter().map(|s| s.im).reduce(|acc, s| {
//             if acc >= s { acc } else { s }
//         }).uw();  

//         println!("{min_phase}, {max_phase}");

//         complex_buffer.iter_mut().for_each(|s| s.re *= s.im / max_phase);

//         Self::ifft(&mut complex_buffer);

//         buffer: Vec<f64> = complex_buffer.iter().map(|s| s.re).collect();
//     }
// }