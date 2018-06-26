use parser::terms::mapper::Finishable;
use parser::Term;

pub(super) struct Component {
    pub factor: u32,
    pub terms: Vec<Term>,
}

impl Finishable for Component {
    fn finish(mut self) -> Vec<Term> {
        if self.factor != 1 {
            if let Some(first_term) = self.terms.first_mut() {
                first_term.factor = self.factor;
            }
        }

        self.terms
    }
}
