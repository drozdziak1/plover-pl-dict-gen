use log::{debug, error, info, trace, warn};

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    chord::{Chord, ChordSeqItem, ChordSequence},
    dict_lookup,
    utils::LenSortableString,
    ErrBox,
};

pub struct Generator {
    prefixes_len_sorted: BTreeMap<LenSortableString<false>, Chord>,
    suffixes_len_sorted: BTreeMap<LenSortableString<false>, Chord>,
    lh_combos_len_sorted: BTreeMap<LenSortableString<false>, Chord>,
    center_combos_len_sorted: BTreeMap<LenSortableString<false>, Chord>,
    rh_combos_len_sorted: BTreeMap<LenSortableString<false>, Chord>,
    pub word_root_dict: BTreeMap<LenSortableString<false>, ChordSequence>,
}

impl Generator {
    pub fn new() -> Result<Self, ErrBox> {
        // In the subsequent steps, we intend to match words against the
        // longest available affixes. Here we prepare prefixes and
        // suffixes sorted by descending length for that purpose.
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
        let lh_combos_len_sorted: BTreeMap<LenSortableString<false>, Chord> =
            dict_lookup::LEFT_HAND_COMBOS
                .into_iter()
                .map(|(txt, chord)| chord.parse().map(|ch| ((*txt).into(), ch)))
                .collect::<Result<_, _>>()?;

        let center_combos_len_sorted: BTreeMap<LenSortableString<false>, Chord> =
            dict_lookup::CENTER_COMBOS
                .into_iter()
                .map(|(txt, chord)| chord.parse().map(|ch| ((*txt).into(), ch)))
                .collect::<Result<_, _>>()?;

        let rh_combos_len_sorted: BTreeMap<LenSortableString<false>, Chord> =
            dict_lookup::RIGHT_HAND_COMBOS
                .into_iter()
                .map(|(txt, chord)| chord.parse().map(|ch| ((*txt).into(), ch)))
                .collect::<Result<_, _>>()?;

        let word_root_dict: BTreeMap<LenSortableString<false>, ChordSequence> =
            dict_lookup::SHORTCUTS
                .into_iter()
                .map(|(word, chord)| {
                    chord.parse::<Chord>().map(|ch| {
                        (
                            LenSortableString(word.to_string()),
                            vec![ChordSeqItem::RootChord(chord.to_string(), ch)].into(),
                        )
                    })
                })
                .collect::<Result<_, _>>()?;

        Ok(Self {
            prefixes_len_sorted,
            suffixes_len_sorted,
            lh_combos_len_sorted,
            center_combos_len_sorted,
            rh_combos_len_sorted,
            word_root_dict,
        })
    }

    pub fn add_word_root(&mut self, word: &str) -> Result<ChordSequence, ErrBox> {
        let chords = self.gen_word_chords(word)?;
        let mut root_chords = chords.clone();
        root_chords.items = chords
            .items
            .iter()
            .filter_map(|item| match item {
                &ChordSeqItem::KnownRootEntry(_, _) | &ChordSeqItem::RootChord(_, _) => Some(item),
                _other => None,
            })
            .cloned()
            .collect();

        self.word_root_dict
            .insert(chords.get_word().into(), root_chords.clone());
        Ok(chords)
    }

    /// Returns Err on sanitization problems
    pub fn gen_word_chords(&self, word: &str) -> Result<ChordSequence, ErrBox> {
        let word = word.trim().to_lowercase();

        // Sanitize
        if word.chars().any(|ch| {
            !(ch.is_ascii_alphabetic() || dict_lookup::PL_DIACRITICS.contains(ch)) // Ascii alphabet + PL accents only
		|| ch.is_whitespace() // No multi-word entries
        }) {
            return Err(format!("{:?} rejected - must be a single word made up exclusively of Polish and latin characters.", word).into());
        }
        // let sjp_sanitized_len_sorted: BTreeSet<LenSortableString<true>> = dict_lookup::SJP_DICT
        //     .lines()
        //     .map(|l| {
        //         l.split(", ").filter_map(|word| {
        //             let sanitized = word.trim().to_lowercase();
        //             if sanitized.chars().any(|ch| {
        //                 !(ch.is_ascii_alphabetic() || dict_lookup::PL_DIACRITICS.contains(ch)) // Ascii alphabet + PL accents only
        //             || ch.is_whitespace() // No multi-word entries
        //             }) || sanitized.chars().count() < 2
        //             {
        //                 None
        //             } else {
        //                 Some(sanitized.into())
        //             }
        //         })
        //     })
        //     .flatten()
        //     .collect();

        trace!("WORD: {}", word);

        let mut word_root = word.clone();

        let mut prefix: Option<ChordSeqItem> = None;

        // Find all prefix matches
        for (pref_str, pref_chord) in self.prefixes_len_sorted.iter() {
            if word_root.starts_with(&pref_str.0) {
                trace!(
                    "REDUCE PREFIX:\t{}-",
                    pref_str.0,
                );
                word_root = word_root.trim_start_matches(&pref_str.0).to_string();
                prefix = Some(ChordSeqItem::Prefix(
                    pref_str.clone().into(),
                    pref_chord.clone(),
                ));
                break;
            }
        }

        let mut suffix: Option<ChordSeqItem> = None;
        // Find all suffix matches
        for (suff_str, suff_chord) in self.suffixes_len_sorted.iter() {
            if word_root.ends_with(&suff_str.0) {
                trace!(
                    "REDUCE SUFFIX:\t-{}",
                    suff_str.0,
                );
                word_root = word_root.trim_end_matches(&suff_str.0).to_string();
                suffix = Some(ChordSeqItem::Suffix(
                    suff_str.clone().into(),
                    suff_chord.clone(),
                ));
                break;
            }
        }

        // Skip word roots achievable with prefixes/suffixes only
        if word_root.is_empty() {
            debug!("SKIP CONSUMED:\t{}", word);
        }

        let mut remaining_root_chars = word_root.clone();

        // Skip exact existing word root matches
        let mut root_chords: ChordSequence =
            if let Some(chords) = self.word_root_dict.get(&word_root.clone().into()) {
                debug!("SKIP EXACT-ROOT:\t{}, {:?}", word_root, chords.to_string());
                remaining_root_chars = "".to_string();
                chords.clone()
            } else {
                ChordSequence::new(vec![])
            };

        'is_empty: while !remaining_root_chars.is_empty() {
            let mut current_chord_str = "".to_string();
            let mut ch = Chord::default();

            // Find existing roots within this one
            for (known_root_str, known_root_chords) in self.word_root_dict.iter() {
                if remaining_root_chars.starts_with(known_root_str.0.as_str())
                    && known_root_str.0.chars().count() > 1
                {
                    remaining_root_chars = remaining_root_chars
                        .trim_start_matches(known_root_str.0.as_str())
                        .to_string();

                    trace!(
                        "REDUCE KNOWN-ROOT:\t{} ({})",
                        known_root_str.0,
                        known_root_chords.to_string(),
                    );
                    root_chords.items.push(ChordSeqItem::KnownRootEntry(
                        known_root_str.to_string(),
                        known_root_chords.clone(),
                    ));
                }
            }

            // Find left-hand match
            for (lh_str, lh_chord) in self.lh_combos_len_sorted.iter() {
                if remaining_root_chars.starts_with(lh_str.0.as_str()) {
                    let new_part: Chord = lh_chord.clone();

                    match ch.merge(&new_part) {
                        Ok(()) => {
                            current_chord_str.push_str(&lh_str.0);
                            remaining_root_chars = remaining_root_chars
                                .trim_start_matches(lh_str.0.as_str())
                                .to_string();
                            trace!(
                                "REDUCE LEFT-HAND:\t{} ({}) ",
                                lh_str,
				lh_chord.to_string()
                            );
                            break;
                        }
                        Err(_e) => {
                            unreachable!();
                        }
                    }
                }
            }

            // Find center match
            for (center_str, center_chord) in self.center_combos_len_sorted.iter() {
                if remaining_root_chars.starts_with(center_str.0.as_str()) {
                    let new_part: Chord = center_chord.clone();

                    match ch.merge(&new_part) {
                        Ok(()) => {
                            current_chord_str.push_str(&center_str.0);

                            trace!(
                                "REDUCE CENTER:\t{}, ({})",
                                center_str,
                                center_chord.to_string(),
                            );

                            remaining_root_chars = remaining_root_chars
                                .trim_start_matches(center_str.0.as_str())
                                .to_string();

                            break;
                        }
                        Err(_e) => {
                            debug!(
                                "CONFLICT CENTER:\t{} + {}, {} + {}",
                                word_root.trim_end_matches(&remaining_root_chars),
                                center_str,
                                ch.to_string(),
                                new_part.to_string(),
                            );
                            root_chords
                                .items
                                .push(ChordSeqItem::RootChord(current_chord_str, ch));
                            continue 'is_empty;
                        }
                    }
                }
            }

            // Find right-hand match
            for (rh_str, rh_chord) in self.rh_combos_len_sorted.iter() {
                if remaining_root_chars.starts_with(rh_str.0.as_str()) {
                    let new_part: Chord = rh_chord.clone();

                    match ch.merge(&new_part) {
                        Ok(()) => {

                            trace!(
                                "REDUCE RIGHT-HAND:\t{}, ({})",
                                rh_str,
                                rh_chord.to_string(),
                            );

                            current_chord_str.push_str(&rh_str.0);

                            remaining_root_chars = remaining_root_chars
                                .trim_start_matches(rh_str.0.as_str())
                                .to_string();
                            break;
                        }
                        Err(_e) => {
                            debug!(
                                "CONFLICT RIGHT-HAND:\t{} + {}, {} + {}",
                                word_root.trim_end_matches(&remaining_root_chars),
                                rh_str,
                                ch.to_string(),
                                new_part.to_string(),
                            );
                            root_chords
                                .items
                                .push(ChordSeqItem::RootChord(current_chord_str, ch));
                            continue 'is_empty;
                        }
                    }
                }
            }

            root_chords
                .items
                .push(ChordSeqItem::RootChord(current_chord_str, ch));
        }

        let v: Vec<_> = prefix
            .into_iter()
            .chain(root_chords.items.into_iter())
            .chain(suffix.into_iter())
            .collect();

        Ok(ChordSequence::new(v))
    }
}
