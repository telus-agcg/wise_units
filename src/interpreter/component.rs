use term::Term;

pub(super) struct Component {
    pub factor: Option<u32>,
    pub terms: Vec<Term>,
}

impl Component {
    pub(super) fn new() -> Self {
        Component {
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
