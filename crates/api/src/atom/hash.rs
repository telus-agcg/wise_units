use std::hash::{Hash, Hasher};

use crate::Atom;

impl Hash for Atom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        stringify!(self).hash(state);
    }
}
