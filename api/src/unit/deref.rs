use crate::{parser::Term, unit::Unit};

//-----------------------------------------------------------------------------
// impl Deref
//-----------------------------------------------------------------------------
impl ::std::ops::Deref for Unit {
    type Target = [Term];

    fn deref(&self) -> &[Term] {
        &self.terms
    }
}
