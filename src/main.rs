mod chord;
mod dict_lookup;
mod generator;
mod utils;

use log::{error, info};
use std::io;

use {
    chord::Chord,
    generator::Generator,
    utils::{ErrBox, LenSortableString},
};

fn main() -> Result<(), ErrBox> {
    env_logger::init();

    info!("Starting...");

    let mut gen = Generator::new()?;

    info!("Generator OK");

    let stdin = io::stdin();

    loop {
        let mut line_buf = String::new();

        stdin.read_line(&mut line_buf)?;

        match gen.add_word_root(&line_buf) {
            Ok(chords) => {
		println!("Chords: {}", chords.to_string());
            }
            Err(e) => {
                error!("{}", e.to_string());
            }
        }
    }
}
