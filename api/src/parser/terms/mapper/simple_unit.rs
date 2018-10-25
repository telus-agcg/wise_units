use crate::parser::{Atom, Prefix};

pub(super) struct SimpleUnit {
    pub(super) prefix: Option<Prefix>,
    pub(super) atom: Option<Atom>,
}

impl Default for SimpleUnit {
    fn default() -> Self {
        Self {
            prefix: None,
            atom: None,
        }
    }
}
