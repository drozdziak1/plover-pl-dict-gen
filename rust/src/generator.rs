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
                .map(|(txt, chord)| format!("-{}", chord).parse().map(|ch| ((*txt).into(), ch)))
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

    /// NOTE: Only root recipe is added to the dictionary, but the
    /// complete chord set is returned.
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
            .insert(root_chords.get_word().into(), root_chords.clone());
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

        debug!("WORD: {}", word);

        let mut word_root = word.clone();

        let mut prefix: Option<ChordSeqItem> = None;

        trace!("ATTEMPT PREFIX");
        // Find all prefix matches
        if let Some((pref_str, pref_chord)) =
            find_longest_affix(&word_root, &self.prefixes_len_sorted, 2, true)
        {
            debug!("REDUCE PREFIX:\t{}-", pref_str);
            word_root = word_root.strip_prefix(&pref_str).unwrap().to_string();
            prefix = Some(ChordSeqItem::Prefix(
                pref_str.clone().into(),
                pref_chord.clone(),
            ));
        }

        let mut suffix: Option<ChordSeqItem> = None;

        trace!("ATTEMPT SUFFIX");
        // Find all suffix matches
        if let Some((suff_str, suff_chord)) =
            find_longest_affix(&word_root, &self.suffixes_len_sorted, 2, false)
        {
            trace!("REDUCE SUFFIX:\t-{}", suff_str,);
            word_root = word_root.strip_suffix(&suff_str).unwrap().to_string();
            suffix = Some(ChordSeqItem::Suffix(
                suff_str.clone().into(),
                suff_chord.clone(),
            ));
        }

        // Skip word roots achievable with prefixes/suffixes only
        if word_root.is_empty() {
            debug!("SKIP CONSUMED:\t{}", word);
        }

        let mut remaining_root_chars = word_root.clone();

        // Skip exact existing word root matches
        let mut root_chords: ChordSequence =
            if let Some(chords) = self.word_root_dict.get(&word_root.clone().into()) {
                debug!("SKIP EXACT-ROOT:\t{}, {}", word_root, chords.to_string());
                remaining_root_chars = "".to_string();
                chords.clone()
            } else {
                ChordSequence::new(vec![])
            };

        'is_empty: while !remaining_root_chars.is_empty() {
            let mut current_chord_str = "".to_string();
            let mut ch = Chord::default();

            let mut roots_found = false;

            trace!("ATTEMPT KNOWN-ROOT");
            // Find longest existing root within this one
            while let Some((known_root_str, known_root_chords)) =
                find_longest_affix(&remaining_root_chars, &self.word_root_dict, 3, true)
            {
                roots_found = true;
                remaining_root_chars = remaining_root_chars
                    .strip_prefix(known_root_str.as_str())
                    .unwrap()
                    .to_string();

                debug!(
                    "REDUCE KNOWN-ROOT:\t{} ({})",
                    known_root_str,
                    known_root_chords.to_string(),
                );
                root_chords.items.push(ChordSeqItem::KnownRootEntry(
                    known_root_str.to_string(),
                    known_root_chords,
                ));
            }

            trace!("ATTEMPT LEFT-HAND");
            // Find longest left-hand cluster
            if let Some((lh_str, lh_chord)) =
                find_longest_affix(&remaining_root_chars, &self.lh_combos_len_sorted, 1, true)
            {
                let new_part: Chord = lh_chord;

                match ch.merge(&new_part) {
                    Ok(()) => {
                        current_chord_str.push_str(&lh_str);
                        remaining_root_chars = remaining_root_chars
                            .strip_prefix(lh_str.as_str())
                            .unwrap()
                            .to_string();
                        debug!("REDUCE LEFT-HAND:\t{} ({}) ", lh_str, new_part.to_string());
                    }
                    Err(_e) => {
                        unreachable!();
                    }
                }
            }

            trace!("ATTEMPT CENTER");
            // Find center match
            while let Some((center_str, center_chord)) = find_longest_affix(
                &remaining_root_chars,
                &self.center_combos_len_sorted,
                1,
                true,
            ) {
                let new_part: Chord = center_chord;

                match ch.merge(&new_part) {
                    Ok(()) => {
                        current_chord_str.push_str(&center_str);
                        remaining_root_chars = remaining_root_chars
                            .strip_prefix(center_str.as_str())
                            .unwrap()
                            .to_string();
                        debug!("REDUCE CENTER:\t{} ({}) ", center_str, new_part.to_string());
                    }
                    Err(_e) => {
                        debug!(
                            "CONFLICT CENTER:\t{} + {}, {} + {}",
                            word_root.strip_suffix(&remaining_root_chars).unwrap(),
                            center_str,
                            ch.to_string(),
                            new_part.to_string(),
                        );
                        break;
                    }
                }
            }

            trace!("ATTEMPT RIGHT_HAND");
            // Find right-hand match
            while let Some((rh_str, rh_chord)) =
                find_longest_affix(&remaining_root_chars, &self.rh_combos_len_sorted, 1, true)
            {
                let new_part: Chord = rh_chord;

                match ch.merge(&new_part) {
                    Ok(()) => {
                        current_chord_str.push_str(&rh_str);
                        remaining_root_chars = remaining_root_chars
                            .strip_prefix(rh_str.as_str())
                            .unwrap()
                            .to_string();
                        debug!("REDUCE RIGHT_HAND:\t{} ({}) ", rh_str, new_part.to_string());
                    }
                    Err(_e) => {
                        debug!(
                            "CONFLICT RIGHT-HAND:\t{} + {}, {} + {}",
                            word_root.strip_suffix(&remaining_root_chars).unwrap(),
                            rh_str,
                            ch.to_string(),
                            new_part.to_string(),
                        );
                        break;
                    }
                }
            }

            if ch == Chord::default() && !roots_found {
                error!("INFINITE-LOOP: {}, {} left", word, remaining_root_chars);
                return Err(format!("infinite loop on {}", word).into());
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

pub fn find_longest_affix<const ASC: bool, T: Clone>(
    needle: &str,
    haystack: &BTreeMap<LenSortableString<ASC>, T>,
    min_match_len: usize,
    is_prefix: bool,
) -> Option<(String, T)> {
    let ndl_vec = needle.chars().collect::<Vec<_>>();
    let needle_len = ndl_vec.len();

    if needle_len == 0 {
        return None;
    }

    for n_chars in (min_match_len..=needle_len).rev() {
        let slice_range = if is_prefix {
            0..n_chars
        } else {
            (needle_len - n_chars)..needle_len
        };

        let slice = ndl_vec.get(slice_range).or_else(|| {
            error!("Computed slice could not be retrieved");
            None
        })?;

        let slice_string = slice.iter().collect::<String>();

        if let Some(item) = haystack.get(&slice_string.clone().into()) {
            trace!("HIT {}", slice_string);
            return Some((slice_string, item.clone()));
        }

        trace!("MISS {}", slice_string);
    }
    None
}
