use crate::v2::{behavior_traits::ucum::Dimensionable, type_traits};

use super::Measurement;

impl<'a, V, U> Dimensionable for Measurement<V, U>
where
    U: type_traits::Unit<'a, V>,
    V: PartialOrd,
{
    type Output = <U as Dimensionable>::Output;

    fn dim(&self) -> Self::Output {
        self.unit.dim()
    }
}
