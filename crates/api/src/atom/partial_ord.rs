use crate::{IsCompatibleWith, UcumUnit};

use super::Atom;

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is_compatible_with(other) {
            self.scalar().partial_cmp(&other.scalar())
        } else {
            None
        }
    }
}
