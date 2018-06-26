use parser::Term;

pub(super) struct MainTerm {
    pub terms: Vec<Term>,
}

impl Into<Vec<Term>> for MainTerm {
    fn into(self) -> Vec<Term> {
        self.terms
    }
}
