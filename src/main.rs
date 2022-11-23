mod chord;
mod dict_lookup;

use log::{debug, info, trace, warn};
use std::collections::{BTreeMap, HashSet};

use chord::Chord;

static SJP_DICT: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/odm.txt"));

static OUT_FILE_PATH: &'static str = "out.txt";

static PL_DIACRITICS: &'static str = "ąćęłńóśźż";

pub type ErrBox = Box<dyn std::error::Error>;

fn main() -> Result<(), ErrBox> {

    env_logger::init();

    info!("Starting...");

    // Helper set to discard whole words that have already been processed
    let mut word_set: HashSet<String> = dict_lookup::SHORTCUTS
        .into_iter()
        .map(|(word, _stroke)| word.to_string())
        .collect();

    let mut file_line_count = 0u32;
    let mut file_word_count = 0u64;
    let mut file_duplicate_count = 0u32;

    let mut file_non_alpha_count = 0u32;

    let mut distinct_roots = 0u32;

    let mut out_dict: BTreeMap<String, String> = dict_lookup::SHORTCUTS
        .into_iter()
        .map(|(word, stroke)| (word.to_string(), stroke.to_string()))
        .collect();

    // In the subsequent steps, we intend to match words against the
    // longest available affixes. Here we prepare prefixes and
    // suffixes sorted by length for that purpose.
    let prefixes_len_sorted = {
        let mut vec = dict_lookup::PREFIXES.into_iter().collect::<Vec<_>>();
        vec.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        vec
    };

    debug!("first prefix: {:?}", prefixes_len_sorted[0]);

    let suffixes_len_sorted = {
        let mut vec = dict_lookup::SUFFIXES.into_iter().collect::<Vec<_>>();
        vec.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        vec
    };

    let left_hand_combos_len_sorted = {
        let mut vec = dict_lookup::LEFT_HAND_COMBOS
            .into_iter()
            .collect::<Vec<_>>();
        vec.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        vec
    };

    let center_combos_len_sorted = {
        let mut vec = dict_lookup::CENTER_COMBOS.into_iter().collect::<Vec<_>>();
        vec.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        vec
    };

    let right_hand_combos_len_sorted = {
        let mut vec = dict_lookup::RIGHT_HAND_COMBOS
            .into_iter()
            .collect::<Vec<_>>();
        vec.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        vec
    };

    info!("Combo/affix dicts READY");

    for line in SJP_DICT.lines() {
        info!("line tick");
        for word in line.split(", ") {
            let sanitized = word.trim().to_lowercase();

            if sanitized.chars().any(|ch| {
                !(ch.is_ascii_alphabetic() || PL_DIACRITICS.contains(ch)) // Ascii alphabet + PL accents only
                    || ch.is_whitespace() // No multi-word entries
            }) {
                file_non_alpha_count += 1;
                continue;
            } else {
                // Input dataset duplicate check
                if word_set.contains(&sanitized) {
                    file_duplicate_count += 1;
                    continue;
                } else {
                    word_set.insert(sanitized.clone());
                }
            }

            info!("word tick: {}", word);

            file_word_count += 1;

            if !out_dict.contains_key(&sanitized) {

            let mut word_root = sanitized.clone();

                // Find all prefix matches
                for (pref_str, _pref_chord) in &prefixes_len_sorted {
                    if word_root.starts_with(*pref_str) {
                        debug!(
                            "prefix tick: Reducing {} with match {}",
                            word_root,
                            pref_str
                        );
                        word_root = word_root.trim_start_matches(*pref_str).to_string();
                    }
                }

                // Find all suffix matches
                for (suff_str, _suff_chord) in &suffixes_len_sorted {
                    if word_root.ends_with(*suff_str) {
                        debug!(
                            "suffix tick: Reducing {} with match {}",
                            word_root,
                            suff_str
                        );
                        word_root = word_root.trim_end_matches(*suff_str).to_string();
                    }
                }

                let mut root_chords: Vec<String> = Vec::new();

                let mut remaining_root_chars = word_root.clone();

                if out_dict.contains_key(&remaining_root_chars) {
                    debug!("Skipping known word root {}", remaining_root_chars);
                    continue;
                }

                'is_empty: while !remaining_root_chars.is_empty() {
                    debug!(
                        "non-empty root chord tick, {:?} ({:?} still present, {} chords generated)",
                        word_root,
                        remaining_root_chars,
                        root_chords.len(),
                    );
                    let mut ch = Chord::default();

                    // Find left-hand match
                    for (lh_str, lh_chord) in &left_hand_combos_len_sorted {
                        if remaining_root_chars.starts_with(*lh_str) {
                            let new_part: Chord = lh_chord.parse()?;

                            match ch.merge(&new_part) {
                                Ok(()) => {
                                    trace!(
                                        "lh tick: Adding {} to {} for match {}",
                                        new_part.to_string(),
                                        ch.to_string(),
                                        lh_str
                                    );

                                    remaining_root_chars = remaining_root_chars
                                        .trim_start_matches(*lh_str)
                                        .to_string();
                                    break;
                                }
                                // Move to new chord if new part cannot be added
                                Err(_e) => {
                                    warn!(
                                        "lh conflict: Cannot add {} to {} for match {}, starting new chord",
                                        new_part.to_string(),
                                        ch.to_string(),
                                        lh_str
                                    );
                                    root_chords.push(ch.to_string());
                                    continue 'is_empty;
                                }
                            }
                        }
                    }

                    // Find center match
                    for (center_str, center_chord) in &center_combos_len_sorted {
                        if remaining_root_chars.starts_with(*center_str) {
                            let new_part: Chord = center_chord.parse()?;

                            match ch.merge(&new_part) {
                                Ok(()) => {
                                    trace!(
                                        "center tick: Adding {} to {} for match {}",
                                        new_part.to_string(),
                                        ch.to_string(),
                                        center_str
                                    );

                                    remaining_root_chars = remaining_root_chars
                                        .trim_start_matches(*center_str)
                                        .to_string();
                                    break;
                                }
                                Err(_e) => {
                                    warn!(
                                        "center conflict: Cannot add {} to {} for match {}, starting new chord",
                                        new_part.to_string(),
                                        ch.to_string(),
                                        center_str
                                    );
                                    root_chords.push(ch.to_string());
                                    continue 'is_empty;
                                }
                            }
                        }
                    }

                    // Find right-hand match
                    for (rh_str, rh_chord) in &right_hand_combos_len_sorted {
                        if remaining_root_chars.starts_with(*rh_str) {
                            let new_part: Chord = rh_chord.parse()?;

                            match ch.merge(&new_part) {
                                Ok(()) => {
                                    trace!(
                                        "rh tick: Adding {} to {} for match {}",
                                        new_part.to_string(),
                                        ch.to_string(),
                                        rh_str
                                    );

                                    remaining_root_chars = remaining_root_chars
                                        .trim_start_matches(*rh_str)
                                        .to_string();
                                    break;
                                }
                                Err(_e) => {
                                    warn!(
                                        "rh conflict: Cannot add {} to {} for match {}, starting new chord",
                                        new_part.to_string(),
                                        ch.to_string(),
                                        rh_str
                                    );
                                    root_chords.push(ch.to_string());
                                    continue 'is_empty;
                                }
                            }
                        }
                    }

                    root_chords.push(ch.to_string());
                }

                distinct_roots += 1;
                info!("New word root: {} => {}", word_root, root_chords.join("/"));
                out_dict.insert(word_root.clone(), root_chords.join("/"));
            }
        }
        file_line_count += 1;
    }

    println!(
	"Processed {} words in {} lines, discarded {} duplicates and {} non-alpha and/or multi-word entries",
	file_word_count, file_line_count, file_duplicate_count, file_non_alpha_count
    );
    println!("Distinct word roots: {}", distinct_roots);

    Ok(())
}
