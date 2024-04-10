use crate::Term;

pub(super) trait Finishable {
    fn finish(self) -> Vec<Term>;
}
