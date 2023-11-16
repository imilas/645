use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Data;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::sync::mpsc;

use std::convert::From;
use std::iter::Sum;
use std::ops::Div;

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();
    let input_device = host
        .default_input_device()
        .expect("no output device available");
    let (tx, rx) = mpsc::channel();

    let mut buffer: AllocRingBuffer<f32> = AllocRingBuffer::with_capacity(20000);

    let config: cpal::StreamConfig = input_device.default_input_config().unwrap().into();
    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        for &sample in data {
            // eprintln!("{} ", sample);
            // buffer.push(sample);
            tx.send(sample).unwrap();
        }
    };
    println!(
        "Attempting to build both streams with f32 samples and `{:?}`.",
        config
    );
    let input_stream = input_device.build_input_stream(&config, input_data_fn, err_fn, None)?;
    input_stream.play()?;

    let mut i = 0;
    for received in rx {
        i += 1;
        if i % 3000 == 0 {
            let avg = average(&buffer.to_vec());
            print!("Got: {},{i},{avg}", received);
            print!("\r");
        }
        buffer.push(received.abs());
    }

    drop(input_stream);
    Ok(())
}
fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}

fn average<'v, T>(v: &'v [T]) -> T
where
    T: Div<Output = T>,
    T: From<u16>,
    T: Sum<&'v T>,
{
    v.iter().sum::<T>() / From::from(v.len() as u16)
}
