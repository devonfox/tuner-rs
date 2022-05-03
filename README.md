## HW3: Tuner

CS410P - Music, Sound, and Computers

*Devon Fox 2022*
### Linux/Mac Build/Run Instructions

This program uses the latest version of Rust (1.59.0).  Make sure this is installed.

To run and analyze a .wav file, simply write `cargo run 'filename.wav`

To run and analyze live input, simply enter `cargo run` in the project folder and it will open a connection to the default input device.  Currently this is setup with a fixed sample buffer of `8192` samples.  It works with my Blue Yeti USB microphone, however, when using on Mac or laptop microphone the buffer size is too high.

To stop, just hit 'enter' on the keyboard, and it will stop the live input thread and stop the program.

Leaving that as a *todo!* option.

*This program will only accept mono (single channel) wav files.*

### What Went Down

This was quite challenging, and in the end I learned a lot about applying an FFT, and how to find the approximate frequency.  I read many different wave files to test, and applied the FFT from the `realfft` crate, which accepts real number inputs and outputs to a complex array.  I then found the most prominent frequency in my own handwritten function and displayed this frequency. 

Getting live input to register correctly was also a challenge using `cpal`.  I attempted to use `portaudio` but had more issues with cross-platform use, so opted for `cpal`.  

###  How It Went

Reading the frequency was challenging but overall, Bart helped me fill a few gaps in my understanding and I was able to get this functionality working.  The function displays an accuracy within 0.5Hz for the most part, except in the cases that we read a higher harmonic, in which case it displays a multiple, for instance in the example file `guitar-a4.wav`, it read 880Hz instead of 440Hz.  The sine waves I tested were all super accurate, as there are no harmonics present.

I had set the input buffer to `8192` samples but realizing after I tried running on my Mac with the macbook pro microphone that the available buffer is too small, so I need to fix this in my implementation so it doesn't just work on a more capable setup. I should maybe not have a fixed buffer, so I can adjust how many samples I want to use for the FFT no matter the size of the input device's buffer.