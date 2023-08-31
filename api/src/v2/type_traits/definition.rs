use crate::v2::behavior_traits::convert::{ToMagnitude, ToScalar};

pub trait Definition<'a, V>: ToScalar<'a, V> + ToMagnitude<'a, V> {
    type Unit;

    fn value(&'a self) -> &'a V;
    fn unit(&'a self) -> Option<&'a Self::Unit>;
}
