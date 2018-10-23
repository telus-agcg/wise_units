use parser::terms::mapper::Finishable;
use parser::Term;

pub(super) struct Component {
    pub(super) factor: Option<u32>,
    pub(super) terms: Vec<Term>,
}

impl Finishable for Component {
    fn finish(mut self) -> Vec<Term> {
        if let Some(factor) = self.factor {
            if factor != 1 {
                if let Some(first_term) = self.terms.first_mut() {
                    first_term.factor = Some(factor);
                }
            }
        }

        self.terms
    }
}
