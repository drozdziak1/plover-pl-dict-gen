use std::{cmp::{Eq, Ordering, PartialEq}, fmt::{Display, Debug}};

pub type ErrBox = Box<dyn std::error::Error>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LenSortableString<const ASCENDING: bool>(pub String);

// The Eq and PartialEq impls below invert String's lexicographic
// ordering, favoring length before contents.

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
        Display::fmt(&self.0, f)
    }
}

impl<const ASC: bool> Debug for LenSortableString<ASC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
