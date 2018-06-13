use parser::Term;

pub(super) struct Component {
    pub factor: Option<u32>,
    pub terms: Vec<Term>,
}

impl Default for Component {
    fn default() -> Self {
        Self {
            factor: None,
            terms: vec![],
        }
    }
}

impl Into<Vec<Term>> for Component {
    fn into(mut self) -> Vec<Term> {
        if let Some(factor) = self.factor {
            if let Some(first_term) = self.terms.first_mut() {
                first_term.factor = factor;
            }
        }

        self.terms
    }
}
