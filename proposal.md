# Reactive Shaders For Music Visualization
I want to create visualizations that are responsive to sound, particularly for a live electronic music setting. I am especially interested in shaders, which are programs that control the 3d rendering of color and sound [1]. These programs are often defined by parameters which control the speed, lightning, color, and other effects in the outputs; this makes shaders tractable and convenient for visualizing sound. About a year ago, I made a simple track in FL-studio, and mapped the amplitude of the mixer tracks to the parameters of a shader [3] which I further customized (for example, rides would turn the camera, a snare would color certain spots, kick would affect zoom). The final result can be found in this time-stamped video [2]. While the result is responsive to sonic events, it has some downsides: The mapping of mixer track amplitudes to shader parameters adds a fair bit of complexity to the production process, as it requires simultaneous setup of the audio and the visual components. Furthermore, the approach would only work for the standalone track as a kind of "music video", and not appropriate for a live setting where many tracks might be played and mixed together. For my project I would like to implement shaders which respond to musical features of any sound that is picked up by a microphone. The features need to be extracted automatically, in near-real time, and ideally reflect musical effects beyond frequency content, making the project suitable for a live musical performance such as a DJ set.  

Similar tools already exist and are used in live performancs. Shaders are a common tool for creating visuals for live music. For example, VirtualDJ has supported the use of shaders as a visualization method in live music settings since 2018 [4]. However, deficiencies exist in the current tools, for example:
1. The responsiveness is simple and often strictly frequency and amplitude based. Patterns and grouping within the frequency spectrogram are not considered.
	- In [5] we see a shader which responds to microphone input. While the end result is eye catching, it mainly responds to "low vs medium vs high" frequency content. 
2. The tools are expensive, closed source, and not very customizable. This is understandable as most musicians prefer not to tinker with software too much, but it prevents implementation of different approaches to feature extraction.
	- Virtualdj is subscription based, and how the shaders respond to the music is not well documented [4].
3. The tools are part of a bigger environment which must be integrated to the performance before it can be used.
	- Adding more integrated tools to a performance can cause latency and have unpredictible effects (linux is often not supported). 

# High Level Approach
To address these problems (also to learn more about sound processing and shader programming) I would like to implement my own set of software for sound visualization using shaders. Using a portable device with a microphone (i.e, my laptop), we can capture the audio in the room where music is being played, and implement a moving spectrogram of sound [6] by using Fast Fourier transforms (FFT) to continually calculate the frequency content of the sound, and save these frequency content values for a certain number of time-steps in the past. The reason for saving past FFT values is that application of real time clustering algorithms such as dbscan [7] can help with extraction of features which are more descriptive of the sound as it progresses; for example, a clustering algorithm might group frequencies related to kicks, snares, and hats effectively, or distinguish between different types of high-frequency instruments. We can then use the characteristics of these clusters as parameters that are passed to a shader program, modifying the colors, lights, and movements accordingly. This setup would work independently of the source of sound, as it uses a microphone rather than requiring an audio stream, making it convenient to setup for a live performance and completely independent of the performance method. The obvious downside here is that the sound captured by the microphone will vary depending on the quality and positioning of the microphone. 

# Technical Implementation 
There are 2 main components here that need to be implemented. 
1. Sound needs to be captured and analyzed in near-real time, and its characteristics need to be extracted.
	- Standard short-time FFT feature extraction will be implemented first. I would like to use a realtime clustering algorithm if time permits [7].
	- This component needs to send the extracted features to the second component
2. We need an environment where we can define tractable shaders, and change their parameters in real time. 
	- This component needs to recieve the extracted features and use them to modify the paramteres.
Since we want high performance and low-latency for recording and feature extraction, I chose to use Rust for implementation of component 1 (This also provides a good excuse to learn a language that i can use for VST plugin programming in the future). So far, in separate programs [9], have verified that I can create spectrograms of sounds, capture microphone input with low-latency, apply clustering with dbscan, and send open sound control messages. Combining all these programs into one may prove difficult, and I may have to skip the clustering for the final project. For component 2, I have verified that I can create shaders using p5.js [8], and that p5.js can send and recieve open sound control messages. I don't have much to report for component 2 and still need to learn more about shader programming, particularly in the p5.js setting. 

# What I hope to demo after reading week 
I plan on first implementing a Rust program which captures microphone inputs, and sends the amplitude of sound as a modifying parameter to p5.js (which hopfully will not be very difficult). The rest of my time during reading week will be spent on learning shader programming, and creating a shader which responds to my microphone. This would obviously be very simple to implement with a different toolset (e.g, it can easily be done inside shadertoy), however, with the "everything from scratch" approach I'd be able to extend the approach with different feature extraction methods and algorithms in my final demo and use it in future performances. 

[1]: https://www.shadertoy.com/playlist/NltcWN
[2]: https://www.youtube.com/watch?v=KLtXvvahR8w&t=11396s
[3]: https://www.shadertoy.com/view/Xd3GRf
[4]: https://www.virtualdj.com/forums/223454/General_Discussion/Best_visualization_shaders_for_VirtualDJ.html
[5]: https://www.shadertoy.com/view/llB3W1
[6]: https://musiclab.chromeexperiments.com/spectrogram/
[7]: https://dl.acm.org/doi/abs/10.1145/1281192.1281210?casa_token=OpITTfd-bn4AAAAA:LBgMnN_E0vqoDMKEjpBxK5_aHtdPGWhdGQKEVsBKllir8IYtGKf-5k7b6nwexcAPoPp_znY6oXkQnDk
[8]: https://p5js.org/learn/getting-started-in-webgl-shaders.html
[9]: https://github.com/imilas/645/tree/main/src

# References:
	[1]: https://www.shadertoy.com/playlist/NltcWN
	[2]: https://www.youtube.com/watch?v=KLtXvvahR8w&t=11396s
	[3]: https://www.shadertoy.com/view/Xd3GRf
	[4]: https://www.virtualdj.com/forums/223454/General_Discussion/Best_visualization_shaders_for_VirtualDJ.html
	[5]: https://www.shadertoy.com/view/llB3W1
	[6]: https://musiclab.chromeexperiments.com/spectrogram/
	[7]: https://dl.acm.org/doi/abs/10.1145/1281192.1281210?casa_token=OpITTfd-bn4AAAAA:LBgMnN_E0vqoDMKEjpBxK5_aHtdPGWhdGQKEVsBKllir8IYtGKf-5k7b6nwexcAPoPp_znY6oXkQnDk
	[8]: https://p5js.org/learn/getting-started-in-webgl-shaders.html
	[9]: https://github.com/imilas/645/tree/main/src
