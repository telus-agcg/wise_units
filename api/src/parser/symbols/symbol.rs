use parser::{Atom, Prefix};

// Technically there should never be a symbol without an atom.
#[derive(Debug)]
pub(crate) struct Symbol {
    pub prefix: Option<Prefix>,
    pub atom: Option<Atom>,
}
