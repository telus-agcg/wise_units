mod convert;
mod ops;
mod unit_conversion;

use super::type_traits;

#[derive(Clone, Debug)]
pub struct Measurement<V, U> {
    value: V,
    unit: U,
}

impl<'a, V, U> Measurement<V, U>
where
    V: PartialOrd + PartialEq,
    U: type_traits::Unit<'a, V>,
{
    pub const fn new(value: V, unit: U) -> Self {
        Self { value, unit }
    }

    pub const fn value(&self) -> &V {
        &self.value
    }

    pub const fn unit(&self) -> &U {
        &self.unit
    }
}
