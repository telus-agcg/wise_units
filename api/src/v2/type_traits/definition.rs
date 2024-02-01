use crate::parser::function_set::FunctionSet;
use crate::v2::behavior_traits::convert::{TryToMagnitude, TryToScalar};

use super::StaticUnit;

// pub trait StaticDefinition<V>: TryToScalar<V> + TryToMagnitude<V> {
pub trait StaticDefinition<V> {
    // type Unit: StaticUnit;
    type Unit;

    fn value(&self) -> &V;
    fn unit(&self) -> Option<&Self::Unit>;
    fn function_set(&self) -> Option<&FunctionSet>;
}

impl StaticDefinition<f64> for f64 {
    type Unit = ();

    fn value(&self) -> &f64 {
        &1.0
    }

    fn unit(&self) -> Option<&Self::Unit> {
        None
    }

    fn function_set(&self) -> Option<&FunctionSet> {
        None
    }
}
