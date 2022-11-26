use std::{fmt::Debug, str::FromStr};

use crate::ErrBox;

/// Middle keys and hyphen - They help us disambiguate left/right keys
static MID_CHARACTERS: &'static str = "JE~*IAU-";

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
                "Conflicting entries between {} and {}",
                self.to_string(),
                other.to_string()
            )
            .into());
        }

        for (self_key, other_key) in self
            .as_mut_vec()
            .into_iter()
            .zip(other.as_vec().into_iter())
        {
            *self_key = *self_key ^ other_key;
        }
        Ok(())
    }

    pub fn full_steno_order() -> Self {
        let mut ret: Self = Default::default();

        for key in ret.as_mut_vec() {
            *key = true;
        }

        ret
    }
}

impl FromStr for Chord {
    type Err = ErrBox;

    fn from_str(s: &str) -> Result<Self, ErrBox> {
        let mut ret = Chord::default();
        let mut left_hand = true; // Are we still adding left-hand chars?
        for ch in s.to_uppercase().chars() {
            match ch {
                'X' => ret.merge(&Chord {
                    x: true,
                    ..Default::default()
                })?,
                'F' => ret.merge(&Chord {
                    f: true,
                    ..Default::default()
                })?,
                'Z' => ret.merge(&Chord {
                    z: true,
                    ..Default::default()
                })?,
                'S' => {
                    if left_hand {
                        ret.merge(&Chord {
                            s_left: true,
                            ..Default::default()
                        })?
                    } else {
                        ret.merge(&Chord {
                            s_right: true,
                            ..Default::default()
                        })?
                    }
                }
                'K' => ret.merge(&Chord {
                    k: true,
                    ..Default::default()
                })?,
                'T' => {
                    if left_hand {
                        ret.merge(&Chord {
                            t_left: true,
                            ..Default::default()
                        })?
                    } else {
                        ret.merge(&Chord {
                            t_right: true,
                            ..Default::default()
                        })?
                    }
                }
                'P' => ret.merge(&Chord {
                    p: true,
                    ..Default::default()
                })?,
                'V' => ret.merge(&Chord {
                    v: true,
                    ..Default::default()
                })?,
                'L' => {
                    if left_hand {
                        ret.merge(&Chord {
                            l_left: true,
                            ..Default::default()
                        })?
                    } else {
                        ret.merge(&Chord {
                            l_right: true,
                            ..Default::default()
                        })?
                    }
                }
                'R' => {
                    if left_hand {
                        ret.merge(&Chord {
                            r_left: true,
                            ..Default::default()
                        })?
                    } else {
                        ret.merge(&Chord {
                            r_right: true,
                            ..Default::default()
                        })?
                    }
                }
                'J' => ret.merge(&Chord {
                    j: true,
                    ..Default::default()
                })?,
                'E' => ret.merge(&Chord {
                    e: true,
                    ..Default::default()
                })?,
                '~' => ret.merge(&Chord {
                    tilde: true,
                    ..Default::default()
                })?,
                '*' => ret.merge(&Chord {
                    asterisk: true,
                    ..Default::default()
                })?,
                'I' => ret.merge(&Chord {
                    i: true,
                    ..Default::default()
                })?,
                'A' => ret.merge(&Chord {
                    a: true,
                    ..Default::default()
                })?,
                'U' => ret.merge(&Chord {
                    u: true,
                    ..Default::default()
                })?,
                'C' => ret.merge(&Chord {
                    c: true,
                    ..Default::default()
                })?,
                // L and R already handled above
                'B' => ret.merge(&Chord {
                    b: true,
                    ..Default::default()
                })?,
                // S handled above
                'G' => ret.merge(&Chord {
                    g: true,
                    ..Default::default()
                })?,
                // T handled above
                'W' => ret.merge(&Chord {
                    w: true,
                    ..Default::default()
                })?,
                'O' => ret.merge(&Chord {
                    o: true,
                    ..Default::default()
                })?,
                'Y' => ret.merge(&Chord {
                    y: true,
                    ..Default::default()
                })?,
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
            items: vec![ChordSeqItem::Plain(s, ch)],
        }
    }

    pub fn collapse(&self) -> Vec<Chord> {
        self.items.iter().map(|i| i.collapse()).flatten().collect()
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
    Plain(String, Chord),
    Nested(String, ChordSequence),
}

impl ChordSeqItem {
    pub fn collapse(&self) -> Vec<Chord> {
        match self {
            Self::Plain(ref s, chord) => vec![chord.clone()],
            Self::Nested(ref s, seq) => seq.collapse(),
        }
    }
}

impl ToString for ChordSeqItem {
    fn to_string(&self) -> String {
        match self {
            Self::Plain(s, ch) => format!("{:?}:{}", s, ch.to_string()),
            Self::Nested(s, chords) => format!("{:?}:({})", s, chords.to_string(),),
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
    fn test_chord_without_conflicts_merges() {
        let mut a = Chord::default();

        let b = Chord::full_steno_order();

        assert!(a.merge(&b).is_ok());

        assert_eq!(a, b);
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
}
