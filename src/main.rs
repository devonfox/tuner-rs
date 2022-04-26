use std::env;

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

    // creates a wav reader from 'hound' crate
}

fn read_wav(filename: String) {
    let mut reader = hound::WavReader::open(&filename).unwrap();

    // maps samples and collects to a vec, unwrapping result in the process
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    let inspec: hound::WavSpec = reader.spec();
    assert_eq!(inspec.channels, 1, "mono input files only.");

    let wav_samprate = inspec.sample_rate;
    let duration = reader.duration() / inspec.sample_rate;
    println!("\nSource File: '{}'", filename);
    println!("Duration: {} second(s)", duration);
    println!("Wav Sample Rate: {} sps", wav_samprate);
    let _size = duration * wav_samprate;
    if 131072 < samples.len() {
        let trim = trim_wav(&samples);
        println!("Trim Len: {}", trim.len());
    }

    println!("Samples Len: {}", samples.len());
}

fn trim_wav(samples: &[i16]) -> Vec<i16> {
    let trim_amount: usize = 131072;
    let mut trimmed: Vec<i16> = Vec::new();

    for sample in samples.iter().take(trim_amount) {
        trimmed.push(*sample);
    }
    trimmed
}

fn read_input() {
    println!("Debug: Reading from input...");
}
