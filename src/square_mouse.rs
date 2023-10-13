extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::time::Duration;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let audio_subsystem = sdl_context.audio()?;

    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1), // mono
        samples: None,     // default sample size
    };

    let _window = video_subsystem
        .window("Mouse", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut events = sdl_context.event_pump()?;
    let mut state;

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
        state = events.relative_mouse_state();
        println!("Relative - X = {:?}, Y = {:?}", state.x(), state.y());

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // Show obtained AudioSpec
            println!("{:?}", spec);
            // initialize the audio callback
            SquareWave {
                phase_inc: 100.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        })?;

        // Start playback
        device.resume();

        // Play for 2 seconds
        std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
