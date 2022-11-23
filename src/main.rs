mod chord;
mod dict_lookup;

use log::{debug, info, trace, warn};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    io,
};

use chord::Chord;

static SJP_DICT: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/odm.txt"));

static PL_DIACRITICS: &'static str = "ąćęłńóśźż";

pub type ErrBox = Box<dyn std::error::Error>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LenSortableString<const ASCENDING: bool>(String);

// The Eq and PartialEq impls below invert String's lexicographic
// ordering, favoring length before contents

impl<const ASC: bool> PartialOrd for LenSortableString<ASC> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if ASC {
            match self.0.chars().count().partial_cmp(&other.0.chars().count()) {
                Some(Ordering::Equal) => self.0.partial_cmp(&other.0),
                other => other,
            }
        } else {
            match other.0.chars().count().partial_cmp(&self.0.chars().count()) {
                Some(Ordering::Equal) => other.0.partial_cmp(&self.0),
                other => other,
            }
        }
    }
}

impl<const ASC: bool> Ord for LenSortableString<ASC> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if ASC {
            match self.0.chars().count().cmp(&other.0.chars().count()) {
                Ordering::Equal => self.0.cmp(&other.0),
                other => other,
            }
        } else {
            match other.0.chars().count().cmp(&self.0.chars().count()) {
                Ordering::Equal => other.0.cmp(&self.0),
                other => other,
            }
        }
    }
}

impl<const ASC: bool> From<String> for LenSortableString<ASC> {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl<const ASC: bool> Into<String> for LenSortableString<ASC> {
    fn into(self) -> String {
        self.0
    }
}

impl<const ASC: bool> From<&str> for LenSortableString<ASC> {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl<const ASC: bool> Display for LenSortableString<ASC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn main() -> Result<(), ErrBox> {
    env_logger::init();

    info!("Starting...");

    let mut word_root_dict: BTreeMap<LenSortableString<false>, Vec<Chord>> = dict_lookup::SHORTCUTS
        .into_iter()
        .map(|(word, stroke)| {
            trace!("Preparing word root for {}, stroke {}", word, stroke);
            stroke
                .parse::<Chord>()
                .map(|ch| (LenSortableString(word.to_string()), vec![ch]))
        })
        .collect::<Result<_, _>>()?;

    let shortcut_roots = word_root_dict.len();

    // In the subsequent steps, we intend to match words against the
    // longest available affixes. Here we prepare prefixes and
    // suffixes sorted by length for that purpose.
    let prefixes_len_sorted: BTreeMap<LenSortableString<false>, Chord> = dict_lookup::PREFIXES
        .into_iter()
        .map(|(txt, chord)| chord.parse().map(|ch| ((*txt).into(), ch)))
        .collect::<Result<_, _>>()?;

    let suffixes_len_sorted: BTreeMap<LenSortableString<false>, Chord> = dict_lookup::SUFFIXES
        .into_iter()
        .map(|(txt, chord)| chord.parse().map(|ch| ((*txt).into(), ch)))
        .collect::<Result<_, _>>()?;

    //  With the left/center/right combos the story is similar. We
    //  wish to match against the longest available word part
    let left_hand_combos_len_sorted: BTreeMap<LenSortableString<false>, Chord> =
        dict_lookup::LEFT_HAND_COMBOS
            .into_iter()
            .map(|(txt, chord)| chord.parse().map(|ch| ((*txt).into(), ch)))
            .collect::<Result<_, _>>()?;

    let center_combos_len_sorted: BTreeMap<LenSortableString<false>, Chord> =
        dict_lookup::CENTER_COMBOS
            .into_iter()
            .map(|(txt, chord)| chord.parse().map(|ch| ((*txt).into(), ch)))
            .collect::<Result<_, _>>()?;

    let right_hand_combos_len_sorted: BTreeMap<LenSortableString<false>, Chord> =
        dict_lookup::RIGHT_HAND_COMBOS
            .into_iter()
            .map(|(txt, chord)| chord.parse().map(|ch| ((*txt).into(), ch)))
            .collect::<Result<_, _>>()?;

    info!("Combo/affix dicts READY");

    let sjp_sanitized_len_sorted: BTreeSet<LenSortableString<true>> = SJP_DICT
        .lines()
        .map(|l| {
            l.split(", ").filter_map(|word| {
                let sanitized = word.trim().to_lowercase();

                if sanitized.chars().any(|ch| {
                    !(ch.is_ascii_alphabetic() || PL_DIACRITICS.contains(ch)) // Ascii alphabet + PL accents only
                    || ch.is_whitespace() // No multi-word entries
                }) || sanitized.chars().count() < 2
                {
                    None
                } else {
                    Some(sanitized.into())
                }
            })
        })
        .flatten()
        .collect();

    info!("SJP raw word set READY");

    let file_word_count = sjp_sanitized_len_sorted.len();

    for word in sjp_sanitized_len_sorted {
        let word = word.0;
        let mut word_root = word.clone();

        trace!("WORD: {}", word);

        // Find all prefix matches
        for (pref_str, _pref_chord) in &prefixes_len_sorted {
            if word_root.starts_with(&pref_str.0) {
                trace!(
                    "REDUCE PREFIX: {:?} + {:?}",
                    pref_str.0,
                    word_root.trim_start_matches(&pref_str.0),
                );
                word_root = word_root.trim_start_matches(&pref_str.0).to_string();
                break;
            }
        }

        // Find all suffix matches
        for (suff_str, _suff_chord) in &suffixes_len_sorted {
            if word_root.ends_with(&suff_str.0) {
                trace!(
                    "REDUCE SUFFIX: {:?} + {:?}",
                    word_root.trim_end_matches(&suff_str.0),
                    suff_str.0,
                );
                word_root = word_root.trim_end_matches(&suff_str.0).to_string();
                break;
            }
        }

        // Skip word roots achievable with prefixes/suffixes only
        if word_root.is_empty() {
            debug!("SKIP CONSUMED: {}", word);
            continue;
        }

        // Skip exact existing word root matches
        if word_root_dict.contains_key(&word_root.clone().into()) {
            debug!(
                "SKIP EXACT-ROOT: {}, {}, {:?}",
                word,
                word_root,
                word_root_dict
                    .get(&word_root.clone().into())
                    .map(|chs| chs.iter().map(|ch| ch.to_string()).collect::<Vec<_>>()),
            );
            continue;
        }

        let mut root_chords: Vec<Chord> = Vec::new();

        let mut remaining_root_chars = word_root.clone();

        'is_empty: while !remaining_root_chars.is_empty() {
            let mut ch = Chord::default();

            // Find existing roots within this one
            for (known_root_str, known_root_chords) in word_root_dict.iter() {
                if remaining_root_chars.starts_with(known_root_str.0.as_str()) {
                    trace!(
                        "REDUCE KNOWN-ROOT: {}: {:?} + {:?}, {:?} + {:?}",
                        word_root,
                        word_root.trim_end_matches(&remaining_root_chars),
                        known_root_str.0,
                        root_chords
                            .iter()
                            .map(|ch| ch.to_string())
                            .collect::<Vec<_>>(),
                        known_root_chords
                            .iter()
                            .map(|ch| ch.to_string())
                            .collect::<Vec<_>>(),
                    );
                    remaining_root_chars = remaining_root_chars
                        .trim_start_matches(known_root_str.0.as_str())
                        .to_string();
                    root_chords.append(&mut known_root_chords.clone());
                }
            }

            // Find left-hand match
            for (lh_str, lh_chord) in &left_hand_combos_len_sorted {
                if remaining_root_chars.starts_with(lh_str.0.as_str()) {
                    let new_part: Chord = lh_chord.clone();

                    match ch.merge(&new_part) {
                        Ok(()) => {
                            trace!(
                                "REDUCE LEFT-HAND: {}: {:?} + {:?}, {:?} + {:?}",
                                word_root,
                                word_root.trim_end_matches(&remaining_root_chars),
                                ch.to_string(),
                                lh_str,
                                new_part.to_string(),
                            );

                            remaining_root_chars = remaining_root_chars
                                .trim_start_matches(lh_str.0.as_str())
                                .to_string();
                            break;
                        }
                        // Move to new chord if new part cannot be added
                        Err(_e) => {
                            warn!(
                                "CONFLICT LEFT-HAND: {}: {:?} + {:?}, {:?} + {:?}",
                                word_root,
                                word_root.trim_end_matches(&remaining_root_chars),
                                ch.to_string(),
                                lh_str,
                                new_part.to_string(),
                            );
                            root_chords.push(ch);
                            continue 'is_empty;
                        }
                    }
                }
            }

            // Find center match
            for (center_str, center_chord) in &center_combos_len_sorted {
                if remaining_root_chars.starts_with(center_str.0.as_str()) {
                    let new_part: Chord = center_chord.clone();

                    match ch.merge(&new_part) {
                        Ok(()) => {
                            trace!(
                                "REDUCE CENTER: {}: {:?} + {:?}, {:?} + {:?}",
                                word_root,
                                word_root.trim_end_matches(&remaining_root_chars),
                                ch.to_string(),
                                center_str,
                                new_part.to_string(),
                            );

                            remaining_root_chars = remaining_root_chars
                                .trim_start_matches(center_str.0.as_str())
                                .to_string();
                            break;
                        }
                        Err(_e) => {
                            trace!(
                                "CONFLICT CENTER: {}: {:?} + {:?}, {:?} + {:?}",
                                word_root,
                                word_root.trim_end_matches(&remaining_root_chars),
                                ch.to_string(),
                                center_str,
                                new_part.to_string(),
                            );
                            root_chords.push(ch);
                            continue 'is_empty;
                        }
                    }
                }
            }

            // Find right-hand match
            for (rh_str, rh_chord) in &right_hand_combos_len_sorted {
                if remaining_root_chars.starts_with(rh_str.0.as_str()) {
                    let new_part: Chord = rh_chord.clone();

                    match ch.merge(&new_part) {
                        Ok(()) => {
                            trace!(
                                "REDUCE RIGHT-HAND: {}: {:?} + {:?}, {:?} + {:?}",
                                word_root,
                                word_root.trim_end_matches(&remaining_root_chars),
                                ch.to_string(),
                                rh_str,
                                new_part.to_string(),
                            );

                            remaining_root_chars = remaining_root_chars
                                .trim_start_matches(rh_str.0.as_str())
                                .to_string();
                            break;
                        }
                        Err(_e) => {
                            trace!(
                                "CONFLICT RIGHT-HAND: {}: {:?} + {:?}, {:?} + {:?}",
                                word_root,
                                word_root.trim_end_matches(&remaining_root_chars),
                                ch.to_string(),
                                rh_str,
                                new_part.to_string(),
                            );
                            root_chords.push(ch);
                            continue 'is_empty;
                        }
                    }
                }
            }

            root_chords.push(ch);
        }

        info!(
            "New word root: {} => {:?}",
            word_root,
            root_chords
                .iter()
                .map(|ch| ch.to_string())
                .collect::<Vec<_>>()
        );
        word_root_dict.insert(word_root.into(), root_chords);
    }

    println!(
        "Processed {} words, using {} shortcut + {} new word roots:\n{:?}",
        file_word_count,
        shortcut_roots,
        word_root_dict.len() - shortcut_roots,
        word_root_dict.keys().collect::<Vec<_>>(),
    );

    let stdin = io::stdin();

    loop {
        let mut line_buf = String::new();

        stdin.read_line(&mut line_buf)?;
    }

    Ok(())
}
