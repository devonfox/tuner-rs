## HW3: Tuner

CS410P - Music, Sound, and Computers

*Devon Fox 2022*
### Linux/Mac Build/Run Instructions

This program uses the latest version of Rust (1.59.0).  Make sure this is installed.

To run, simply write `cargo run 'filename.wav`

At the moment, this is the only available command, and running without a file argument will not work as my live input is still broken.  I am in the process of rewriting it, and hopefully can get something working before the grading process begins.

*This program will only accept mono (single channel) wav files.*

### What Went Down

This was quite challenging, and in the end I learned a lot about applying an FFT, and how to find the approximate frequency.  I read many different wave files to test, and applied the FFT from the `realfft` crate, which accepts real number inputs and outputs to a complex array.  I then found the most prominent frequency in my own handwritten function and displayed this frequency. 

*todo!*

*Still need to assess and finish the live input portion*

###  How It Went

Reading the frequency was challenging but overall, Bart helped me fill a few gaps in my understanding and I was able to get this functionality working.  The function displays an accuracy within 0.5Hz for the most part, except in the cases that we read a higher harmonic, in which case it displays a multiple, for instance in the example file `guitar-a4.wav`, it read 880Hz instead of 440Hz.  The sine waves I tested were all super accurate, as there are no harmonics present.

*todo!*

*Still need to assess and finish the live input portion*
