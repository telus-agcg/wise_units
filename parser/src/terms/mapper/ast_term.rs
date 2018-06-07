use term::Term;
use terms::mapper::component::Component;

pub(super) struct AstTerm {
    pub component: Option<Component>,
    pub terms: Vec<Term>,
}

impl Default for AstTerm {
    fn default() -> Self {
        Self {
            component: None,
            terms: vec![],
        }
    }
}

impl Into<Vec<Term>> for AstTerm {
    fn into(mut self) -> Vec<Term> {
        let mut terms: Vec<Term> = Vec::with_capacity(self.terms.len() + 1);

        if let Some(component) = self.component {
            terms.append(&mut component.into());
        }

        terms.append(&mut self.terms);

        terms
    }
}
