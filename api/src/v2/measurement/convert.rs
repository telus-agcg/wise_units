use std::ops::Mul;

use num_traits::{Inv, Zero};

use crate::v2::{
    behavior_traits::convert::{
        CheckedInvert, CheckedToInverse, ConversionError, Invert, ScalarConversionError, ToInverse,
        ToMagnitude, ToReduced, ToScalar, TryConvertTo, TryToScalar, UnitConversionError,
    },
    type_traits,
};

use super::Measurement;

//-----------------------------------------------------------------------------
// Invert
//-----------------------------------------------------------------------------
impl<V, U> Invert for Measurement<V, U>
where
    V: Inv<Output = V> + Copy,
    U: Invert,
{
    /// Note that this implementation allows for the value to be infinity or NaN. If you want to
    /// guard against that, leverage a `TryInvert` implementation.
    ///
    fn invert(&mut self) {
        self.value = self.value.inv();
        self.unit.invert();
    }
}

impl<V, U> CheckedInvert for Measurement<V, U>
where
    V: Inv<Output = V> + Copy + Zero,
    U: Invert,
{
    fn checked_invert(&mut self) -> Option<()> {
        if self.value.is_zero() {
            return None;
        }

        self.invert();

        Some(())
    }
}

//-----------------------------------------------------------------------------
// *ToInverse
//-----------------------------------------------------------------------------
impl<V, U> ToInverse for Measurement<V, U>
where
    V: Inv<Output = V> + Copy,
    U: ToInverse,
{
    fn to_inverse(&self) -> Self {
        Self {
            value: self.value.inv(),
            unit: self.unit.to_inverse(),
        }
    }
}

impl<V, U> CheckedToInverse for Measurement<V, U>
where
    V: Inv<Output = V> + Copy + Zero,
    U: ToInverse,
{
    fn checked_to_inverse(&self) -> Option<Self> {
        if self.value.is_zero() {
            return None;
        }

        Some(Self {
            value: self.value.inv(),
            unit: self.unit.to_inverse(),
        })
    }
}

//-----------------------------------------------------------------------------
// ToScalar
//-----------------------------------------------------------------------------
impl<V, U> ToScalar<V> for Measurement<V, U>
where
    V: Copy + Mul<V, Output = V>,
    U: ToScalar<V>,
{
    fn to_scalar(&self) -> V {
        self.value * self.unit.to_scalar()
    }
}

//-----------------------------------------------------------------------------
// ToMagnitude
//-----------------------------------------------------------------------------
impl<S, V, U> ToMagnitude<S> for Measurement<V, U>
where
    V: Copy + Mul<S, Output = S>,
    U: ToMagnitude<S>,
{
    fn to_magnitude(&self) -> S {
        self.value * self.unit.to_magnitude()
    }
}

impl<'a, V, U> TryConvertTo<'a, U> for Measurement<V, U>
where
    U: type_traits::Unit<'a, V> + Clone + 'a,
    &'a U: ToString,
    V: PartialOrd + Clone + Mul<V, Output = V> + 'a,
{
    type Error = ConversionError<<U as TryToScalar<V>>::Error>;

    fn try_convert_to(&'a self, rhs: &'a U) -> Result<Self, Self::Error> {
        if !self.unit.dim_eq(rhs) {
            let e = UnitConversionError::new(&self.unit, rhs);
            return Err(ConversionError::Unit(e));
        }

        let new_measurement = Self {
            value: self.value.clone()
                * rhs.try_to_scalar().map_err(|e| {
                    let scalar_error = ScalarConversionError::new(rhs, e);

                    ConversionError::Scalar(scalar_error)
                })?,
            unit: rhs.clone(),
        };

        Ok(new_measurement)
    }
}

impl ToReduced for Measurement<f64, crate::Unit> {
    fn to_reduced(&self) -> Self {
        let _reduced_unit = crate::reduce::ToReduced::to_reduced(&self.unit);

        // self.convert_to(&reduced_unit)
        todo!()
    }
}
