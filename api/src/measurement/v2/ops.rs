use crate::{v2::traits::ops, Measurement};

// impl ops::IsCompatibleWith<Self> for Measurement {
//     fn is_compatible_with(&self, rhs: &Self) -> bool {
//         crate::IsCompatibleWith::is_compatible_with(self, rhs)
//     }
// }

// impl ops::IsCompatibleWith<Unit> for Measurement {
//     fn is_compatible_with(&self, rhs: &Unit) -> bool {
//         crate::IsCompatibleWith::is_compatible_with(self, rhs)
//     }
// }

impl ops::FieldEq<Self> for Measurement {
    fn field_eq(&self, rhs: &Self) -> bool {
        // Just delegate to the old trait impl for now.
        crate::FieldEq::field_eq(self, rhs)
    }
}

impl<'a> ops::TryAddRef<'a> for Measurement {
    type Error = crate::Error;

    fn try_add_ref(&'a self, rhs: &'a Self) -> Result<Self, Self::Error> {
        // Just delegate to the old trait impl for now.
        std::ops::Add::add(self, rhs)
    }
}

impl<'a> ops::TrySubRef<'a> for Measurement {
    type Error = crate::Error;

    fn try_sub_ref(&'a self, rhs: &'a Self) -> Result<Self, Self::Error> {
        // Just delegate to the old trait impl for now.
        std::ops::Sub::sub(self, rhs)
    }
}

impl ops::MulRef for Measurement {
    fn mul_ref(&self, rhs: &Self) -> Self {
        // Just delegate to the old trait impl for now.
        std::ops::Mul::mul(self, rhs)
    }
}

impl ops::DivRef for Measurement {
    fn div_ref(&self, rhs: &Self) -> Self {
        // Just delegate to the old trait impl for now.
        std::ops::Div::div(self, rhs)
    }
}
