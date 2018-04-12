use atom::Atom;
use prefix::Prefix;

pub(super) struct Annotatable {
    pub prefix: Option<Prefix>,
    pub atom: Option<Atom>,
    pub exponent: i32,
}

impl Annotatable {
    pub(super) fn new() -> Self {
        Annotatable {
            prefix: None,
            atom: None,
            exponent: 1,
        }
    }
}
