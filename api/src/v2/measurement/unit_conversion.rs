use std::ops::Mul;

use crate::v2::{
    behavior_traits::unit_conversion::{self, ToReduced},
    type_traits,
};

use super::Measurement;

impl<'a, V, U> unit_conversion::TryConvertTo<'a, U> for Measurement<V, U>
where
    U: type_traits::Unit<'a, V> + Clone + 'a,
    V: PartialOrd + Clone + Mul<V, Output = V> + 'a,
{
    type Error = unit_conversion::ConversionError<'a, Self, U>;

    fn try_convert_to(&'a self, rhs: &'a U) -> Result<Self, Self::Error> {
        if !self.unit.is_compatible_with(rhs) {
            let e = unit_conversion::ConversionError::new_borrowed(self, rhs);
            return Err(e);
        }

        let new_measurement = Self {
            value: self.value.clone() * rhs.to_scalar(),
            unit: rhs.clone(),
        };

        Ok(new_measurement)
    }
}

// impl<'a> TryConvertTo<'a, &'a crate::Unit> for Measurement<f64, crate::Unit> {
//     type Error = ConversionError<'a, crate::Unit, crate::Unit>;

//     fn try_convert_to(&'a self, rhs: &'a crate::Unit) -> Result<Self, Self::Error> {
//         if !self.unit.is_compatible_with(rhs) {
//             return Err(ConversionError::new_borrowed(&self.unit, rhs));
//         }

//         Ok(Self {
//             value: todo!(),
//             unit: rhs.clone(),
//         })
//     }
// }

// impl<'a, T> TryConvertTo<'a, T> for Measurement<f64, crate::Unit>
// where
//     T: AsRef<str>,
// {
//     type Error = ConversionError<'a, crate::Unit, &'a str>;

//     fn try_convert_to(&'a self, rhs: T) -> Result<Self, Self::Error> {
//         let other_unit = crate::Unit::from_str(rhs)?;

//         self.try_convert_to(&other_unit)
//     }
// }

// impl<V, U> ToReduced for Measurement<V, U>
// where
//     Self: ConvertTo<U, Self>,
//     U: ToReduced,
// {
//     fn to_reduced(&self) -> Self {
//         let reduced_unit = self.unit.to_reduced();

//         self.convert_to(&reduced_unit)
//     }
// }
impl ToReduced for Measurement<f64, crate::Unit> {
    fn to_reduced(&self) -> Self {
        let _reduced_unit = crate::reduce::ToReduced::to_reduced(&self.unit);

        // self.convert_to(&reduced_unit)
        todo!()
    }
}
