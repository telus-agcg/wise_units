//! This module defines functionality for both `Add` and `Sub`. Their behavior are very similar
//! internally and it's easier to test them both if they're in the same module.
//!
use std::ops::{Add, Sub};

use crate::{convertible::Convertible, error::Error, measurement::Measurement};

//-----------------------------------------------------------------------------
// impl Add
//-----------------------------------------------------------------------------
#[cfg_attr(
    feature = "cffi",
    ffi_common::derive::expose_fn(extend_type(Measurement))
)]
fn add_measurements(lhs: &Measurement, rhs: &Measurement) -> Result<Measurement, Error> {
    let rhs_converted = rhs.convert_to(&lhs.unit)?;
    let new_value = lhs.value + rhs_converted.value;

    Ok(Measurement {
        value: new_value,
        unit: lhs.unit.clone(),
    })
}

impl Add for Measurement {
    type Output = Result<Self, Error>;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        add_measurements(&self, &other)
    }
}

impl<'a> Add<&'a Self> for Measurement {
    type Output = Result<Self, Error>;

    #[inline]
    fn add(self, other: &'a Self) -> Self::Output {
        add_measurements(&self, other)
    }
}

impl<'a> Add for &'a Measurement {
    type Output = Result<Measurement, Error>;

    #[inline]
    fn add(self, other: &'a Measurement) -> Self::Output {
        add_measurements(self, other)
    }
}

impl<'a> Add<Measurement> for &'a Measurement {
    type Output = Result<Measurement, Error>;

    #[inline]
    fn add(self, other: Measurement) -> Self::Output {
        add_measurements(self, &other)
    }
}

//-----------------------------------------------------------------------------
// impl Sub
//-----------------------------------------------------------------------------
#[cfg_attr(
    feature = "cffi",
    ffi_common::derive::expose_fn(extend_type(Measurement))
)]
fn sub_measurements(lhs: &Measurement, rhs: &Measurement) -> Result<Measurement, Error> {
    let rhs_converted = rhs.convert_to(&lhs.unit)?;
    let new_value = lhs.value - rhs_converted.value;

    Ok(Measurement {
        value: new_value,
        unit: lhs.unit.clone(),
    })
}

impl Sub for Measurement {
    type Output = Result<Self, Error>;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        sub_measurements(&self, &other)
    }
}

impl<'a> Sub<&'a Self> for Measurement {
    type Output = Result<Self, Error>;

    #[inline]
    fn sub(self, other: &'a Self) -> Self::Output {
        sub_measurements(&self, other)
    }
}

impl<'a> Sub for &'a Measurement {
    type Output = Result<Measurement, Error>;

    #[inline]
    fn sub(self, other: &'a Measurement) -> Self::Output {
        sub_measurements(self, other)
    }
}

impl<'a> Sub<Measurement> for &'a Measurement {
    type Output = Result<Measurement, Error>;

    #[inline]
    fn sub(self, other: Measurement) -> Self::Output {
        sub_measurements(self, &other)
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::const_units::{
        l1::{CENTIMETER, FOOT, KILOMETER, METER, NANOMETER, NANOPARSEC},
        l2::{
            CENTIMETER_SQUARED, DECIMETER_SQUARED, FOOT_SQUARED, METER_SQUARED, YOCTOPARSEC_SQUARED,
        },
        l3::{DECIMETER_CUBED, METER_CUBED},
        m1::{CENTITONNE, GRAM, KILOGRAM},
        m2::CENTIGRAM_SQUARED,
        t1::SECOND,
        t2::SECOND_SQUARED,
    };

    use super::*;

    macro_rules! validate_add {
        ($lhs:expr, $rhs:expr, $expected:expr) => {
            // borrowed + borrowed
            pretty_assertions::assert_eq!((&$lhs + &$rhs).unwrap(), $expected);

            // borrowed + owned
            pretty_assertions::assert_eq!((&$lhs + $rhs.clone()).unwrap(), $expected);

            // owned + borrowed
            pretty_assertions::assert_eq!(($lhs.clone() + &$rhs).unwrap(), $expected);

            // owned + owned
            pretty_assertions::assert_eq!(($lhs + $rhs).unwrap(), $expected);
        };
    }

    macro_rules! validate_add_error {
        ($lhs:expr, $rhs:expr) => {
            // borrowed + borrowed
            assert!((&$lhs + &$rhs).is_err());

            // borrowed + owned
            assert!((&$lhs + $rhs.clone()).is_err());

            // owned + borrowed
            assert!(($lhs.clone() + &$rhs).is_err());

            // owned + owned
            assert!(($lhs + $rhs).is_err());
        };
    }

    macro_rules! validate_sub {
        ($lhs:expr, $rhs:expr, $expected:expr) => {
            // borrowed - borrowed
            pretty_assertions::assert_eq!((&$lhs - &$rhs).unwrap(), $expected);

            // borrowed - owned
            pretty_assertions::assert_eq!((&$lhs - $rhs.clone()).unwrap(), $expected);

            // owned - borrowed
            pretty_assertions::assert_eq!(($lhs.clone() - &$rhs).unwrap(), $expected);

            // owned - owned
            pretty_assertions::assert_eq!(($lhs - $rhs).unwrap(), $expected);
        };
    }

    macro_rules! validate_sub_error {
        ($lhs:expr, $rhs:expr) => {
            // borrowed - borrowed
            assert!((&$lhs + &$rhs).is_err());

            // borrowed - owned
            assert!((&$lhs + $rhs.clone()).is_err());

            // owned - borrowed
            assert!(($lhs.clone() + &$rhs).is_err());

            // owned - owned
            assert!(($lhs + $rhs).is_err());
        };
    }

    const ONE_METER_SQUARED: Measurement = Measurement::new(1.0, METER_SQUARED);
    const ONE_CENTIMETER_SQUARED: Measurement = Measurement::new(1.0, CENTIMETER_SQUARED);
    const ONE_DECIMETER_SQUARED: Measurement = Measurement::new(1.0, DECIMETER_SQUARED);
    const ONE_DECIMETER_CUBED: Measurement = Measurement::new(1.0, DECIMETER_CUBED);

    mod annotation {
        use super::*;

        #[test]
        fn validate_same_single_term_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(annotation: "tree")));
            let rhs = Measurement::new(2.0, unit!(term!(annotation: "tree")));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(3.0, unit!(term!(annotation: "tree")))
            );
            validate_sub!(
                lhs,
                rhs,
                Measurement::new(-1.0, unit!(term!(annotation: "tree")))
            );
        }

        #[test]
        fn validate_same_multi_term_annotation() {
            let thing_per_tree = parse_unit!("{thing}/{tree}");
            let lhs = Measurement::new(1.0, thing_per_tree.clone());
            let rhs = Measurement::new(2.0, thing_per_tree.clone());

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(3.0, thing_per_tree.clone())
            );
            validate_sub!(lhs, rhs, Measurement::new(-1.0, thing_per_tree.clone()));
        }

        #[test]
        fn validate_different_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(annotation: "tree")));
            let rhs = Measurement::new(2.0, unit!(term!(annotation: "pants")));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }
    }

    mod atom {
        use super::*;

        #[test]
        fn validate_same_atom() {
            let lhs = Measurement::new(1.0, METER);
            let rhs = Measurement::new(2.0, METER);

            validate_add!(lhs.clone(), rhs.clone(), Measurement::new(3.0, METER));
            validate_sub!(lhs, rhs, Measurement::new(-1.0, METER));
        }

        #[test]
        fn validate_different_atom_same_dimension() {
            let meter = Measurement::new(1.0, METER);
            let foot = Measurement::new(1.0, FOOT);

            validate_add!(meter.clone(), foot.clone(), Measurement::new(1.3048, METER));
            validate_sub!(
                meter.clone(),
                foot.clone(),
                Measurement::new(0.695_2, METER)
            );

            validate_add!(
                foot.clone(),
                meter.clone(),
                Measurement::new(4.280_839_895_013_123_5, FOOT)
            );
            validate_sub!(foot, meter, Measurement::new(-2.280_839_895_013_123, FOOT));
        }

        #[test]
        fn validate_different_atom_different_dimension() {
            validate_add_error!(Measurement::new(1.0, METER), Measurement::new(1.0, GRAM));
            validate_sub_error!(Measurement::new(1.0, METER), Measurement::new(1.0, GRAM));
        }
    }

    mod atom_annotation {
        use super::*;

        #[test]
        fn validate_same_atom_same_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(Gram, annotation: "tree")));
            let rhs = Measurement::new(2.0, unit!(term!(Gram, annotation: "tree")));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(3.0, unit!(term!(Gram, annotation: "tree")))
            );
            validate_sub!(
                lhs,
                rhs,
                Measurement::new(-1.0, unit!(term!(Gram, annotation: "tree")))
            );
        }

        #[test]
        fn validate_same_atom_different_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(Gram, annotation: "seed")));
            let rhs = Measurement::new(2.0, unit!(term!(Gram, annotation: "tree")));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_different_atom_same_dimension_same_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(Gram, annotation: "seed")));
            let rhs = Measurement::new(2.0, unit!(term!(PoundAvoirdupois, annotation: "seed")));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(908.184_74, unit!(term!(Gram, annotation: "seed")))
            );
            validate_sub!(
                lhs,
                rhs,
                Measurement::new(-906.184_74, unit!(term!(Gram, annotation: "seed")))
            );
        }

        #[test]
        fn validate_different_atom_same_dimension_different_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(Gram, annotation: "seed")));
            let rhs = Measurement::new(2.0, unit!(term!(PoundAvoirdupois, annotation: "tree")));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_different_atom_different_dimension_same_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(Gram, annotation: "seed")));
            let rhs = Measurement::new(2.0, unit!(term!(Meter, annotation: "seed")));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }
    }

    mod atom_exponent {
        use super::*;

        #[test]
        fn validate_same_atom_same_exponent() {
            validate_add!(
                ONE_METER_SQUARED,
                Measurement::new(2.0, METER_SQUARED),
                Measurement::new(3.0, METER_SQUARED)
            );
            validate_sub!(
                ONE_METER_SQUARED,
                Measurement::new(2.0, METER_SQUARED),
                Measurement::new(-1.0, METER_SQUARED)
            );
        }

        #[test]
        fn validate_same_atom_different_exponent() {
            validate_add_error!(ONE_METER_SQUARED, Measurement::new(2.0, METER_CUBED));
            validate_sub_error!(ONE_METER_SQUARED, Measurement::new(2.0, METER_CUBED));
        }

        #[test]
        fn validate_different_atom_same_exponent() {
            validate_add_error!(ONE_METER_SQUARED, Measurement::new(2.0, SECOND_SQUARED));
            validate_sub_error!(ONE_METER_SQUARED, Measurement::new(2.0, SECOND_SQUARED));
        }

        #[test]
        fn validate_different_atom_different_dimension() {
            validate_add_error!(ONE_METER_SQUARED, Measurement::new(2.0, SECOND));
            validate_sub_error!(ONE_METER_SQUARED, Measurement::new(2.0, SECOND));
        }

        #[test]
        fn validate_different_atom_same_dimension() {
            validate_add!(
                ONE_METER_SQUARED,
                Measurement::new(1.0, FOOT_SQUARED),
                Measurement::new(1.092_903_04, METER_SQUARED)
            );
            validate_sub!(
                ONE_METER_SQUARED,
                Measurement::new(1.0, FOOT_SQUARED),
                Measurement::new(0.907_096_96, METER_SQUARED)
            );

            validate_add!(
                Measurement::new(1.0, FOOT_SQUARED),
                ONE_METER_SQUARED,
                Measurement::new(11.763_910_416_709_722, FOOT_SQUARED)
            );
            validate_sub!(
                Measurement::new(1.0, FOOT_SQUARED),
                ONE_METER_SQUARED,
                Measurement::new(-9.763_910_416_709_722, FOOT_SQUARED)
            );
        }
    }

    mod atom_exponent_annotation {
        use super::*;

        #[test]
        fn validate_same_atom_same_exponent_same_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(Second, exponent: 2, annotation: "wind")));
            let rhs = Measurement::new(2.0, unit!(term!(Second, exponent: 2, annotation: "wind")));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(3.0, unit!(term!(Second, exponent: 2, annotation: "wind")))
            );
            validate_sub!(
                lhs,
                rhs,
                Measurement::new(-1.0, unit!(term!(Second, exponent: 2, annotation: "wind")))
            );
        }

        #[test]
        fn validate_same_atom_different_exponent_same_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(Second, exponent: 2, annotation: "wind")));
            let rhs = Measurement::new(2.0, unit!(term!(Second, exponent: 3, annotation: "wind")));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_same_atom_same_exponent_different_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(Second, exponent: 2, annotation: "wind")));
            let rhs = Measurement::new(2.0, unit!(term!(Second, exponent: 2, annotation: "earth")));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_different_atom_same_dimension_same_exponent_same_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(Second, exponent: 2, annotation: "wind")));
            let rhs = Measurement::new(1.0, unit!(term!(Minute, exponent: 2, annotation: "wind")));
            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(
                    3601.0,
                    unit!(term!(Second, exponent: 2, annotation: "wind"))
                )
            );
            validate_sub!(
                lhs,
                rhs,
                Measurement::new(
                    -3599.0,
                    unit!(term!(Second, exponent: 2, annotation: "wind"))
                )
            );
        }

        #[test]
        fn validate_different_atom_same_dimension_different_exponent_same_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(Second, exponent: 2, annotation: "wind")));
            let rhs = Measurement::new(1.0, unit!(term!(Minute, exponent: 3, annotation: "wind")));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_different_atom_same_dimension_same_exponent_different_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(Second, exponent: 2, annotation: "wind")));
            let rhs = Measurement::new(1.0, unit!(term!(Minute, exponent: 2, annotation: "fire")));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_different_atom_different_dimension_same_exponent_same_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(Second, exponent: 2, annotation: "wind")));
            let rhs = Measurement::new(1.0, unit!(term!(Gram, exponent: 2, annotation: "wind")));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }
    }

    mod prefix_atom {
        use super::*;

        #[test]
        fn validate_same_prefix_same_atom() {
            let lhs = Measurement::new(1.0, KILOMETER);
            let rhs = Measurement::new(2.0, KILOMETER);

            validate_add!(lhs.clone(), rhs.clone(), Measurement::new(3.0, KILOMETER));
            validate_sub!(lhs, rhs, Measurement::new(-1.0, KILOMETER));
        }

        #[test]
        fn validate_different_prefix_same_atom() {
            let km = Measurement::new(1.0, KILOMETER);
            let cm = Measurement::new(2.0, CENTIMETER);

            validate_add!(
                km.clone(),
                cm.clone(),
                Measurement::new(1.000_02, KILOMETER)
            );
            validate_sub!(km.clone(), cm.clone(), Measurement::new(0.99998, KILOMETER));

            validate_add!(
                cm.clone(),
                km.clone(),
                Measurement::new(100_002.0, CENTIMETER)
            );
            validate_sub!(cm, km, Measurement::new(-99_998.0, CENTIMETER));
        }

        #[test]
        fn validate_same_prefix_different_atom_same_dimension() {
            let nm = Measurement::new(1.0, NANOMETER);
            let np = Measurement::new(1.0, NANOPARSEC);

            validate_add!(
                nm.clone(),
                np.clone(),
                Measurement::new(3.085_678e16, NANOMETER)
            );
            validate_sub!(
                nm.clone(),
                np.clone(),
                Measurement::new(-3.085_678e16, NANOMETER)
            );

            validate_add!(
                np.clone(),
                nm.clone(),
                Measurement::new(
                    1.000_000_000_000_000_032_407_788_498_994_386_322_876_204_192,
                    NANOPARSEC
                )
            );
            validate_sub!(
                np,
                nm,
                Measurement::new(
                    1.000_000_000_000_000_032_407_788_498_994_386_322_876_204_192,
                    NANOPARSEC
                )
            );
        }

        #[test]
        fn validate_same_prefix_different_atom_different_dimension() {
            let lhs = Measurement::new(1.0, KILOMETER);
            let rhs = Measurement::new(1.0, KILOGRAM);

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_different_prefix_different_atom_same_dimension() {
            let kg = Measurement::new(1.0, KILOGRAM);
            let ct = Measurement::new(1.0, CENTITONNE);

            validate_add!(kg.clone(), ct.clone(), Measurement::new(11.0, KILOGRAM));
            validate_sub!(kg, ct, Measurement::new(-9.0, KILOGRAM));
        }
    }

    mod prefix_atom_annotation {
        use super::*;

        #[test]
        fn validate_prefix_same_atom_same_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("kg{tree}"));
            let rhs = Measurement::new(2.0, parse_unit!("kg{tree}"));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(3.0, parse_unit!("kg{tree}"))
            );
            validate_sub!(lhs, rhs, Measurement::new(-1.0, parse_unit!("kg{tree}")));
        }

        #[test]
        fn validate_prefix_same_atom_different_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("kg{tree}"));
            let rhs = Measurement::new(2.0, parse_unit!("kg{pants}"));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_prefix_different_atom_same_dimension_same_annotation() {
            validate_add!(
                Measurement::new(1.0, parse_unit!("cg{tree}")),
                Measurement::new(2.0, parse_unit!("ct{tree}")),
                Measurement::new(2_000_001.0, parse_unit!("cg{tree}"))
            );
            validate_sub!(
                Measurement::new(1.0, parse_unit!("cg{tree}")),
                Measurement::new(2.0, parse_unit!("ct{tree}")),
                Measurement::new(-1_999_999.0, parse_unit!("cg{tree}"))
            );

            validate_add!(
                Measurement::new(1.0, parse_unit!("cg{tree}")),
                Measurement::new(2.0, parse_unit!("[lb_av]{tree}")),
                Measurement::new(90_719.474, parse_unit!("cg{tree}"))
            );
            validate_sub!(
                Measurement::new(1.0, parse_unit!("cg{tree}")),
                Measurement::new(2.0, parse_unit!("[lb_av]{tree}")),
                Measurement::new(-90_717.474, parse_unit!("cg{tree}"))
            );
        }

        #[test]
        fn validate_prefix_different_atom_different_dimension_same_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("Mg{tree}"));
            let rhs = Measurement::new(2.0, parse_unit!("cm{tree}"));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }
    }

    mod prefix_atom_exponent {
        use super::*;

        #[test]
        fn validate_same_prefix_same_atom_same_exponent() {
            validate_add!(
                ONE_DECIMETER_SQUARED,
                ONE_CENTIMETER_SQUARED,
                Measurement::new(1.01, DECIMETER_SQUARED)
            );
            validate_sub!(
                ONE_DECIMETER_SQUARED,
                ONE_CENTIMETER_SQUARED,
                Measurement::new(0.99, DECIMETER_SQUARED)
            );
        }

        #[test]
        fn validate_same_prefix_same_atom_different_exponent() {
            validate_add_error!(ONE_DECIMETER_SQUARED, ONE_DECIMETER_CUBED);
            validate_sub_error!(ONE_DECIMETER_SQUARED, ONE_DECIMETER_CUBED);
        }

        #[test]
        fn validate_different_prefix_different_atom_same_dimension_same_exponent() {
            validate_add!(
                Measurement::new(1.0, YOCTOPARSEC_SQUARED),
                ONE_DECIMETER_SQUARED,
                Measurement::new(10_502_647_553_956.53, YOCTOPARSEC_SQUARED)
            );
            validate_sub!(
                Measurement::new(1.0, YOCTOPARSEC_SQUARED),
                ONE_DECIMETER_SQUARED,
                Measurement::new(-10_502_647_553_954.53, YOCTOPARSEC_SQUARED)
            );
        }

        #[test]
        fn validate_different_prefix_different_atom_different_dimension_same_exponent() {
            let lhs = Measurement::new(1.0, YOCTOPARSEC_SQUARED);
            let rhs = Measurement::new(1.0, CENTIGRAM_SQUARED);

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }
    }

    mod prefix_atom_exponent_annotation {
        use super::*;

        #[test]
        fn validate_same_atom_same_exponent_same_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("kg2{seed}"));
            let rhs = Measurement::new(2.0, parse_unit!("kg2{seed}"));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(3.0, parse_unit!("kg2{seed}"))
            );
            validate_sub!(lhs, rhs, Measurement::new(-1.0, parse_unit!("kg2{seed}")));
        }

        #[test]
        fn validate_same_atom_same_exponent_different_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("kg2{seed}"));

            {
                let rhs = Measurement::new(2.0, parse_unit!("kg2{plant}"));
                validate_add_error!(lhs.clone(), rhs.clone());
                validate_sub_error!(lhs.clone(), rhs);
            }

            let rhs = Measurement::new(2.0, parse_unit!("kg4{seed}/kg2"));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(3.0, parse_unit!("kg2{seed}"))
            );
            validate_sub!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(-1.0, parse_unit!("kg2{seed}"))
            );
        }

        #[test]
        fn validate_same_atom_different_exponent_same_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("kg2{seed}"));
            let rhs = Measurement::new(2.0, parse_unit!("kg3{seed}"));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_different_atom_same_dimension_same_exponent_same_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("kg2{seed}")); // kilogram

            {
                let rhs = Measurement::new(2.0, parse_unit!("nt2{seed}")); // nanotonne

                validate_add!(
                    lhs.clone(),
                    rhs.clone(),
                    Measurement::new(1.000_000_000_002, parse_unit!("kg2{seed}"))
                );
                validate_sub!(
                    lhs.clone(),
                    rhs,
                    Measurement::new(0.999_999_999_998, parse_unit!("kg2{seed}"))
                );
            }

            {
                let rhs = Measurement::new(2.0, parse_unit!("[lb_av]2{seed}"));

                validate_add!(
                    lhs.clone(),
                    rhs.clone(),
                    Measurement::new(1.411_492_076_244_433_8, parse_unit!("kg2{seed}"))
                );
                validate_sub!(
                    lhs.clone(),
                    rhs,
                    Measurement::new(0.588_507_923_755_566_2, parse_unit!("kg2{seed}"))
                );
            }

            {
                let rhs = Measurement::new(2.0, parse_unit!("kg4{seed}/kg2{seed}"));
                validate_add!(
                    lhs.clone(),
                    rhs.clone(),
                    Measurement::new(3.0, parse_unit!("kg2{seed}"))
                );
                validate_sub!(
                    lhs.clone(),
                    rhs,
                    Measurement::new(-1.0, parse_unit!("kg2{seed}"))
                );
            }

            validate_add!(
                lhs,
                Measurement::new(2.0, parse_unit!("ct4{seed}/[lb_av]2{seed}")),
                Measurement::new(97_208.218_095_347_4, parse_unit!("kg2{seed}"))
            );
        }
    }

    mod factor {
        use super::*;

        #[test]
        fn validate_same_factor() {
            let lhs = Measurement::new(1.0, parse_unit!("2"));
            let rhs = Measurement::new(2.0, parse_unit!("2"));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(3.0, parse_unit!("2"))
            );
            validate_sub!(lhs, rhs, Measurement::new(-1.0, parse_unit!("2")));
        }

        // NOTE: While the units are factors of different values, they are of the same dimension,
        // thus it makes sense that these should pass.
        #[test]
        fn validate_different_factor() {
            let lhs = Measurement::new(1.0, parse_unit!("2"));
            let rhs = Measurement::new(2.0, parse_unit!("3"));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(4.0, parse_unit!("2"))
            );
            validate_sub!(lhs, rhs, Measurement::new(-2.0, parse_unit!("2")));
        }

        #[test]
        fn validate_factor_and_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("2"));
            let rhs = Measurement::new(2.0, parse_unit!("{thing}"));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_factor_and_factor_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("2"));
            let rhs = Measurement::new(2.0, parse_unit!("3{thing}"));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_factor_and_factor_exponent() {
            let lhs = Measurement::new(1.0, parse_unit!("2"));
            let rhs = Measurement::new(2.0, unit!(term!(factor: 2, exponent: 2)));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(5.0, parse_unit!("2"))
            );
            validate_sub!(lhs, rhs, Measurement::new(-3.0, parse_unit!("2")));
        }

        #[test]
        fn validate_factor_and_factor_exponent_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("2"));
            let rhs = Measurement::new(
                2.0,
                unit!(term!(factor: 2, exponent: 2, annotation: "meow")),
            );

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_factor_and_dimensionless_atom() {
            let lhs = Measurement::new(1.0, parse_unit!("2"));
            let rhs = Measurement::new(44.0, unit!(term!(PartsPerThousand)));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(1.022, parse_unit!("2"))
            );
            validate_sub!(lhs, rhs, Measurement::new(0.978, parse_unit!("2")));
        }

        #[test]
        fn validate_factor_and_dimensioned_atom() {
            let lhs = Measurement::new(1.0, parse_unit!("2"));
            let rhs = Measurement::new(44.0, unit!(term!(Meter)));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }
    }

    mod factor_annotation {
        use super::*;

        #[test]
        fn validate_same_factor_same_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("2{seed}"));
            let rhs = Measurement::new(2.0, parse_unit!("2{seed}"));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(3.0, parse_unit!("2{seed}"))
            );
            validate_sub!(lhs, rhs, Measurement::new(-1.0, parse_unit!("2{seed}")));
        }

        #[test]
        fn validate_same_factor_different_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("2{seed}"));
            let rhs = Measurement::new(2.0, parse_unit!("2{plant}"));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        // NOTE: While the units are factors of different values, they are of the same dimension,
        // thus it makes sense that these should pass.
        #[test]
        fn validate_different_factor_same_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("2{seed}"));
            let rhs = Measurement::new(2.0, parse_unit!("3{seed}"));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(4.0, parse_unit!("2{seed}"))
            );
            validate_sub!(lhs, rhs, Measurement::new(-2.0, parse_unit!("2{seed}")));
        }

        #[test]
        fn validate_factor_annotation_and_same_annotation() {
            {
                let lhs = Measurement::new(1.0, parse_unit!("2{thing}"));
                let rhs = Measurement::new(2.0, parse_unit!("{thing}"));
                validate_add!(
                    lhs.clone(),
                    rhs.clone(),
                    Measurement::new(2.0, parse_unit!("2{thing}"))
                );
                validate_sub!(lhs, rhs, Measurement::new(0.0, parse_unit!("2{thing}")));
            }

            {
                let lhs = Measurement::new(1.0, parse_unit!("{thing}"));
                let rhs = Measurement::new(2.0, parse_unit!("2{thing}"));

                validate_add!(
                    lhs.clone(),
                    rhs.clone(),
                    Measurement::new(5.0, parse_unit!("{thing}"))
                );
                validate_sub!(lhs, rhs, Measurement::new(-3.0, parse_unit!("{thing}")));
            }
        }

        #[test]
        fn validate_different_factor_different_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("2{seed}"));
            let rhs = Measurement::new(2.0, parse_unit!("3{plant}"));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_factor_annotation_and_factor() {
            let lhs = Measurement::new(2.0, parse_unit!("3{thing}"));
            let rhs = Measurement::new(1.0, parse_unit!("2"));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_factor_annotation_and_factor_exponent() {
            let lhs = Measurement::new(1.0, parse_unit!("2{stuff}"));
            let rhs = Measurement::new(2.0, unit!(term!(factor: 2, exponent: 2)));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_factor_annotation_and_factor_exponent_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("2{meow}"));

            {
                let rhs = Measurement::new(
                    2.0,
                    unit!(term!(factor: 2, exponent: 2, annotation: "meow")),
                );
                validate_add!(
                    lhs.clone(),
                    rhs.clone(),
                    Measurement::new(5.0, unit!(term!(factor: 2, annotation: "meow")))
                );
                validate_sub!(
                    lhs.clone(),
                    rhs,
                    Measurement::new(-3.0, unit!(term!(factor: 2, annotation: "meow")))
                );
            }

            let rhs = Measurement::new(
                2.0,
                unit!(term!(factor: 2, exponent: 1, annotation: "meow")),
            );

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(3.0, parse_unit!("2{meow}"))
            );
            validate_sub!(lhs, rhs, Measurement::new(-1.0, parse_unit!("2{meow}")));
        }

        #[test]
        fn validate_factor_annotation_and_dimensionless_atom() {
            let lhs = Measurement::new(1.0, parse_unit!("2{stuff}"));
            let rhs = Measurement::new(44.0, unit!(term!(PartsPerThousand)));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_factor_annotation_and_dimensionless_atom_same_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("2{stuff}"));
            let rhs = Measurement::new(44.0, unit!(term!(PartsPerThousand, annotation: "stuff")));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(1.022, parse_unit!("2{stuff}"))
            );
            validate_sub!(lhs, rhs, Measurement::new(0.978, parse_unit!("2{stuff}")));
        }

        #[test]
        fn validate_factor_annotation_and_dimensionless_atom_different_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("2{stuff}"));
            let rhs = Measurement::new(44.0, unit!(term!(PartsPerThousand, annotation: "things")));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_factor_annotation_and_dimensioned_atom() {
            let lhs = Measurement::new(1.0, parse_unit!("2{cat}"));
            let rhs = Measurement::new(44.0, unit!(term!(Meter)));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }
    }

    mod factor_exponent {
        use super::*;

        #[test]
        fn validate_same_factor_same_exponent() {
            let lhs = Measurement::new(1.0, unit!(term!(factor: 2, exponent: 2)));
            let rhs = Measurement::new(2.0, unit!(term!(factor: 2, exponent: 2)));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(3.0, unit!(term!(factor: 2, exponent: 2)))
            );
            validate_sub!(
                lhs,
                rhs,
                Measurement::new(-1.0, unit!(term!(factor: 2, exponent: 2)))
            );
        }

        #[test]
        fn validate_same_factor_different_exponent() {
            {
                let lhs = Measurement::new(1.0, unit!(term!(factor: 2, exponent: 2)));
                let rhs = Measurement::new(2.0, unit!(term!(factor: 2, exponent: 3)));
                validate_add!(
                    lhs.clone(),
                    rhs.clone(),
                    Measurement::new(5.0, unit!(term!(factor: 2, exponent: 2)))
                );
                validate_sub!(
                    lhs,
                    rhs,
                    Measurement::new(-3.0, unit!(term!(factor: 2, exponent: 2)))
                );
            }

            let lhs = Measurement::new(1.0, unit!(term!(factor: 2, exponent: 3)));
            let rhs = Measurement::new(2.0, unit!(term!(factor: 2, exponent: 2)));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(2.0, unit!(term!(factor: 2, exponent: 3)))
            );
            validate_sub!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(0.0, unit!(term!(factor: 2, exponent: 3)))
            );
        }

        #[test]
        fn validate_different_factor_same_exponent() {
            let lhs = Measurement::new(1.0, unit!(term!(factor: 2, exponent: 2)));
            let rhs = Measurement::new(2.0, unit!(term!(factor: 3, exponent: 2)));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(5.5, unit!(term!(factor: 2, exponent: 2)))
            );
            validate_sub!(
                lhs,
                rhs,
                Measurement::new(-3.5, unit!(term!(factor: 2, exponent: 2)))
            );
        }

        #[test]
        fn validate_factor_exponent_and_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(factor: 2, exponent: 3)));
            let rhs = Measurement::new(2.0, parse_unit!("{thing}"));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_factor_and_factor_annotation() {
            let lhs = Measurement::new(1.0, parse_unit!("2"));
            let rhs = Measurement::new(2.0, parse_unit!("3{thing}"));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_factor_exponent_and_factor() {
            let lhs = Measurement::new(1.0, unit!(term!(factor: 2, exponent: 2)));
            let rhs = Measurement::new(2.0, parse_unit!("2"));

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(2.0, unit!(term!(factor: 2, exponent: 2)))
            );
            validate_sub!(
                lhs,
                rhs,
                Measurement::new(0.0, unit!(term!(factor: 2, exponent: 2)))
            );
        }

        #[test]
        fn validate_factor_exponent_and_factor_exponent_annotation() {
            let lhs = Measurement::new(1.0, unit!(term!(factor: 2, exponent: 2)));
            let rhs = Measurement::new(
                2.0,
                unit!(term!(factor: 2, exponent: 2, annotation: "meow")),
            );

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }

        #[test]
        fn validate_factor_exponent_and_dimensionless_atom() {
            let lhs = Measurement::new(1.0, unit!(term!(factor: 2, exponent: 2)));
            let rhs = Measurement::new(44.0, unit!(term!(PartsPerThousand)));
            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(1.011, unit!(term!(factor: 2, exponent: 2)))
            );
            validate_sub!(
                lhs,
                rhs,
                Measurement::new(0.989, unit!(term!(factor: 2, exponent: 2)))
            );
        }

        #[test]
        fn validate_factor_and_dimensioned_atom() {
            let lhs = Measurement::new(1.0, unit!(term!(factor: 2, exponent: 2)));
            let rhs = Measurement::new(44.0, unit!(term!(Meter)));

            validate_add_error!(lhs.clone(), rhs.clone());
            validate_sub_error!(lhs, rhs);
        }
    }

    // TODO: Add sub stuff here
    mod factor_exponent_annotation {
        use super::*;

        #[test]
        fn validate_same_factor_same_exponent_same_annotation() {
            validate_add!(
                Measurement::new(
                    1.0,
                    unit!(term!(factor: 2, exponent: 2, annotation: "seed"))
                ),
                Measurement::new(
                    2.0,
                    unit!(term!(factor: 2, exponent: 2, annotation: "seed"))
                ),
                Measurement::new(
                    3.0,
                    unit!(term!(factor: 2, exponent: 2, annotation: "seed"))
                )
            );
            validate_sub!(
                Measurement::new(
                    1.0,
                    unit!(term!(factor: 2, exponent: 2, annotation: "seed"))
                ),
                Measurement::new(
                    2.0,
                    unit!(term!(factor: 2, exponent: 2, annotation: "seed"))
                ),
                Measurement::new(
                    -1.0,
                    unit!(term!(factor: 2, exponent: 2, annotation: "seed"))
                )
            );
        }

        #[test]
        fn validate_same_factor_same_exponent_different_annotation() {
            validate_add_error!(
                Measurement::new(
                    1.0,
                    unit!(term!(factor: 2, exponent: 2, annotation: "seed"))
                ),
                Measurement::new(
                    2.0,
                    unit!(term!(factor: 2, exponent: 2, annotation: "plant"))
                )
            );
            validate_sub_error!(
                Measurement::new(
                    1.0,
                    unit!(term!(factor: 2, exponent: 2, annotation: "seed"))
                ),
                Measurement::new(
                    2.0,
                    unit!(term!(factor: 2, exponent: 2, annotation: "plant"))
                )
            );
        }

        // NOTE: While the units are factors of different values, they are of the same dimension,
        // thus it makes sense that these should pass.
        #[test]
        fn validate_different_factor_different_exponent_same_annotation() {
            validate_add!(
                Measurement::new(
                    1.0,
                    unit!(term!(factor: 2, exponent: 2, annotation: "seed"))
                ),
                Measurement::new(
                    2.0,
                    unit!(term!(factor: 3, exponent: 3, annotation: "seed"))
                ),
                Measurement::new(
                    14.5,
                    unit!(term!(factor: 2, exponent: 2, annotation: "seed"))
                )
            );
            validate_sub!(
                Measurement::new(
                    1.0,
                    unit!(term!(factor: 2, exponent: 2, annotation: "seed"))
                ),
                Measurement::new(
                    2.0,
                    unit!(term!(factor: 3, exponent: 3, annotation: "seed"))
                ),
                Measurement::new(
                    -12.5,
                    unit!(term!(factor: 2, exponent: 2, annotation: "seed"))
                )
            );
        }
    }

    mod factor_atom {
        use super::*;

        #[test]
        fn validate_same_factor_same_atom() {
            let unit = unit!(term!(Liter, factor: 2));

            let lhs = Measurement::new(1.0, unit.clone());
            let rhs = Measurement::new(2.0, unit.clone());

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(3.0, unit.clone())
            );
            validate_sub!(lhs, rhs, Measurement::new(-1.0, unit.clone()));
        }

        #[test]
        fn validate_different_factor_same_atom() {
            let lhs_unit = unit!(term!(Liter, factor: 2));
            let rhs_unit = unit!(term!(Liter, factor: 3));

            let lhs = Measurement::new(1.0, lhs_unit.clone());
            let rhs = Measurement::new(2.0, rhs_unit);

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(4.0, lhs_unit.clone())
            );
            validate_sub!(lhs, rhs, Measurement::new(-2.0, lhs_unit.clone()));
        }

        #[test]
        fn validate_same_factor_different_atom_same_dimension() {
            let lhs_unit = unit!(term!(Gram, factor: 2000));
            let rhs_unit = unit!(term!(Tonne, factor: 2));

            let lhs = Measurement::new(1.0, lhs_unit.clone());
            let rhs = Measurement::new(1.0, rhs_unit.clone());

            validate_add!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(1001.0, lhs_unit.clone())
            );
            validate_sub!(
                lhs.clone(),
                rhs.clone(),
                Measurement::new(-999.0, lhs_unit.clone())
            );

            validate_add!(
                rhs.clone(),
                lhs.clone(),
                Measurement::new(1.001, rhs_unit.clone())
            );
            validate_sub!(rhs, lhs, Measurement::new(0.999, rhs_unit.clone()));
        }

        #[test]
        fn validate_different_atom_different_dimension() {
            let lhs_unit = unit!(term!(Gram, factor: 2));
            let rhs_unit = unit!(term!(Meter, factor: 3));

            validate_add_error!(
                Measurement::new(1.0, lhs_unit.clone()),
                Measurement::new(1.0, rhs_unit.clone())
            );
            validate_sub_error!(
                Measurement::new(1.0, lhs_unit.clone()),
                Measurement::new(1.0, rhs_unit.clone())
            );
        }
    }

    mod factor_atom_annotation {
        use super::*;
    }

    mod factor_atom_exponent {
        use super::*;
    }

    mod factor_atom_exponent_annotation {
        use super::*;
    }

    mod factor_prefix_atom {
        use super::*;
    }

    mod factor_prefix_atom_annotation {
        use super::*;
    }

    mod factor_prefix_atom_exponent {
        use super::*;
    }

    mod factor_prefix_atom_exponent_annotation {
        use super::*;
    }
}
