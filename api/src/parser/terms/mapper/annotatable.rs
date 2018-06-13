use parser::{Atom, Prefix};

pub(super) struct Annotatable {
    pub prefix: Option<Prefix>,
    pub atom: Option<Atom>,
    pub exponent: i32,
}

impl Default for Annotatable {
    fn default() -> Self {
        Self {
            prefix: None,
            atom: None,
            exponent: 1,
        }
    }
}
