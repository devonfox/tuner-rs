use std::env;

fn main() {
    // Takes first argument as a filename to a wav file to resample to half the rate

    let numargs = env::args().count();
    match numargs {
        1 => (),
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
    let reader = hound::WavReader::open(&filename).unwrap();

    // maps samples and collects to a vec, unwrapping result in the process
    //let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    let inspec: hound::WavSpec = reader.spec();
    assert_eq!(inspec.channels, 1, "mono input files only.");

    let wav_samprate = inspec.sample_rate;
    let duration = reader.duration() / inspec.sample_rate;
    println!("\nSource File: '{}'", filename);
    println!("Duration: {} second(s)", duration);
    println!("Wav Sample Rate: {} sps", wav_samprate);
}
