use std::{
    convert::Infallible,
    f64::{INFINITY, NAN},
    ops::{Add, Div, Mul, Sub},
};

use crate::{
    v2::behavior_traits::{convert::ToScalar, ops, unit_conversion::TryConvertTo},
    Measurement, Unit,
};

impl<'a> ops::Comparable<'a, f64> for Measurement {
    /// Overriding default to use `ulps_eq` for comparing `f64`
    ///
    fn is_commensurable_with(&'a self, rhs: &'a Self) -> bool {
        if !ops::Comparable::is_compatible_with(self, rhs) {
            return false;
        }

        approx::ulps_eq!(self.to_scalar(), rhs.to_scalar())
    }
}

impl<'a> ops::Comparable<'a, f64, Unit> for Measurement {
    /// Overriding default to use `ulps_eq` for comparing `f64`
    ///
    fn is_commensurable_with(&'a self, rhs: &'a Unit) -> bool {
        if !ops::Comparable::is_compatible_with(self, rhs) {
            return false;
        }

        approx::ulps_eq!(self.to_scalar(), rhs.to_scalar())
    }
}

impl<'a> ops::TryAddRef<'a> for Measurement {
    type Error = crate::Error;

    fn try_add_ref(&'a self, rhs: &'a Self) -> Result<Self, Self::Error> {
        // Just delegate to the old trait impl for now.
        Add::add(self, rhs)
    }
}

impl<'a> ops::TrySubRef<'a> for Measurement {
    type Error = crate::Error;

    fn try_sub_ref(&'a self, rhs: &'a Self) -> Result<Self, Self::Error> {
        // Just delegate to the old trait impl for now.
        Sub::sub(self, rhs)
    }
}

impl ops::MulRef for Measurement {
    fn mul_ref(&self, rhs: &Self) -> Self {
        // Just delegate to the old trait impl for now.
        Mul::mul(self, rhs)
    }
}

impl ops::TryMulRef<'_> for Measurement {
    type Error = Infallible;

    fn try_mul_ref(&self, rhs: &Self) -> Result<Self, Self::Error> {
        Ok(ops::MulRef::mul_ref(self, rhs))
    }
}

impl ops::CheckedMulRef for Measurement {
    fn checked_mul_ref(&self, rhs: &Self) -> Option<Self> {
        fn do_float_things(lhs: f64, rhs: f64) -> Option<f64> {
            if rhs == 0.0 || rhs == NAN || rhs == INFINITY {
                return None;
            }

            let value = lhs * rhs;

            if value == NAN || value == INFINITY {
                None
            } else {
                Some(value)
            }
        }

        match rhs.try_convert_to(&self.unit) {
            Ok(converted) => Some(Self {
                value: do_float_things(self.value, converted.value)?,
                unit: Mul::mul(&self.unit, &converted.unit),
            }),
            Err(_) => Some(Self {
                value: do_float_things(self.value, rhs.value)?,
                unit: Mul::mul(&self.unit, &rhs.unit),
            }),
        }
    }
}

impl ops::DivRef for Measurement {
    fn div_ref(&self, rhs: &Self) -> Self {
        // Just delegate to the old trait impl for now.
        Div::div(self, rhs)
    }
}

impl ops::TryDivRef<'_> for Measurement {
    type Error = Infallible;

    fn try_div_ref(&self, rhs: &Self) -> Result<Self, Self::Error> {
        Ok(ops::DivRef::div_ref(self, rhs))
    }
}

impl ops::CheckedDivRef for Measurement {
    fn checked_div_ref(&self, rhs: &Self) -> Option<Self> {
        fn do_float_things(lhs: f64, rhs: f64) -> Option<f64> {
            if rhs == 0.0 || rhs == NAN || rhs == INFINITY {
                return None;
            }

            let value = lhs / rhs;

            if value == NAN || value == INFINITY {
                None
            } else {
                Some(value)
            }
        }

        match rhs.try_convert_to(&self.unit) {
            Ok(converted) => Some(Self {
                value: do_float_things(self.value, converted.value)?,
                unit: Div::div(&self.unit, &converted.unit),
            }),
            Err(_) => Some(Self {
                value: do_float_things(self.value, rhs.value)?,
                unit: Div::div(&self.unit, &rhs.unit),
            }),
        }
    }
}
