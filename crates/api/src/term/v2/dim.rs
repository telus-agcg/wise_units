use std::borrow::Cow;

use crate::{
    v2::dim::{Dimension, IsCommensurableWith},
    Composable, Composition, IsCompatibleWith, Term,
};

impl Dimension<Composition> for Term {
    fn dimension(&self) -> Composition {
        Composable::composition(self)
    }
}

impl IsCommensurableWith<Composition> for Term {
    /// See `impl IsCompatibleWith for Term` as to why this has a special implementation here.
    ///
    fn is_commensurable_with(&self, rhs: &Self) -> bool {
        IsCompatibleWith::is_compatible_with(self, rhs)
    }
}

impl<'a> Dimension<Composition> for Cow<'a, [Term]> {
    fn dimension(&self) -> Composition {
        Composable::composition(self)
    }
}

impl<'a> IsCommensurableWith<Composition> for Cow<'a, [Term]> {
    /// See `impl<'a> IsCompatibleWith for Cow<'a, [Term]>` as to why this has a special implementation here.
    ///
    fn is_commensurable_with(&self, rhs: &Self) -> bool {
        IsCompatibleWith::is_compatible_with(self, rhs)
    }
}
