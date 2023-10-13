use spectrum_analyzer::scaling::divide_by_N_sqrt;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};

/// Minimal example.
fn main() {
    // YOU need to implement the samples source; get microphone input for example
    //let samples: &[f32] = &[0.0, 3.14, 2.718, -1.0, -2.0, -4.0, 7.0, 6.0];
    let sample_rate: usize = 44100;
    let seconds: usize = 10;
    let sample_count = sample_rate * seconds;
    let all_samples = (0..sample_count).map(|x| x as f32).collect::<Vec<f32>>();
    let samples = all_samples;
    // apply hann window for smoothing; length must be a power of 2 for the FFT
    // 2048 is a good starting point with 44100 kHz
    let hann_window = hann_window(&samples[0..128]);
    // calc spectrum
    let spectrum_hann_window = samples_fft_to_spectrum(
        // (windowed) samples
        &hann_window,
        // sampling rate
        44100,
        // optional frequency limit: e.g. only interested in frequencies 50 <= f <= 150?
        FrequencyLimit::All,
        // optional scale
        Some(&divide_by_N_sqrt),
    )
    .unwrap();

    for (fr, fr_val) in spectrum_hann_window.data().iter() {
        println!("{}Hz => {}", fr, fr_val)
    }
}
