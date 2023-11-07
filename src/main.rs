mod fft;
mod utils;
use plotly::Scatter;
use plotly::{HeatMap, Plot};
use spectrum_analyzer::FrequencySpectrum;

pub fn windows(v: &Vec<f32>, overlap: i32) -> std::slice::Windows<'_, f32> {
    // returns an iterator of vector windows
    let iter = v.windows(overlap as usize);
    return iter;
}

pub fn plot_fft_window(samples: Vec<f32>) {
    // uses plot.ly to plot mel frequencies of a vector
    let spec = fft::fft(samples);
    let (v1, v2) = spec_to_mels(spec);
    let mut plot = Plot::new();
    let trace = Scatter::new(v1, v2);
    plot.add_trace(trace);
    plot.show();
}

pub fn spec_to_mels(spec: FrequencySpectrum) -> (Vec<f32>, Vec<f32>) {
    // takes in a spectrogram and return 2 vectors (mels,amplitudes)
    let v1: Vec<f32> = spec.to_mel_map().iter().map(|e| *e.0 as f32).collect();
    let v2: Vec<f32> = spec.to_mel_map().iter().map(|e| *e.1).collect();
    return (v1, v2);
}

pub fn rolling_window(v: &Vec<f32>, size: i32, step: i32) -> Vec<Vec<f32>> {
    let w = Vec::new();
    i = 0; // loop iterator
    while i < v.len() {
        w.push(v[0..10]);
        i += v.len();
    }
    println!("{:?}", w);
    return w;
}
fn main() {
    let v = utils::buff_to_vec("data/s1.wav".to_string());
    let samples: Vec<f32> = utils::convert_vecs(v);
    let w: Vec<Vec<f32>> = windows(&samples, 100).map(|x| x.to_vec()).collect();
    // plot_fft_window(w[0]);
    rolling_window(&v, 10, 1000);
}
