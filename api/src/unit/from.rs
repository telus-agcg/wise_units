use crate::parser::Term;
use crate::unit::Unit;

//-----------------------------------------------------------------------------
// impl From
//-----------------------------------------------------------------------------
impl ::std::convert::From<Vec<Term>> for Unit {
    fn from(terms: Vec<Term>) -> Self {
        Self { terms }
    }
}

impl ::std::convert::From<&[Term]> for Unit {
    fn from(terms: &[Term]) -> Self {
        Self { terms: terms.to_vec() }
    }
}
