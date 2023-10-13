use std::{fmt::Debug, str::FromStr};

use crate::ErrBox;

/// Middle keys and hyphen - They help us disambiguate left/right keys
const MID_CHARACTERS: &'static str = "JE~*IAU-";

const INVALID_CHORDS: &'static [&'static str] = &["XS", "FZ", "L*C", "R~R", "-TY", "-WO", "JIU"];

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Chord {
    x: bool,
    f: bool,
    z: bool,
    s_left: bool,
    k: bool,
    t_left: bool,
    p: bool,
    v: bool,
    l_left: bool,
    r_left: bool,
    j: bool,
    e: bool,
    tilde: bool,
    asterisk: bool,
    i: bool,
    a: bool,
    u: bool,
    c: bool,
    r_right: bool,
    l_right: bool,
    b: bool,
    s_right: bool,
    g: bool,
    t_right: bool,
    w: bool,
    o: bool,
    y: bool,
}

impl Chord {
    pub fn as_vec(&self) -> Vec<&bool> {
        // Will fail the build if the struct schema is changed without updating this assignment
        let Self {
            x,
            f,
            z,
            s_left,
            k,
            t_left,
            p,
            v,
            l_left,
            r_left,
            j,
            e,
            tilde,
            asterisk,
            i,
            a,
            u,
            c,
            r_right,
            l_right,
            b,
            s_right,
            g,
            t_right,
            w,
            o,
            y,
        } = self;

        vec![
            x, f, z, s_left, k, t_left, p, v, l_left, r_left, j, e, tilde, asterisk, i, a, u, c,
            r_right, l_right, b, s_right, g, t_right, w, o, y,
        ]
    }

    pub fn as_mut_vec(&mut self) -> Vec<&mut bool> {
        // Will fail the build if the struct schema is changed without updating this assignment
        let Self {
            x,
            f,
            z,
            s_left,
            k,
            t_left,
            p,
            v,
            l_left,
            r_left,
            j,
            e,
            tilde,
            asterisk,
            i,
            a,
            u,
            c,
            r_right,
            l_right,
            b,
            s_right,
            g,
            t_right,
            w,
            o,
            y,
        } = self;
        vec![
            x, f, z, s_left, k, t_left, p, v, l_left, r_left, j, e, tilde, asterisk, i, a, u, c,
            r_right, l_right, b, s_right, g, t_right, w, o, y,
        ]
    }

    /// Sum this chord with another, failing if an already pressed key is pressed in other
    pub fn merge(&mut self, other: &Chord) -> Result<(), ErrBox> {
        if self
            .as_vec()
            .into_iter()
            .zip(other.as_vec().into_iter())
            .any(|(self_key, other_key)| *self_key && *other_key)
        {
            return Err(format!(
                "Duplicate keys between {} and {}",
                self.to_string(),
                other.to_string()
            )
            .into());
        }

        let mut new = Self::default();

        for ((self_key, other_key), new_key) in self
            .as_vec()
            .into_iter()
            .zip(other.as_vec().into_iter())
            .zip(new.as_mut_vec().into_iter())
        {
            *new_key = *self_key ^ other_key;
        }

        // Check for invalid three- and four-key combinations
        new.validate()?;

        *self = new;
        Ok(())
    }

    pub fn contains(&self, other: &Self) -> bool {
        let zipped = self.as_vec().into_iter().zip(other.as_vec().into_iter());

        let mut result = true;
        for (self_key, other_key) in zipped {
            // other has key set but self does not, bail out
            if *other_key && !*self_key {
                result = false;
                break;
            }
        }

        result
    }

    pub fn full_steno_order() -> Self {
        let mut ret: Self = Default::default();

        for key in ret.as_mut_vec() {
            *key = true;
        }

        ret
    }

    fn validate(&self) -> Result<(), ErrBox> {
        for ch_str in INVALID_CHORDS {
            let ch = Chord::from_str(ch_str)?;

            if self.contains(&ch) {
                return Err(format!(
                    "Invalid chord: contains invalid combination {}",
                    ch.to_string()
                )
                .into());
            }
        }
        Ok(())
    }
}

impl FromStr for Chord {
    type Err = ErrBox;

    fn from_str(s: &str) -> Result<Self, ErrBox> {
        let mut ret = Chord::default();
        let mut left_hand = true; // Are we still adding left-hand chars?
        for ch in s.to_uppercase().chars() {
            match ch {
                'X' => {
                    ret.x = true;
                }
                'F' => {
                    ret.f = true;
                }
                'Z' => {
                    ret.z = true;
                }
                'S' => {
                    if left_hand {
                        ret.s_left = true;
                    } else {
                        ret.s_right = true;
                    }
                }
                'K' => {
                    ret.k = true;
                }
                'T' => {
                    if left_hand {
                        ret.t_left = true;
                    } else {
                        ret.t_right = true;
                    }
                }
                'P' => {
                    ret.p = true;
                }
                'V' => {
                    ret.v = true;
                }
                'L' => {
                    if left_hand {
                        ret.l_left = true;
                    } else {
                        ret.l_right = true;
                    }
                }
                'R' => {
                    if left_hand {
                        ret.r_left = true;
                    } else {
                        ret.r_right = true;
                    }
                }
                'J' => {
                    ret.j = true;
                }
                'E' => {
                    ret.e = true;
                }
                '~' => {
                    ret.tilde = true;
                }
                '*' => {
                    ret.asterisk = true;
                }
                'I' => {
                    ret.i = true;
                }
                'A' => {
                    ret.a = true;
                }
                'U' => {
                    ret.u = true;
                }
                'C' => {
                    ret.c = true;
                }
                // L and R already handled above
                'B' => {
                    ret.b = true;
                }
                // S handled above
                'G' => {
                    ret.g = true;
                }
                // T handled above
                'W' => {
                    ret.w = true;
                }
                'O' => {
                    ret.o = true;
                }
                'Y' => {
                    ret.y = true;
                }
                // hyphen should only switch the left hand flag, which
                // will happen automatically with the MID_CHARACTERS
                // check
                '-' => {}
                other => return Err(format!("Unknown character {:?}", other).into()),
            }

            if MID_CHARACTERS.contains(ch) {
                left_hand = false;
            }
        }
        Ok(ret)
    }
}

impl ToString for Chord {
    fn to_string(&self) -> String {
        #[deny(unused_variables)]
        let Self {
            x,
            f,
            z,
            s_left,
            k,
            t_left,
            p,
            v,
            l_left,
            r_left,
            j,
            e,
            tilde,
            asterisk,
            i,
            a,
            u,
            c,
            r_right,
            l_right,
            b,
            s_right,
            g,
            t_right,
            w,
            o,
            y,
        } = self;

        let mut ret = String::new();

        let mut needs_hyphen = true;

        if *x {
            ret.push('X');
        }
        if *f {
            ret.push('F');
        }
        if *z {
            ret.push('Z');
        }
        if *s_left {
            ret.push('S');
        }
        if *k {
            ret.push('K');
        }
        if *t_left {
            ret.push('T');
        }
        if *p {
            ret.push('P');
        }
        if *v {
            ret.push('V');
        }
        if *l_left {
            ret.push('L');
        }
        if *r_left {
            ret.push('R');
        }

        if *j {
            ret.push('J');
            needs_hyphen = false;
        }
        if *e {
            ret.push('E');
            needs_hyphen = false;
        }
        if *tilde {
            ret.push('~');
            needs_hyphen = false;
        }
        if *asterisk {
            ret.push('*');
            needs_hyphen = false;
        }
        if *i {
            ret.push('I');
            needs_hyphen = false;
        }
        if *a {
            ret.push('A');
            needs_hyphen = false;
        }
        if *u {
            ret.push('U');
            needs_hyphen = false;
        }

        // if needs_hyphen is still set, we need to disambiguate
        if needs_hyphen {
            ret.push('-');
        }

        if *c {
            ret.push('C');
        }
        if *r_right {
            ret.push('R');
        }
        if *l_right {
            ret.push('L');
        }
        if *b {
            ret.push('B');
        }
        if *s_right {
            ret.push('S');
        }
        if *g {
            ret.push('G');
        }
        if *t_right {
            ret.push('T');
        }
        if *w {
            ret.push('W');
        }
        if *o {
            ret.push('O');
        }
        if *y {
            ret.push('Y');
        }

        ret
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ChordSequence {
    pub items: Vec<ChordSeqItem>,
}

impl ChordSequence {
    pub fn new(items: Vec<ChordSeqItem>) -> Self {
        Self { items }
    }
    pub fn from_chord(s: String, ch: Chord) -> Self {
        Self {
            items: vec![ChordSeqItem::RootChord(s, ch)],
        }
    }

    pub fn collapse(&self) -> Vec<Chord> {
        self.items.iter().map(|i| i.collapse()).flatten().collect()
    }

    pub fn print_chords(&self) -> String {
        let chords = self.collapse();

        if chords.is_empty() {
            return String::from("<empty>");
        }

        return chords
            .iter()
            .map(|ch| ch.to_string())
            .collect::<Vec<_>>()
            .join(" + ");
    }

    pub fn is_oneshot(&self) -> bool {
	self.items.len() == 1
    }

    /// Re-assembles the word from sequence items
    pub fn get_word(&self) -> String {
        let mut ret = String::new();

        for item in self.items.iter() {
            let s = match item {
                ChordSeqItem::RootChord(s, _)
                | ChordSeqItem::Prefix(s, _)
                | ChordSeqItem::Suffix(s, _) => s,
            };

            ret.push_str(s);
        }
        ret
    }
}

impl From<Vec<ChordSeqItem>> for ChordSequence {
    fn from(items: Vec<ChordSeqItem>) -> Self {
        Self { items }
    }
}

impl From<ChordSeqItem> for ChordSequence {
    fn from(item: ChordSeqItem) -> Self {
        Self { items: vec![item] }
    }
}

impl ToString for ChordSequence {
    fn to_string(&self) -> String {
        self.items
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
            .join(" + ")
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum ChordSeqItem {
    RootChord(String, Chord),
    Prefix(String, Chord),
    Suffix(String, Chord),
}

impl ChordSeqItem {
    pub fn collapse(&self) -> Vec<Chord> {
        match self {
            Self::RootChord(_s, chord) => vec![chord.clone()],
            Self::Prefix(_s, chord) => vec![chord.clone()],
            Self::Suffix(_s, chord) => vec![chord.clone()],
        }
    }
}

impl ToString for ChordSeqItem {
    fn to_string(&self) -> String {
        match self {
            Self::RootChord(s, ch) => format!("RC:\"{}\":{}", s, ch.to_string()),
            Self::Prefix(s, ch) => format!("P:\"{}-\":{}", s, ch.to_string()),
            Self::Suffix(s, ch) => format!("S:\"-{}\":{}", s, ch.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chord_conflict_is_detected() {
        let mut a = Chord::default();
        a.x = true;

        let mut b = Chord::default();
        b.x = true;

        assert!(a.merge(&b).is_err())
    }

    #[test]
    fn test_chord_without_conflicts_merges() -> Result<(), ErrBox> {
        let mut a = Chord::from_str("XZKPLJE")?;

        let b = Chord::from_str("~ICLSTO")?;

        a.merge(&b)?;

        assert_eq!(&a.to_string(), "XZKPLJE~ICLSTO");

        Ok(())
    }

    #[test]
    fn test_chord_parser_recognizes_whole_set() -> Result<(), ErrBox> {
        let full_str = "XFZSKTPVLRJE~*IAUCRLBSGTWOY";
        let parsed: Chord = full_str.parse()?;

        let full_steno = Chord::full_steno_order();

        // Cross validate struct and generated string
        assert_eq!(parsed, full_steno);
        assert_eq!(&full_steno.to_string(), full_str);

        Ok(())
    }

    #[test]
    fn test_contains() {
        let full_steno = Chord::full_steno_order();
        let empty = Chord::default();

        // Full steno order contains itself
        assert!(full_steno.contains(&full_steno));

        // Full steno contains empty chord
        assert!(full_steno.contains(&empty));

        // Empty chord contains itself
        assert!(empty.contains(&empty));

        // Empty chord does not contain full steno
        assert!(!empty.contains(&full_steno));
    }

    #[test]
    fn test_validate() -> Result<(), ErrBox> {
        let full_steno = Chord::full_steno_order();
        let empty = Chord::default();

        // Full is never valid
        assert!(full_steno.validate().is_err());

        // Empty is always valid
        empty.validate()?;

        // Each of the invalid combos is always invalid
        for invalid_str in INVALID_CHORDS {
            let invalid = Chord::from_str(invalid_str)?;
            assert!(invalid.validate().is_err());
        }

        Ok(())
    }
}
