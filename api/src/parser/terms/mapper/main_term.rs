use parser::Term;

pub(super) struct MainTerm {
    pub(super) terms: Vec<Term>,
}

impl Into<Vec<Term>> for MainTerm {
    fn into(self) -> Vec<Term> {
        self.terms
    }
}
