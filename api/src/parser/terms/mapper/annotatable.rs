use parser::{Atom, Prefix};

pub(super) struct Annotatable {
    pub prefix: Option<Prefix>,
    pub atom: Option<Atom>,
    pub exponent: i32,
}
