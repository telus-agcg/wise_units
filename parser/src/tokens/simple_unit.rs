use super::{AtomSymbol, PrefixSymbol};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SimpleUnit {
    pub(crate) factor: Option<u32>,
    pub(crate) prefix_symbol: Option<&'static PrefixSymbol>,
    pub(crate) atom_symbol: &'static AtomSymbol,
}

impl SimpleUnit {
    #[must_use]
    pub const fn new(
        factor: Option<u32>,
        prefix_symbol: Option<&'static PrefixSymbol>,
        atom_symbol: &'static AtomSymbol,
    ) -> Self {
        Self {
            factor,
            prefix_symbol,
            atom_symbol,
        }
    }

    #[must_use]
    pub const fn factor(&self) -> Option<u32> {
        self.factor
    }

    #[must_use]
    pub const fn prefix_symbol(&self) -> Option<&PrefixSymbol> {
        self.prefix_symbol
    }

    #[must_use]
    pub const fn atom_symbol(&self) -> &AtomSymbol {
        self.atom_symbol
    }
}
