use parser::{Atom, Prefix, Term};

pub(super) struct BasicComponent {
    pub atom: Option<Atom>,
    pub prefix: Option<Prefix>,
    pub factor: u32,
    pub exponent: i32,
    pub annotation: Option<String>,
    pub terms: Vec<Term>,
}

impl Default for BasicComponent {
    fn default() -> Self {
        Self {
            atom: None,
            prefix: None,
            factor: 1,
            exponent: 1,
            annotation: None,
            terms: vec![],
        }
    }
}

impl Into<Vec<Term>> for BasicComponent {
    fn into(mut self) -> Vec<Term> {
        let mut terms: Vec<Term> = Vec::with_capacity(self.terms.len() + 1);

        let self_term = Term {
            atom: self.atom,
            prefix: self.prefix,
            factor: self.factor,
            exponent: self.exponent,
            annotation: self.annotation,
        };

        terms.push(self_term);
        terms.append(&mut self.terms);

        terms
    }
}
