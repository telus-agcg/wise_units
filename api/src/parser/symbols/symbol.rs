use crate::parser::{Atom, Prefix};

// Technically there should never be a symbol without an atom.
#[derive(Debug)]
pub(crate) struct Symbol {
    pub(crate) prefix: Option<Prefix>,
    pub(crate) atom: Option<Atom>,
}
