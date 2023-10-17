use log::{debug, error, info, trace, warn};
use regex::Regex;

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
    pub chunk_dict: BTreeMap<LenSortableString<false>, ChordSequence>,
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

        let chunk_dict: BTreeMap<LenSortableString<false>, ChordSequence> = Default::default();

        Ok(Self {
            prefixes_len_sorted,
            suffixes_len_sorted,
            lh_combos_len_sorted,
            center_combos_len_sorted,
            rh_combos_len_sorted,
            word_root_dict,
            chunk_dict,
        })
    }

    /// NOTE: Only root recipe is added to the dictionary, but the
    /// complete chord set is returned.
    pub fn add_word_root(&mut self, word: &str) -> Result<ChordSequence, ErrBox> {
        let (word_chords, chunk_chords) = self.gen_word_chords(word)?;
        let mut root_chords = word_chords.clone();
        root_chords.items = word_chords
            .items
            .iter()
            .filter_map(|item| match item {
                &ChordSeqItem::RootChord(_, _) => Some(item),
                _other => None,
            })
            .cloned()
            .collect();

        self.word_root_dict
            .insert(root_chords.get_word().into(), root_chords.clone());
        for chunk in chunk_chords {
            self.chunk_dict
                .insert(chunk.get_word().into(), chunk.clone());
        }
        Ok(word_chords)
    }

    /// Generate
    pub fn gen_word_chords(
        &self,
        word: &str,
    ) -> Result<(ChordSequence, Vec<ChordSequence>), ErrBox> {
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
        if !dict_lookup::PREFIX_EXCEPTIONS.contains(&word_root) {
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
        } else {
            trace!("SKIP PREFIX EXCEPTION");
        }

        let mut suffix: Option<ChordSeqItem> = None;

        trace!("ATTEMPT SUFFIX");
        // Find all suffix matches
        if !dict_lookup::SUFFIX_EXCEPTIONS.contains(&word_root) {
            if let Some((suff_str, suff_chord)) =
                find_longest_affix(&word_root, &self.suffixes_len_sorted, 2, false)
            {
                debug!("REDUCE SUFFIX:\t-{}", suff_str,);
                word_root = word_root.strip_suffix(&suff_str).unwrap().to_string();
                suffix = Some(ChordSeqItem::Suffix(
                    suff_str.clone().into(),
                    suff_chord.clone(),
                ));
            }
        } else {
            trace!("SKIP SUFFIX EXCEPTION");
        }

        let mut root_chords = Vec::new();
        let mut new_chunks = Vec::new();

        if let Some(chords) = self.word_root_dict.get(&word_root.clone().into()).cloned() {
            debug!("SKIP EXACT-ROOT:\t{} ({})", word_root, chords.to_string());
            root_chords = chords.items;
        } else {
            for chunk in syllable_split(&word_root) {
                let mut chunk_chords = if let Some(chunk_chords) =
                    self.chunk_dict.get(&chunk.clone().into()).cloned()
                {
                    debug!(
                        "SKIP EXACT-CHUNK:\t{} ({})",
                        chunk,
                        chunk_chords.to_string()
                    );
                    chunk_chords
                } else {
                    let chunk_chords = self.gen_chunk_chords(&chunk)?;
                    // This is an unknown chunk, add it to new chunks
                    new_chunks.push(chunk_chords.clone());
                    chunk_chords
                };

                root_chords.append(&mut chunk_chords.items);
            }
        }

        let chords: Vec<_> = prefix
            .into_iter()
            .chain(root_chords.into_iter())
            .chain(suffix.into_iter())
            .collect();

        Ok((ChordSequence::new(chords), new_chunks))
    }

    /// Returns Err on sanitization problems
    pub fn gen_chunk_chords(&self, chunk: &str) -> Result<ChordSequence, ErrBox> {
        debug!("CHUNK: {}", chunk);

        let mut remaining_chunk_chars = chunk.to_owned();

        let mut chunk_chords: ChordSequence = ChordSequence::new(vec![]);

        while !remaining_chunk_chars.is_empty() {
            let mut current_chord_str = "".to_string();
            let mut ch = Chord::default();

            trace!("ATTEMPT LEFT-HAND");
            // Find longest left-hand cluster
            while let Some((lh_str, lh_chord)) =
                find_longest_affix(&remaining_chunk_chars, &self.lh_combos_len_sorted, 1, true)
            {
                let new_part: Chord = lh_chord;

                match ch.merge(&new_part) {
                    Ok(()) => {
                        current_chord_str.push_str(&lh_str);
                        remaining_chunk_chars = remaining_chunk_chars
                            .strip_prefix(lh_str.as_str())
                            .unwrap()
                            .to_string();
                        debug!("REDUCE LEFT-HAND:\t{} ({}) ", lh_str, new_part.to_string());
                    }
                    Err(e) => {
                        debug!("CONFLICT LEFT-HAND: {}", e.to_string());
			break;
                    }
                }
            }

            trace!("ATTEMPT CENTER");
            // Find center match
            while let Some((center_str, center_chord)) = find_longest_affix(
                &remaining_chunk_chars,
                &self.center_combos_len_sorted,
                1,
                true,
            ) {
                let new_part: Chord = center_chord;

                match ch.merge(&new_part) {
                    Ok(()) => {
                        current_chord_str.push_str(&center_str);
                        remaining_chunk_chars = remaining_chunk_chars
                            .strip_prefix(center_str.as_str())
                            .unwrap()
                            .to_string();
                        debug!("REDUCE CENTER:\t{} ({}) ", center_str, new_part.to_string());
                    }
                    Err(_e) => {
                        debug!(
                            "CONFLICT CENTER:\t{} + {}, {} + {}",
                            chunk.strip_suffix(&remaining_chunk_chars).unwrap(),
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
                find_longest_affix(&remaining_chunk_chars, &self.rh_combos_len_sorted, 1, true)
            {
                let new_part: Chord = rh_chord;

                match ch.merge(&new_part) {
                    Ok(()) => {
                        current_chord_str.push_str(&rh_str);
                        remaining_chunk_chars = remaining_chunk_chars
                            .strip_prefix(rh_str.as_str())
                            .unwrap()
                            .to_string();
                        debug!("REDUCE RIGHT_HAND:\t{} ({}) ", rh_str, new_part.to_string());
                    }
                    Err(_e) => {
                        debug!(
                            "CONFLICT RIGHT-HAND:\t{} + {}, {} + {}",
                            chunk.strip_suffix(&remaining_chunk_chars).unwrap(),
                            rh_str,
                            ch.to_string(),
                            new_part.to_string(),
                        );
                        break;
                    }
                }
            }

            if ch == Chord::default() {
                error!("INFINITE-LOOP: {}, {} left", chunk, remaining_chunk_chars);
                return Err(format!("infinite loop on {}", chunk).into());
            }

            chunk_chords
                .items
                .push(ChordSeqItem::RootChord(current_chord_str, ch));
        }

        Ok(chunk_chords)
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

pub fn syllable_split(word: &str) -> Vec<String> {
    let re = Regex::new(r"[^aąeęioóuy]*(ia|ią|ie|ię|io|iu|ió|au|eu|a|ą|e|ę|i|o|ó|u|y)").unwrap();

    let mut matches: Vec<_> = re.find_iter(word).map(|m| m.as_str().to_owned()).collect();

    let split = re.split(word).collect::<Vec<_>>();

    // If the word ends with consonant(s), we add them to the final syllable
    match (matches.iter_mut().last(), split.iter().last()) {
        (Some(last_match), Some(last_split)) => {
            last_match.push_str(last_split);
        }
        // Edge case: word has no vowels, use as is
        (None, Some(last_split)) => {
            matches = vec![last_split.to_string()];
        }
        _other => {}
    }

    matches
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_syllable_split_simple() {
        env_logger::init();

        assert_eq!(
            syllable_split("przebiegłość"),
            vec!["prze", "bie", "głość"].to_owned()
        );
        assert_eq!(
            syllable_split("wyniosły"),
            vec!["wy", "nio", "sły"].to_owned()
        );
        assert_eq!(syllable_split("aorta"), vec!["a", "o", "rta"].to_owned());
        assert_eq!(syllable_split("towot"), vec!["to", "wot"].to_owned());
        assert_eq!(
            syllable_split("dodekahedron"),
            vec!["do", "de", "ka", "he", "dron"].to_owned()
        );
        assert_eq!(syllable_split("kościół"), vec!["ko", "ściół"].to_owned());
    }
}
