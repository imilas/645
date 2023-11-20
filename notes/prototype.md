# What it does

This prototype captures audio in real-time and changes the color of the screen based on the frequency content of the sound. 
The frequency content will change the color of the screen. Currently, red is low frequencies, green is mid, and blue is high. 

# How is it made

This project is writtent purely in Rust. It uses Cpal library to capture sounds and the wgpu library to create visuals. 

In the original proposal, I intended to use processing/p5.js for the visual aspect. However, I ran into many issues connecting the FFT algorithm results to p5.js. 
After trying and reading about many libraries and languages, I decided that the easiest and most extensible way forward (I plan on working on this project after the course) would be to learn GPU programming from scratch and use a rust library.

# How will it be extended 




