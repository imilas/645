mod fft;
mod utils;
use plotly::{Plot, Scatter};

fn main() {
    // let mut reader = hound::WavReader::open("data/k2.wav").unwrap();
    let v = utils::buff_to_vec("data/s1.wav".to_string());
    let samples: Vec<f32> = utils::convert_vecs(v);
    // 2048 is a good starting point with 44100 kHz
    let spec = fft::fft(samples);

    for (fr, fr_val) in spec.to_mel_map().iter() {
        println!("{} => {}", fr, fr_val)
    }

    let mut plot = Plot::new();
    let v1: Vec<f32> = spec.to_mel_map().iter().map(|e| *e.0 as f32).collect();
    let v2: Vec<f32> = spec.to_mel_map().iter().map(|e| *e.1).collect();
    let trace = Scatter::new(v1.clone(), v2);
    plot.add_trace(trace);
    plot.show();
}
