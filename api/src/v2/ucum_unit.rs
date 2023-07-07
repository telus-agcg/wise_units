#![allow(clippy::module_name_repetitions)]
//! These traits are a progression from the existing `crate::UcumUnit` trait, allowing the same
//! functionality, but also allowing downstream crates to implement for wrapper types.

pub trait UcumUnitFlags {
    fn is_special(&self) -> bool;
    fn is_metric(&self) -> bool;
    fn is_arbitrary(&self) -> bool;
}

impl<T> UcumUnitFlags for T
where
    T: crate::UcumUnit,
{
    fn is_special(&self) -> bool {
        crate::UcumUnit::is_special(self)
    }

    fn is_metric(&self) -> bool {
        crate::UcumUnit::is_metric(self)
    }

    fn is_arbitrary(&self) -> bool {
        crate::UcumUnit::is_arbitrary(self)
    }
}

pub trait UcumUnitComputedValues<T> {
    fn scalar(&self) -> T;
    fn magnitude(&self) -> T;
}

impl<T> UcumUnitComputedValues<f64> for T
where
    T: crate::UcumUnit,
{
    fn scalar(&self) -> f64 {
        crate::UcumUnit::scalar(self)
    }

    fn magnitude(&self) -> f64 {
        crate::UcumUnit::magnitude(self)
    }
}
