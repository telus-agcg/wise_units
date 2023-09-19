use std::ops::Mul;

use num_traits::{Inv, Zero};

use crate::v2::{
    behavior_traits::convert::{
        CheckedInvert, CheckedToInverse, Invert, ToInverse, ToMagnitude, ToScalar,
    },
    measurement::Measurement,
};

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
impl<'a, S, V, U> ToMagnitude<'a, S> for Measurement<V, U>
where
    V: 'a,
    &'a V: Mul<S, Output = S>,
    U: ToMagnitude<'a, S>,
{
    fn to_magnitude(&'a self) -> S {
        Mul::mul(&self.value, self.unit.to_magnitude())
    }
}
