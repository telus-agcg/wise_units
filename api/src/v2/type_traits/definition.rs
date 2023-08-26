use crate::v2::behavior_traits::convert::{ToMagnitude, ToScalar};

pub trait Definition<V>: ToScalar<V> + ToMagnitude<V> {
    type Unit;

    fn value(&self) -> &V;
    fn unit(&self) -> &Option<Self::Unit>;
}
