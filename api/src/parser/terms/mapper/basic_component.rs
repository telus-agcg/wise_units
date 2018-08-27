use parser::terms::mapper::Finishable;
use parser::{Atom, Prefix, Term};

pub(super) struct BasicComponent {
    pub atom: Option<Atom>,
    pub prefix: Option<Prefix>,
    pub factor: Option<u32>,
    pub exponent: Option<i32>,
    pub annotation: Option<String>,
    pub terms: Vec<Term>,
}

impl Default for BasicComponent {
    fn default() -> Self {
        Self {
            atom: None,
            prefix: None,
            factor: None,
            exponent: None,
            annotation: None,
            terms: vec![],
        }
    }
}

impl Finishable for BasicComponent {
    fn finish(self) -> Vec<Term> {
        let mut terms: Vec<Term> = Vec::with_capacity(self.terms.len() + 1);

        let self_term = Term {
            atom: self.atom,
            prefix: self.prefix,
            factor: self.factor,
            exponent: self.exponent,
            annotation: self.annotation,
        };

        terms.push(self_term);

        for term in self.terms {
            terms.push(term);
        }

        terms
    }
}
