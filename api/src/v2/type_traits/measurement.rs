use std::cmp::Ordering;

use super::Unit;

pub trait Measurement: Sized {
    type Value;
    type Unit: Unit;

    fn value(&self) -> &Self::Value;
    fn unit(&self) -> &Self::Unit;

    fn equals<T>(&self, rhs: &T) -> bool;
    fn is_commensurable_with<T>(&self, rhs: &T) -> bool;
    fn commensurable_ord<T>(&self, rhs: &T) -> Option<Ordering>;

    fn add_ref<T, O>(&self, rhs: &T) -> O;
    fn sub_ref<T, O>(&self, rhs: &T) -> O;
    fn mul_ref<T, O>(&self, rhs: &T) -> O;
    fn div_ref<T, O>(&self, rhs: &T) -> O;
    fn invert(&mut self);
    fn to_inverse<T>(&self) -> T;

    fn to_scalar<T>(&self) -> T;
    fn to_magnitude<T>(&self) -> T;

    fn convert_to<Rhs, O>(&self, rhs: &Rhs) -> O;
}
