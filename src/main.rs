mod chord;
mod dict_lookup;

use std::collections::{BTreeMap, HashSet};

use chord::Chord;

static SJP_DICT: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/odm.txt"));

static OUT_FILE_PATH: &'static str = "out.txt";

pub type ErrBox = Box<dyn std::error::Error>;

fn main() -> Result<(), ErrBox> {
    println!("Starting...");

    // Helper set to discard words that have already been processed
    let mut word_set: HashSet<String> = dict_lookup::SHORTCUTS
        .into_iter()
        .map(|(word, stroke)| word.to_string())
        .collect();

    let mut file_line_count = 0u32;
    let mut file_word_count = 0u64;
    let mut file_duplicate_count = 0u32;

    let mut file_non_alpha_count = 0u32;

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
        let mut vec = dict_lookup::CENTER_COMBOS
            .into_iter()
            .collect::<Vec<_>>();
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

    println!("Combo/affix dicts READY");

    for line in SJP_DICT.lines() {
	println!("line tick");
        for word in line.split(", ") {
	    let mut sanitized = word.trim().to_lowercase();

            if sanitized
                .chars()
                .any(|ch| !ch.is_alphabetic() || ch.is_whitespace())
            {
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

	    println!("word tick: {}", word);

            file_word_count += 1;

            if !out_dict.contains_key(&sanitized) {
                // Find all prefix matches
                for (pref_str, _pref_chord) in &prefixes_len_sorted {
                    if sanitized.starts_with(*pref_str) {
			println!("prefix tick: Reducing {} with match {}", sanitized, pref_str);
                        sanitized = sanitized.trim_start_matches(*pref_str).to_string();
                    }
                }

                // Find all suffix matches
                for (suff_str, _suff_chord) in &suffixes_len_sorted {
                    if sanitized.ends_with(*suff_str) {
			println!("suffix tick: Reducing {} with match {}", sanitized, suff_str);
                        sanitized = sanitized.trim_end_matches(*suff_str).to_string();
                    }
                }

                let mut root_chords: Vec<String> = Vec::new();

                let mut remaining_root_chars = sanitized.clone();

                while !remaining_root_chars.is_empty() {
		    println!("non-empty tick, {:?} ({:?} still present)", sanitized, remaining_root_chars);
                    let mut ch = Chord::default();

                    // Find left-hand match
                    for (lh_str, lh_chord) in &left_hand_combos_len_sorted {
                        if remaining_root_chars.starts_with(*lh_str) {
                            remaining_root_chars =
                                remaining_root_chars.trim_start_matches(*lh_str).to_string();
                            let new_part: Chord = lh_chord.parse()?;

			    println!("lh tick: Adding {} for match {}", new_part.to_string(), lh_str);

                            ch.merge(&new_part)?;

                            break;
                        }
                    }

                    // Find center match
                    for (center_str, center_chord) in &center_combos_len_sorted {
                        if remaining_root_chars.starts_with(*center_str) {
                            remaining_root_chars = remaining_root_chars
                                .trim_start_matches(*center_str)
                                .to_string();
                            let new_part: Chord = center_chord.parse()?;

			    println!("center tick: Adding {} for match {}", new_part.to_string(), center_str);

                            ch.merge(&new_part)?;

                            break;
                        }
                    }

                    // Find right-hand match
                    for (rh_str, rh_chord) in &right_hand_combos_len_sorted {
                        if remaining_root_chars.starts_with(*rh_str) {
                            remaining_root_chars =
                                remaining_root_chars.trim_start_matches(*rh_str).to_string();

                            // Disambiguate right part
                            let rh_chord_with_hyphen = format!("-{}", rh_chord);

                            let new_part: Chord = rh_chord_with_hyphen.parse()?;

			    println!("rh tick: Adding {} for match {}", new_part.to_string(), rh_str);


                            ch.merge(&new_part)?;

                            break;
                        }
                    }
                    root_chords.push(ch.to_string());
                }

		out_dict.insert(sanitized.clone(), root_chords.join("/"));
            }
        }
        file_line_count += 1;
    }

    println!(
	"Processed {} words in {} lines, discarded {} duplicates and {} non-alpha and/or multi-word entries",
	file_word_count, file_line_count, file_duplicate_count, file_non_alpha_count
    );

    Ok(())
}
