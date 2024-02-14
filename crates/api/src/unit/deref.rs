use crate::{parser::Term, unit::Unit};

//-----------------------------------------------------------------------------
// impl Deref
//-----------------------------------------------------------------------------
// TODO: Get rid of this in 1.0.
impl ::std::ops::Deref for Unit {
    type Target = [Term];

    fn deref(&self) -> &[Term] {
        &self.terms
    }
}
