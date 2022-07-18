use lazy_static::lazy_static;
use log::info;
use rodio::{Decoder, OutputStream, Source, source::SineWave, Sink};
use std::{io::{Cursor, BufReader}, time::Duration, fs::File};

pub fn play_sound() {
    info!("Playing sound");
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("./src/lib_res/gun.wav").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).unwrap();
}
