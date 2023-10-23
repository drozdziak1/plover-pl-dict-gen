use lazy_static::lazy_static;
use log::{debug, error, info, trace, warn};
use regex::Regex;

use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
};

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
    pub word_root_conflict_dict: BTreeMap<ChordSequence, BTreeSet<String>>,
    pub chunk_dict: BTreeMap<LenSortableString<false>, ChordSequence>,
    pub chunk_conflict_dict: BTreeMap<ChordSequence, BTreeSet<String>>,
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
            word_root_conflict_dict: BTreeMap::new(),
            chunk_dict,
            chunk_conflict_dict: BTreeMap::new(),
        })
    }

    /// NOTE: Only root recipe is added to the dictionary, but the
    /// complete chord set is returned.
    pub fn add_word_root(&mut self, word: &str) -> Result<ChordSequence, ErrBox> {
        let (word_chords, new_chunk_chords) = self.gen_word_chords(word)?;
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

        // Record word root conflicts
        if let Some(existing) = self.word_root_conflict_dict.get_mut(&root_chords) {
            trace!(
                "WORD-ROOT-CONFLICT Stroke(s) {} already exist for: {:?}",
                root_chords.print_chords(),
                existing
            );
            existing.insert(root_chords.get_word());
        } else {
            let mut new_set = BTreeSet::new();
            new_set.insert(root_chords.get_word());
            self.word_root_conflict_dict
                .insert(root_chords.clone(), new_set);
        }

        for chunk in new_chunk_chords {
            self.chunk_dict
                .insert(chunk.get_word().into(), chunk.clone());

            if let Some(existing) = self.chunk_conflict_dict.get_mut(&chunk) {
                trace!(
                    "CHUNK-CONFLICT Stroke(s) {} already exist for: {:?}",
                    chunk.print_chords(),
                    existing
                );
                existing.insert(chunk.get_word());
            } else {
                let mut new_set = BTreeSet::new();
                new_set.insert(chunk.get_word());
                self.chunk_conflict_dict.insert(chunk.clone(), new_set);
            }
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

    #[deny(unused_variables)]
    pub fn save_to_file(&self, f: File) -> Result<(), ErrBox> {
        let chunk_iter = self
            .chunk_dict
            .iter()
            .map(|(s, ch_seq)| (ch_seq.print_chords(), format!("{{&{}}}", s)));

        let prefix_iter = self
            .prefixes_len_sorted
            .iter()
            .map(|(s, ch)| (ch.to_string(), format!("{}{}", s, "{^}")));

        let suffix_iter = self
            .suffixes_len_sorted
            .iter()
            .map(|(s, ch)| (ch.to_string(), format!("{}{}", "{^}", s)));

        let special_char_iter = dict_lookup::SPECIAL_CHARS
            .into_iter()
            .map(|(s, ch)| (ch.to_string(), s.to_string()));

        let commands_iter = dict_lookup::COMMANDS
            .into_iter()
            .map(|(s, ch)| (ch.to_string(), s.to_string()));

        let chained = chunk_iter.chain(prefix_iter).chain(suffix_iter).chain(special_char_iter).chain(commands_iter);

        let final_dict: BTreeMap<String, String> = chained.collect();

        serde_json::to_writer_pretty(f, &final_dict)?;

        Ok(())
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

const CONSONANT_SUBSTR: &'static str =
    "(ch|cz|dz|dź|dż|sz|rz|b|c|ć|d|f|g|h|j|k|l|ł|m|n|ń|p|q|r|s|ś|t|v|w|x|z|ź|ż)";
const VOWEL_SUBSTR: &'static str = "(ia|ią|ie|ię|io|iu|ió|au|eu|a|ą|e|ę|i|o|ó|u|y)";

lazy_static! {
    static ref RE_ROUGH_SYL: Regex = {
        let rough_syl_pattern: String = format!("{}*{}", CONSONANT_SUBSTR, VOWEL_SUBSTR);
        Regex::new(&rough_syl_pattern).unwrap()
    };
    static ref RE_CONSONANT_GROUP: Regex = {
        let consonant_group_pattern: String =
            format!("^({}{}+).*", CONSONANT_SUBSTR, CONSONANT_SUBSTR);

        Regex::new(&consonant_group_pattern).unwrap()
    };
    static ref RE_SINGLE_CONSONANT: Regex = {
        let single_consonant_pattern: String = format!("^{}$", CONSONANT_SUBSTR);

        Regex::new(&single_consonant_pattern).unwrap()
    };
}

pub fn syllable_split(word: &str) -> Vec<String> {
    let mut rough_syllables: Vec<_> = RE_ROUGH_SYL
        .find_iter(word)
        .map(|m| m.as_str().to_owned())
        .collect();

    let split = RE_ROUGH_SYL.split(word).collect::<Vec<_>>();

    // If the word ends with consonant(s), we add them to the final syllable
    match (rough_syllables.iter_mut().last(), split.iter().last()) {
        (Some(last_match), Some(last_split)) => {
            last_match.push_str(last_split);
        }
        // Edge case: word has no vowels, use as is
        (None, Some(last_split)) => {
            rough_syllables = vec![last_split.to_string()];
        }
        _other => {}
    }

    let mut rebalanced: Vec<_> = rough_syllables.get(0).cloned().into_iter().collect();

    // Rebalance consonants
    for idx in 1..rough_syllables.len() {
        trace!("Trying {}", rough_syllables[idx]);
        let mut push_todo = rough_syllables[idx].clone();

        if let Some(caps) = RE_CONSONANT_GROUP.captures(&rough_syllables[idx]) {
            if let Some(group) = caps.get(1) {
                let grp = group.as_str();
                trace!("Checking {} for single consonant digraph", grp);

                if (*RE_SINGLE_CONSONANT).is_match(grp) {
                    trace!("{} is a single digraph", grp);
                } else {
                    trace!("{} is not a single digraph", grp);
                    if let Some(consonant) = caps.get(2) {
                        let rebalance_todo = consonant.as_str();
                        push_todo = rough_syllables[idx]
                            .strip_prefix(rebalance_todo)
                            .unwrap()
                            .to_owned();

                        trace!(
                            "{} <- {} + {}",
                            rebalanced[idx - 1],
                            rebalance_todo,
                            push_todo
                        );

                        rebalanced[idx - 1].push_str(rebalance_todo);
                    }
                }
            }
        }
        rebalanced.push(push_todo);
    }

    rebalanced
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_syllable_split_simple() {
        env_logger::init();

        assert_eq!(
            syllable_split("przebiegłość"),
            vec!["prze", "bieg", "łość"].to_owned()
        );
        assert_eq!(
            syllable_split("wyniosły"),
            vec!["wy", "nios", "ły"].to_owned()
        );
        assert_eq!(syllable_split("aorta"), vec!["a", "or", "ta"].to_owned());
        assert_eq!(syllable_split("towot"), vec!["to", "wot"].to_owned());
        assert_eq!(
            syllable_split("dodekahedron"),
            vec!["do", "de", "ka", "hed", "ron"].to_owned()
        );
        assert_eq!(syllable_split("kościół"), vec!["koś", "ciół"].to_owned());
        assert_eq!(syllable_split("zawżdy"), vec!["zaw", "żdy"].to_owned());
        assert_eq!(
            syllable_split("spółgłoska"),
            vec!["spół", "głos", "ka"].to_owned()
        );

        assert_eq!(syllable_split("kuchta"), vec!["kuch", "ta"].to_owned());
        assert_eq!(
            syllable_split("marzanna"),
            vec!["ma", "rzan", "na"].to_owned()
        );

        assert_eq!(syllable_split("marznąć"), vec!["marz", "nąć"].to_owned());
    }
}
