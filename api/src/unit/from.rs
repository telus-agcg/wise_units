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
