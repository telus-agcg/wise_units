use crate::parser::terms::mapper::{Component, Finishable};
use crate::parser::Term;

pub(super) struct AstTerm {
    pub(super) component: Option<Component>,
    pub(super) terms: Vec<Term>,
}

impl Finishable for AstTerm {
    fn finish(self) -> Vec<Term> {
        if let Some(component) = self.component {
            let component_terms: Vec<Term> = component.finish();

            let mut total_terms: Vec<Term> =
                Vec::with_capacity(self.terms.len() + component_terms.len());

            total_terms.extend_from_slice(&component_terms);
            total_terms.extend_from_slice(&self.terms);

            total_terms
        } else {
            self.terms
        }
    }
}
