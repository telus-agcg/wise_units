use std::{cmp::Ordering, ops::Deref};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct SymbolKey(pub(crate) &'static str);

impl PartialOrd for SymbolKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Since this implementation determines how `SymbolKey`es sort in a `BTreeSet`, it effectively
/// determines how strings will match when parsing.
///
impl Ord for SymbolKey {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.len().cmp(&other.0.len()) {
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => self.0.cmp(other.0),
            Ordering::Less => Ordering::Greater,
        }
    }
}

impl Deref for SymbolKey {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
