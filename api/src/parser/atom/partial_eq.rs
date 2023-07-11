use crate::{Atom, IsCompatibleWith, UcumUnit};

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        if !self.is_compatible_with(other) {
            return false;
        }

        // TODO: should this use `ulps_eq!()` like the `Term` impl?
        self.scalar() == other.scalar()
    }
}
