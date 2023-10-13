use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::thread;
use std::time::Duration;

fn tone(freq: f32, dur: f32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source of the sake of the example.
    let source = SineWave::new(freq)
        .take_duration(Duration::from_secs_f32(dur))
        .amplify(0.20);
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    //
    sink.sleep_until_end();
    println!("{},{}", dur, freq)
}
fn main() {
    thread::spawn(|| {
        tone(100.0, 2.0);
    });
    thread::sleep(Duration::from_secs_f32(3.0));
}
