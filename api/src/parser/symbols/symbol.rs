use parser::{Atom, Prefix};

// Technically there should never be a symbol without an atom.
#[derive(Debug)]
pub (crate) struct Symbol {
    pub prefix: Option<Prefix>,
    pub atom: Option<Atom>,
}

impl Default for Symbol {
    fn default() -> Self {
        Self {
            prefix: None,
            atom: None,
        }
    }
}
