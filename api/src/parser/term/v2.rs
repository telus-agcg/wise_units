mod convert;
mod ops;
mod ucum;

use crate::{v2::type_traits::Term as TTerm, Atom, Prefix, Term};

impl<'a> TTerm<'a, f64> for Term {
    type Prefix = Prefix;
    type Atom = Atom;
    type Annotation = &'a str;

    fn factor(&self) -> Option<u32> {
        self.factor
    }

    fn prefix_symbol(&self) -> Option<Self::Prefix> {
        self.prefix
    }

    fn atom_symbol(&self) -> Option<Self::Atom> {
        self.atom
    }

    fn exponent(&self) -> Option<i32> {
        self.exponent
    }

    fn annotation(&'a self) -> Option<Self::Annotation> {
        self.annotation.as_deref()
    }
}
