use term::Term;
use unit::Unit;

pub(super) struct MainTerm {
    pub terms: Vec<Term>,
}

impl MainTerm {
    pub fn new() -> Self {
        MainTerm { terms: vec![] }
    }
}

impl Into<Unit> for MainTerm {
    fn into(self) -> Unit {
        Unit { terms: self.terms }
    }
}
