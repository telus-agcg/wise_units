use crate::{v2::traits::unit_conversion, Unit};

impl unit_conversion::ToReduced for Unit {
    fn to_reduced(&self) -> Self {
        // Just delegate to the old trait impl for now.
        crate::reduce::ToReduced::to_reduced(self)
    }
}
