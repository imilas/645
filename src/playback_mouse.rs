// made extensive use of rust-SDL2 examples : https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/mouse-state.rs
extern crate rand;
extern crate sdl2;
use rand::Rng;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::AudioSubsystem;
use std::i16;
use std::sync::mpsc;
use std::time::Duration;

const RECORDING_LENGTH_SECONDS: usize = 1;

fn render(canvas: &mut WindowCanvas, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.present();
}

struct Recording {
    record_buffer: Vec<i16>,
    pos: usize,
    done_sender: mpsc::Sender<Vec<i16>>,
    done: bool,
}

// Append the input of the callback to the record_buffer.
// When the record_buffer is full, send it to the main thread via done_sender.
impl AudioCallback for Recording {
    type Channel = i16;

    fn callback(&mut self, input: &mut [i16]) {
        if self.done {
            return;
        }

        for x in input {
            self.record_buffer[self.pos] = *x;
            self.pos += 1;
            if self.pos >= self.record_buffer.len() {
                self.done = true;
                self.done_sender
                    .send(self.record_buffer.clone())
                    .expect("could not send record buffer");
                break;
            }
        }
    }
}

fn record(
    audio_subsystem: &AudioSubsystem,
    desired_spec: &AudioSpecDesired,
) -> Result<Vec<i16>, String> {
    println!(
        "Capturing {:} seconds... Please rock!",
        RECORDING_LENGTH_SECONDS
    );

    let (done_sender, done_receiver) = mpsc::channel();

    let capture_device = audio_subsystem.open_capture(None, desired_spec, |spec| {
        println!("Capture Spec = {:?}", spec);
        Recording {
            record_buffer: vec![
                0;
                spec.freq as usize
                    * RECORDING_LENGTH_SECONDS
                    * spec.channels as usize
            ],
            pos: 0,
            done_sender,
            done: false,
        }
    })?;

    println!(
        "AudioDriver: {:?}",
        capture_device.subsystem().current_audio_driver()
    );
    capture_device.resume();

    // Wait until the recording is done.
    let recorded_vec = done_receiver.recv().map_err(|e| e.to_string())?;

    capture_device.pause();

    Ok(recorded_vec)
}

struct SoundPlayback {
    data: Vec<i16>,
    pos: usize,
}

impl AudioCallback for SoundPlayback {
    type Channel = i16;

    fn callback(&mut self, out: &mut [i16]) {
        for dst in out.iter_mut() {
            *dst = *self.data.get(self.pos).unwrap_or(&0);
            self.pos += 1;
        }
    }
}

fn replay_recorded_vec(
    audio_subsystem: &AudioSubsystem,
    desired_spec: &AudioSpecDesired,
    recorded_vec: Vec<i16>,
) -> Result<(), String> {
    let playback_device = audio_subsystem.open_playback(None, desired_spec, |spec| {
        println!("Playback Spec = {:?}", spec);
        SoundPlayback {
            data: recorded_vec,
            pos: 0,
        }
    })?;
    // Start playback
    playback_device.resume();

    std::thread::sleep(Duration::from_millis(500));
    // Device is automatically closed when dropped
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let audio_subsystem = sdl_context.audio()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Mouse", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");
    let mut desired_spec = AudioSpecDesired {
        freq: Some(44000),
        channels: None,
        samples: None,
    };
    let mut recorded_vec = record(&audio_subsystem, &desired_spec)?;
    // replay_recorded_vec(&audio_subsystem, &desired_spec, recorded_vec.clone())?;

    let mut events = sdl_context.event_pump()?;
    let mut state;
    let mut hold_freq = true;
    'running: loop {
        // Handle events
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    desired_spec.freq = Some(44000);
                    recorded_vec = record(&audio_subsystem, &desired_spec)?;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
                } => {
                    hold_freq = !hold_freq;
                }

                _ => {}
            }
        }
        // get mouse movement speed, and update the color and playback frequency
        state = events.relative_mouse_state();
        let movement_factor = i32::from((state.x().abs() * state.y().abs()) % 150) as u8;
        render(
            &mut canvas,
            Color::RGB(
                movement_factor + rand::thread_rng().gen_range(0..50),
                movement_factor + rand::thread_rng().gen_range(0..50),
                movement_factor + rand::thread_rng().gen_range(0..50),
            ),
        );
        if !hold_freq {
            desired_spec.freq = Some(state.x().abs() * state.y().abs() + 5000);
        }
        replay_recorded_vec(&audio_subsystem, &desired_spec, recorded_vec.clone())?;
    }

    Ok(())
}
