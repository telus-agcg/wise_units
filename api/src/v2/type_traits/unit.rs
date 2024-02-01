use crate::{Classification, Composition, Dimension, Property};

use super::definition::StaticDefinition;

pub trait StaticUnit<V> {
    // type Unit: StaticUnit<V>;
    type Definition: StaticDefinition<V>;

    fn primary_code(&self) -> &'static str;
    fn secondary_code(&self) -> Option<&'static str>;
    fn print_symbol(&self) -> Option<&'static str>;
    fn names(&self) -> Names;

    fn is_special(&self) -> bool;
    fn is_metric(&self) -> bool;
    fn is_arbitrary(&self) -> bool;

    fn class(&self) -> Classification;
    fn property(&self) -> Property;
    fn dimensions(&self) -> Composition;

    fn value(&self) -> &Self::Definition;
    // fn value(&self) -> &V;
    // fn unit(&self) -> Option<&Self::Unit>;
    // fn function_set(&self) -> Option<&FunctionSet>;
}

/// Some Units have two names; some have one.
#[derive(Debug, Clone, Copy)]
pub enum Names {
    One(&'static str),
    Two((&'static str, &'static str)),
}

// pub trait BasicStaticUnit<V>: StaticUnit<V> {
pub trait BasicStaticUnit<V> {
    fn primary_code(&self) -> &'static str;
    fn secondary_code(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn print_symbol(&self) -> &'static str;
    fn property(&self) -> Property;
    fn dim(&self) -> Dimension;
}

// impl<T, V> StaticUnit<V> for T
// where
//     T: BasicStaticUnit<V>,
//     // V: num_traits::One + StaticDefinition<V>,
//     V: StaticDefinition<V>,
//     <T as StaticUnit<V>>::Definition: num_traits::One,
// {
//     type Definition = V;
//
//     fn primary_code(&self) -> &'static str {
//         <T as BasicStaticUnit<V>>::primary_code(self)
//     }
//
//     fn secondary_code(&self) -> Option<&'static str> {
//         Some(<T as BasicStaticUnit<V>>::secondary_code(self))
//     }
//
//     fn print_symbol(&self) -> Option<&'static str> {
//         Some(<T as BasicStaticUnit<V>>::print_symbol(self))
//     }
//
//     fn names(&self) -> Names {
//         Names::One(<T as BasicStaticUnit<V>>::name(self))
//     }
//
//     #[inline]
//     fn is_special(&self) -> bool {
//         false
//     }
//
//     #[inline]
//     fn is_metric(&self) -> bool {
//         true
//     }
//
//     #[inline]
//     fn is_arbitrary(&self) -> bool {
//         false
//     }
//
//     fn class(&self) -> Classification {
//         Classification::Si
//     }
//
//     fn property(&self) -> Property {
//         <T as BasicStaticUnit<V>>::property(self)
//     }
//
//     fn dimensions(&self) -> Composition {
//         Composition::from(<T as BasicStaticUnit<V>>::dim(self))
//     }
//
//     fn value(&self) -> &Self::Definition {
//         // &<T as StaticUnit<V>>::Definition::from(num_traits::One::one())
//         &num_traits::One::one()
//     }
//
//     // fn value(&self) -> &V {
//     //     num_traits::One::one()
//     // }
// }

#[cfg(test)]
mod tests {
    use crate::v2::behavior_traits::convert::{ToMagnitude, ToScalar};

    use super::*;

    struct Meter;

    impl ToScalar<f64> for Meter {
        fn to_scalar(&self) -> f64 {
            1.0
        }
    }

    impl ToMagnitude<f64> for Meter {
        fn to_magnitude(&self) -> f64 {
            1.0
        }
    }

    impl BasicStaticUnit<f64> for Meter {
        fn primary_code(&self) -> &'static str {
            "m"
        }

        fn secondary_code(&self) -> &'static str {
            "M"
        }

        fn name(&self) -> &'static str {
            "meter"
        }

        fn print_symbol(&self) -> &'static str {
            "m"
        }

        fn property(&self) -> Property {
            Property::Length
        }

        fn dim(&self) -> Dimension {
            Dimension::Length
        }
    }

    impl StaticUnit<f64> for Meter {
        type Definition;

        fn primary_code(&self) -> &'static str {
            todo!()
        }

        fn secondary_code(&self) -> Option<&'static str> {
            todo!()
        }

        fn print_symbol(&self) -> Option<&'static str> {
            todo!()
        }

        fn names(&self) -> Names {
            todo!()
        }

        fn is_special(&self) -> bool {
            todo!()
        }

        fn is_metric(&self) -> bool {
            todo!()
        }

        fn is_arbitrary(&self) -> bool {
            todo!()
        }

        fn class(&self) -> Classification {
            todo!()
        }

        fn property(&self) -> Property {
            todo!()
        }

        fn dimensions(&self) -> Composition {
            todo!()
        }

        fn value(&self) -> &Self::Definition {
            todo!()
        }
    }

    #[test]
    fn base_unit_test() {}

    // #[test]
    // fn unit_test() {
    //     struct Liter;
    //
    //     impl ToScalar<f64> for Liter {
    //         fn to_scalar(&self) -> f64 {
    //             0.001
    //         }
    //     }
    //
    //     impl ToMagnitude<f64> for Liter {
    //         fn to_magnitude(&self) -> f64 {
    //             1.0
    //         }
    //     }
    //
    //     impl StaticDefinition<f64> for Liter {
    //         type Unit = crate::Unit;
    //
    //         fn value(&self) -> &f64 {
    //             &1.0
    //         }
    //
    //         fn unit(&self) -> Option<&Self::Unit> {
    //             todo!()
    //         }
    //
    //         fn function_set(&self) -> Option<&FunctionSet> {
    //             None
    //         }
    //     }
    // }
}
