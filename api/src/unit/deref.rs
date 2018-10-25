use crate::parser::Term;
use crate::unit::Unit;

//-----------------------------------------------------------------------------
// impl Deref
//-----------------------------------------------------------------------------
impl ::std::ops::Deref for Unit {
    type Target = [Term];

    fn deref(&self) -> &[Term] {
        self.terms.as_slice()
    }
}
