use crate::parser::Term;

pub(super) struct MainTerm {
    pub(super) terms: Vec<Term>,
}

impl From<MainTerm> for Vec<Term> {
    fn from(main_term: MainTerm) -> Self {
        main_term.terms
    }
}
