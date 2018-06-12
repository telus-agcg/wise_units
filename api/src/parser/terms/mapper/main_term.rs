use parser::Term;

pub(super) struct MainTerm {
    pub terms: Vec<Term>,
}

impl Default for MainTerm {
    fn default() -> Self {
        Self { terms: vec![] }
    }
}

impl Into<Vec<Term>> for MainTerm {
    fn into(self) -> Vec<Term> {
        self.terms
    }
}
