use std::{
    f64::{INFINITY, NAN},
    ops::{Add, Div, Mul, Sub},
};

use crate::v2::{
    behavior_traits::{convert::TryConvertTo, ops},
    type_traits,
};

use super::Measurement;

impl<'a, V, U> ops::TryAddRef<'a> for Measurement<V, U>
where
    U: type_traits::Unit<'a, V> + Clone + 'a,
    &'a U: ToString,
    V: PartialOrd + Clone + Mul<V, Output = V> + 'a,
    V: Add<V, Output = V>,
{
    type Error = <Self as TryConvertTo<'a, U>>::Error;

    fn try_add_ref(&'a self, rhs: &'a Self) -> Result<Self, Self::Error> {
        let rhs_converted = rhs.try_convert_to(&self.unit)?;
        let new_value = Add::add(self.value.clone(), rhs_converted.value);

        Ok(Self {
            value: new_value,
            unit: self.unit.clone(),
        })
    }
}

impl<'a, V, U> ops::TrySubRef<'a> for Measurement<V, U>
where
    U: type_traits::Unit<'a, V> + Clone + 'a,
    &'a U: ToString,
    V: PartialOrd + Clone + Mul<V, Output = V> + 'a,
    V: Sub<V, Output = V>,
{
    type Error = <Self as TryConvertTo<'a, U>>::Error;

    fn try_sub_ref(&'a self, rhs: &'a Self) -> Result<Self, Self::Error> {
        let rhs_converted = rhs.try_convert_to(&self.unit)?;
        let new_value = Sub::sub(self.value.clone(), rhs_converted.value);

        Ok(Self {
            value: new_value,
            unit: self.unit.clone(),
        })
    }
}

impl ops::MulRef for Measurement<f64, crate::Unit> {
    fn mul_ref(&self, rhs: &Self) -> Self {
        match rhs.try_convert_to(&self.unit) {
            Ok(converted) => Self {
                value: self.value * converted.value,
                unit: Mul::mul(&self.unit, &converted.unit),
            },
            Err(_) => Self {
                value: self.value * rhs.value,
                unit: Mul::mul(&self.unit, &rhs.unit),
            },
        }
    }
}

impl ops::CheckedMulRef for Measurement<f64, crate::Unit> {
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

impl ops::DivRef for Measurement<f64, crate::Unit> {
    fn div_ref(&self, rhs: &Self) -> Self {
        match rhs.try_convert_to(&self.unit) {
            Ok(converted) => Self {
                value: self.value / converted.value,
                unit: Div::div(&self.unit, &converted.unit),
            },
            Err(_) => Self {
                value: self.value / rhs.value,
                unit: Div::div(&self.unit, &rhs.unit),
            },
        }
    }
}

impl ops::CheckedDivRef for Measurement<f64, crate::Unit> {
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
