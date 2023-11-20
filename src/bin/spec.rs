use main::fft;
use main::utils;

use plotly::{Contour, HeatMap, Layout, Plot};

pub fn windows(v: &Vec<f32>, overlap: i32) -> std::slice::Windows<'_, f32> {
    // returns an iterator of vector windows
    let iter = v.windows(overlap as usize);
    return iter;
}

pub fn slices(v: &Vec<f32>, w: usize, step: usize) -> Vec<Vec<f32>> {
    let mut slices = Vec::new();
    let mut i: usize = 0;
    while i + w < v.len() {
        // println!("{:?}", &samples[i..i + w]);
        let slice = v[i..i + w].to_vec();
        let slice_fft = fft::fft(&slice, w);
        let slice_mel = fft::spec_to_mels(&slice_fft).1;
        slices.push(slice_mel);
        i += step;
    }
    return slices;
}

fn main() {
    let v = utils::buff_to_vec("data/kitchen.wav".to_string());
    let samples: Vec<f32> = utils::convert_vecs(v);
    let w: usize = usize::pow(2, 11);
    let step: usize = 100;
    let mut s = slices(&samples, w, step);
    println!("{:?},{}", s.len(), s[0].len());
    s = utils::transpose(s);
    let trace = HeatMap::new_z(s);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();
}
