use rppal::gpio::Gpio;
use rand::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, source::Source};
use std::time::Duration;

//const SOUNDS_DIR: &'static str = "sounds";
const SOUNDS_DIR: &'static str = "/usr/local/share/doorbell/sounds";

fn choose_sound_file() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let files = std::fs::read_dir(SOUNDS_DIR)?;
    let selected = files.choose(&mut rand::thread_rng()).ok_or_else(|| "no sounds found")??;
    Ok(selected.path())
}

fn play_sound<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let file = BufReader::new(File::open(path)?);
    let source = Decoder::new(file)?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gpio = Gpio::new()?;
    let pin = gpio.get(15)?.into_input_pullup();

    loop {
        if pin.is_low() {
            let sound = choose_sound_file()?;
            println!("playing {:?}", sound);
            play_sound(sound)?;
        }
    }

    Ok(())
}
