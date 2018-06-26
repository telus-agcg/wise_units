use parser::terms::mapper::{Component, Finishable};
use parser::Term;

pub(super) struct AstTerm {
    pub component: Option<Component>,
    pub terms: Vec<Term>,
}

impl Finishable for AstTerm {
    fn finish(self) -> Vec<Term> {
        if let Some(component) = self.component {
            let component_terms: Vec<Term> = component.finish();

            let mut total_terms: Vec<Term> =
                Vec::with_capacity(self.terms.len() + component_terms.len());

            for term in component_terms {
                total_terms.push(term);
            }

            for term in self.terms {
                total_terms.push(term);
            }

            total_terms
        } else {
            self.terms
        }
    }
}
