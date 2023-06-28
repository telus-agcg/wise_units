use crate::{Atom, IsCompatibleWith, UcumUnit};

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        if !self.is_compatible_with(other) {
            return false;
        }

        self.scalar() == other.scalar()
    }
}
