mod chord;
mod dict_lookup;
mod generator;
mod utils;

use {
    indicatif::ProgressBar,
    log::{error, info},
};

use std::{collections::BTreeSet, io};

use indicatif::ProgressStyle;

use {
    chord::Chord,
    generator::Generator,
    utils::{ErrBox, LenSortableString},
};

fn main() -> Result<(), ErrBox> {
    println!("Starting...");

    let mut gen = Generator::new()?;

    println!("Generator OK");

    let sjp_sanitized_len_sorted: BTreeSet<LenSortableString<true>> = dict_lookup::SJP_DICT
        .lines()
        .filter_map(|l| {
            match l.split(", ").next() {
                Some(word) => {
                    let sanitized = word.trim().to_lowercase();

                    if sanitized.chars().any(|ch| {
                        !(ch.is_ascii_alphabetic() || dict_lookup::PL_DIACRITICS.contains(ch)) // Ascii alphabet + PL accents only
		    || ch.is_whitespace() // No multi-word entries
                    }) || sanitized.chars().count() < 2
                    // No single character entries
                    {
                        None
                    } else {
                        Some(sanitized.into())
                    }
                }
                None => None,
            }
        })
        .collect();

    println!("Raw SJP OK");

    let bar = ProgressBar::new(sjp_sanitized_len_sorted.len() as u64).with_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg} [{per_sec}]",
        )?,
    );

    // env_logger::init();

    for (idx, sjp_word) in sjp_sanitized_len_sorted.iter().enumerate() {
        gen.add_word_root(&sjp_word.0)?;

        // Don't hog I/O for the progress bar
        if idx % 1000 == 0 {
            bar.set_message(sjp_word.0.clone());
            bar.inc(1000);
        }
    }

    bar.finish();

    println!("SJP processing OK");
    println!("{} distinct word roots created", gen.word_root_dict.len());

    let stdin = io::stdin();

    // display logging messages only after SJP processing
    env_logger::init();

    loop {
        let mut line_buf = String::new();

        stdin.read_line(&mut line_buf)?;

        match gen.add_word_root(&line_buf) {
            Ok(chords) => {
		println!("Chords: {}", chords.print_chords());
                println!("Full expansion: {}", chords.to_string());
            }
            Err(e) => {
                error!("{}", e.to_string());
            }
        }
    }
}
