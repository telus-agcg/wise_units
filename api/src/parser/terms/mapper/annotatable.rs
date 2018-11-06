use crate::parser::{Atom, Prefix};

pub(super) struct Annotatable {
    pub(super) prefix: Option<Prefix>,
    pub(super) atom: Option<Atom>,
    pub(super) exponent: Option<i32>,
}
