# What it does
This prototype captures audio in real-time and changes the color of the screen based on the frequency content of the sound. 
The frequency content will change the color of the screen. Currently, red is low frequencies, green is mid, and blue is high. 

# How is it made
This project is written purely in Rust. It uses the Cpal library [2] to capture sounds and the wgpu library [1] to create visuals. 
In the original proposal, I intended to use processing/p5.js for the visual aspect. However, I ran into many issues connecting the FFT algorithm results to p5.js. I used the ringbuf library [3] in rust to create a memory heap that is accessable both by the microphone and the visualizer. This way the microphone can push samples into the heap while the visualizer can apply the FFT algorithm and extract features, and clearing the heap to be filled for the next period of time. The result is very fast and efficient, at the cost of not being very sophosticated (yet).

After trying and reading about many libraries and languages, I decided that the easiest and most extensible way forward (I plan on working on this project after the course) would be to learn GPU programming from scratch and use a rust library. I will write a forum post about graphics libraries available in rust and c++ in more detail.

# How will it be extended 
Currently the visual aspect is very simple, and I intend to extend it further. The ultimate visual goal is to modify shaders, and that is what I will be focusing on completing before the deadline. Currently there are examples in the wgpu repository which show use of shaders, however it looks like a fairly involved process. Failing that, there are many other interesting visualization techniques to learn and implement. 

The other goal is the audio features, which currently is only the presence of low/mid/high frequencies in a short buffer of sound. I am not sure if I will be able to extend this further before the deadline, but I will continue working on it for the next few months. 

# Implementation Diagram



# References
1. WGPU: https://github.com/gfx-rs/wgpu
2. CPAL: https://github.com/RustAudio/cpal
3. RingBuffer: https://docs.rs/ringbuf/latest/ringbuf/
