mod chord;
mod dict_lookup;
mod generator;
mod utils;

use {
    indicatif::ProgressBar,
    log::{debug, error, info},
};

use std::{collections::BTreeSet, fs::File, io};

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
        .map(|l| {
            l.split(", ").filter_map(|word| {
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
            })
        })
        .flatten()
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
    println!("{} distinct word chunks created", gen.chunk_dict.len());

    // display logging messages only after SJP processing
    env_logger::init();

    let mut chunk_conflicts_sorted_asc = Vec::new();
    // Figure out the number of conflicting strokes
    for (stroke, chunks) in gen.chunk_conflict_dict.iter() {
        if chunks.len() > 1 {
            chunk_conflicts_sorted_asc.push((stroke.clone(), chunks.clone()));
        }
    }

    chunk_conflicts_sorted_asc.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    for (stroke, chunks) in chunk_conflicts_sorted_asc.iter() {
        debug!("CHUNK CONFLICT {} -> {:?}", stroke.print_chords(), chunks);
    }

    println!(
        "{}/{} chunk outlines have conflicts",
        chunk_conflicts_sorted_asc.len(),
        gen.chunk_conflict_dict.len()
    );

    let mut word_root_conflicts_sorted_asc = Vec::new();
    // Figure out the number of conflicting strokes
    for (stroke, word_roots) in gen.word_root_conflict_dict.iter() {
        if word_roots.len() > 1 {
            word_root_conflicts_sorted_asc.push((stroke.clone(), word_roots.clone()));
        }
    }

    word_root_conflicts_sorted_asc.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    for (stroke, word_roots) in word_root_conflicts_sorted_asc.iter() {
        debug!(
            "WORD-ROOT-CONFLICT {} -> {:?}",
            stroke.print_chords(),
            word_roots
        );
    }

    println!(
        "{}/{} word root outlines have conflicts",
        word_root_conflicts_sorted_asc.len(),
        gen.word_root_conflict_dict.len()
    );

    let stdin = io::stdin();

    let fname = "syllables.json";

    let f = File::create(fname)?;

    gen.save_syllables(f)?;

    println!("Wrote syllables to {}", fname);

    let fname = "word_roots.json";

    let f = File::create(fname)?;

    gen.save_word_roots(f)?;

    println!("Wrote word roots to {}", fname);

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
