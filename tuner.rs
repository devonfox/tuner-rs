//! CS410P - HW3: Tuner
//! Devon Fox 2022

use cpal::traits::StreamTrait;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::BufferSize;
use cpal::Sample;
use cpal::SampleRate;
use cpal::StreamConfig;
use num_complex::Complex64;
use realfft::RealFftPlanner;
use std::env;
use std::f64;
use std::io::stdin;

fn main() {
    // Takes first argument as a filename to a wav file to resample to half the rate

    let numargs = env::args().count();
    match numargs {
        1 => read_input(),
        2 => {
            let filename = env::args().nth(1).expect("no filename provided");
            read_wav(filename);
        }
        _ => {
            println!("Too many arguments!");
        }
    }
}

fn read_wav(filename: String) {
    let mut reader = hound::WavReader::open(&filename).unwrap();

    // maps samples and collects to a vec, unwrapping result in the process
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    let inspec: hound::WavSpec = reader.spec();
    assert_eq!(inspec.channels, 1, "mono input files only.");

    let wav_samprate = inspec.sample_rate;
    let _duration = reader.duration() / inspec.sample_rate;
    // println!("\nSource File: '{}'", filename);
    // println!("Duration: {} second(s)", duration);
    // println!("Wav Sample Rate: {} sps", wav_samprate);
    let samples_size = samples.len();
    let samples_max: usize = 131072;
    let trim = match samples_max < samples_size {
        true => trim_wav(&samples, samples_max),
        false => trim_wav(&samples, closest_power(samples_size)),
    };

    let length = trim.len();
    // Apply the FFT here
    let mut real_planner = RealFftPlanner::<f64>::new();

    // Windowing in the conversion function
    let mut convertedsamples = vecconvert(trim);
    // create a FFT
    let r2c = real_planner.plan_fft_forward(length);
    // make input and output vectors

    let mut spectrum = r2c.make_output_vec();

    // Are they the length we expect?
    assert_eq!(convertedsamples.len(), length);
    assert_eq!(spectrum.len(), length / 2 + 1);

    // Forward transform the input data
    r2c.process(&mut convertedsamples, &mut spectrum).unwrap();

    // Report largest bin/freq

    let freq = highest_freq(spectrum, wav_samprate);
    println!("\nFrequency of '{}': {:.1} Hz\n", filename, freq);
}

fn highest_freq(fft_output: Vec<Complex64>, samplerate: u32) -> f64 {
    let mut max: f64 = 0.0;
    let mut position = 0;
    for i in 0..fft_output.len() {
        let re = fft_output[i].re * (1.0 / f64::sqrt(fft_output.len() as f64));
        let im = fft_output[i].im * (1.0 / f64::sqrt(fft_output.len() as f64));
        let mut bin = f64::sqrt(f64::powf(re, 2.0) + f64::powf(im, 2.0));
        bin = bin.abs();
        if bin > max {
            max = bin;
            position = i;
        }
    }

    position as f64 * samplerate as f64 * 0.5 / fft_output.len() as f64
}

fn trim_wav(samples: &[i16], length: usize) -> Vec<i16> {
    let mut trimmed: Vec<i16> = Vec::new();

    for sample in samples.iter().take(length) {
        trimmed.push(*sample);
    }
    trimmed
}

fn read_input() {
    println!("Debug: Reading fronm a live input is still broken...");

    // let _pa = pa::PortAudio::new().unwrap();

    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No default input device");

    println!("Device: {:?}", device.name());
    let samplerate: SampleRate = SampleRate(48000);
    let buffersize: BufferSize = BufferSize::Fixed(8192);
    let config2: StreamConfig = StreamConfig {
        channels: 1,
        sample_rate: samplerate,
        buffer_size: buffersize,
    };
    let _config = device
        .default_input_config()
        .expect("No default input config");

    println!("Config: {:?}", config2);

    let stream = device
        .build_input_stream(
            &config2,
            move |data: &[f32], _| {
                std::thread::sleep(std::time::Duration::from_millis(100));
                // let mut buffer: Vec<f64> = Vec::new();
                let mut buffer: Vec<f64> = data.iter().map(|x| x.to_i16() as f64).collect();
                buffer.push(0.0);
                // println!("{}", buffer.len());
                let window = apodize::triangular_iter(buffer.len()).collect::<Vec<f64>>();

                // buffer that will hold data * window
                let mut windowed_buffer = vec![0.; buffer.len()];

                for (windowed, (window, data)) in windowed_buffer
                    .iter_mut()
                    .zip(window.iter().zip(buffer.iter()))
                {
                    *windowed = *window * *data;
                }
                let length = windowed_buffer.len();
                let mut real_planner = RealFftPlanner::<f64>::new();
                let r2c = real_planner.plan_fft_forward(length);
                let mut spectrum = r2c.make_output_vec();
                assert_eq!(windowed_buffer.len(), length);
                assert_eq!(spectrum.len(), length / 2 + 1);
                r2c.process(&mut windowed_buffer, &mut spectrum).unwrap();

                let freq = highest_freq(spectrum, 48000);
                println!("\n{:.1} Hz", freq);
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            },
            err_fn,
        )
        .expect("Invalid stream");
    stream.play().unwrap();
    let mut input = String::new();
    let stdin = stdin();
    input.clear();
    match stdin.read_line(&mut input) {
        Ok(_) => println!("Ending program..."),
        Err(err) => println!("Error: {}", err),
    }
    drop(stream);
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}

fn closest_power(samples: usize) -> usize {
    let mut trimmed = 0;
    for i in (0..samples).rev() {
        if i & (i - 1) == 0 {
            trimmed = i;
            break;
        }
    }
    if (trimmed * 2) == samples {
        return samples;
    }
    trimmed
}

fn vecconvert(samples: Vec<i16>) -> Vec<f64> {
    let mut output: Vec<f64> = Vec::new();
    for sample in samples {
        output.push(sample as f64);
    }
    let window = apodize::triangular_iter(output.len()).collect::<Vec<f64>>();

    // buffer that will hold data * window
    let mut windowed_data = vec![0.; output.len()];

    for (windowed, (window, data)) in windowed_data
        .iter_mut()
        .zip(window.iter().zip(output.iter()))
    {
        *windowed = *window * *data;
    }

    windowed_data
}
