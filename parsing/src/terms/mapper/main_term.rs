use term::Term;
// use unit::Unit;

pub(super) struct MainTerm {
    pub terms: Vec<Term>,
}

impl MainTerm {
    pub fn new() -> Self {
        MainTerm { terms: vec![] }
    }
}

impl Into<Vec<Term>> for MainTerm {
    fn into(self) -> Vec<Term> {
       self.terms
    }
}
