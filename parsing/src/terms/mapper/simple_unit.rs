use atom::Atom;
use prefix::Prefix;

pub(super) struct SimpleUnit {
    pub prefix: Option<Prefix>,
    pub atom: Option<Atom>,
}

impl SimpleUnit {
    pub(super) fn new() -> Self {
        SimpleUnit {
            prefix: None,
            atom: None,
        }
    }
}
