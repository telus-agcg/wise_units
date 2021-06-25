use crate::parser::terms::mapper::{Component, Finishable};
use crate::parser::Term;

pub(super) struct AstTerm {
    pub(super) component: Option<Component>,
    pub(super) terms: Vec<Term>,
}

impl Finishable for AstTerm {
    fn finish(self) -> Vec<Term> {
        if let Some(component) = self.component {
            let mut component_terms = component.finish();
            component_terms.extend(self.terms.into_iter());
            component_terms
        } else {
            self.terms
        }
    }
}
