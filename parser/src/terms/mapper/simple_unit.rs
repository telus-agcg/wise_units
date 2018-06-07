use atom::Atom;
use prefix::Prefix;

pub(super) struct SimpleUnit {
    pub prefix: Option<Prefix>,
    pub atom: Option<Atom>,
}

impl Default for SimpleUnit {
    fn default() -> Self {
        Self {
            prefix: None,
            atom: None,
        }
    }
}
