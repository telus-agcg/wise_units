#![allow(clippy::module_name_repetitions)]

pub trait UcumUnitFlags {
    fn is_special(&self) -> bool;
    fn is_metric(&self) -> bool;
    fn is_arbitrary(&self) -> bool;
}

pub trait UcumUnitComputedValues<T> {
    fn scalar(&self) -> T;
    fn magnitude(&self) -> T;
}
