use std::{
    fmt,
    num::NonZeroI8,
    ops::{Div, Mul},
    str::FromStr,
};

pub const DIMENSIONLESS: Dimension = Dimension([None, None, None, None, None, None, None]);

pub const LENGTH: Dimension = Dimension([
    Some(unsafe { NonZeroI8::new_unchecked(1) }),
    None,
    None,
    None,
    None,
    None,
    None,
]);

pub const TIME: Dimension = Dimension([
    None,
    Some(unsafe { NonZeroI8::new_unchecked(1) }),
    None,
    None,
    None,
    None,
    None,
]);

pub const MASS: Dimension = Dimension([
    None,
    None,
    Some(unsafe { NonZeroI8::new_unchecked(1) }),
    None,
    None,
    None,
    None,
]);

pub const PLANE_ANGLE: Dimension = Dimension([
    None,
    None,
    None,
    Some(unsafe { NonZeroI8::new_unchecked(1) }),
    None,
    None,
    None,
]);

pub const TEMPERATURE: Dimension = Dimension([
    None,
    None,
    None,
    None,
    Some(unsafe { NonZeroI8::new_unchecked(1) }),
    None,
    None,
]);

pub const ELECTRIC_CHARGE: Dimension = Dimension([
    None,
    None,
    None,
    None,
    None,
    Some(unsafe { NonZeroI8::new_unchecked(1) }),
    None,
]);

pub const LUMINOUS_INTENSITY: Dimension = Dimension([
    None,
    None,
    None,
    None,
    None,
    None,
    Some(unsafe { NonZeroI8::new_unchecked(1) }),
]);

trait MaybeAdd<T> {
    fn maybe_add(self, rhs: T) -> Self;
}

impl MaybeAdd<i8> for Option<NonZeroI8> {
    fn maybe_add(self, rhs: i8) -> Self {
        match self {
            Some(non_zero) if non_zero.get() == 1 => NonZeroI8::new(rhs),
            Some(non_zero) if non_zero.get() == -1 => NonZeroI8::new(-rhs),
            Some(_) if rhs == 1 => self,
            Some(non_zero) if rhs == -1 => NonZeroI8::new(-non_zero.get()),
            Some(non_zero) => NonZeroI8::new(non_zero.get() + rhs),
            None => None,
        }
    }
}

impl MaybeAdd<Self> for Option<NonZeroI8> {
    fn maybe_add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Some(lhs_non_zero), Some(rhs_non_zero)) => {
                NonZeroI8::new(lhs_non_zero.get() + rhs_non_zero.get())
            }
            (Some(_), None) => self,
            (None, Some(_)) => rhs,
            (None, None) => None,
        }
    }
}

// Order:
//     Length               (L),
//     Time                 (T),
//     Mass                 (M),
//     PlaneAngle           (A),
//     Temperature,         (C)
//     ElectricCharge       (Q),
//     LuminousIntensity    (F),
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Dimension(pub(crate) [Option<NonZeroI8>; 7]);

impl Dimension {
    #[must_use]
    pub const fn new(inner: [Option<NonZeroI8>; 7]) -> Self {
        Self(inner)
    }

    #[must_use]
    pub const fn new_dimensionless() -> Self {
        DIMENSIONLESS
    }

    #[must_use]
    pub fn is_dimensionless(self) -> bool {
        self == DIMENSIONLESS
    }

    #[must_use]
    pub fn into_inverted(self) -> Self {
        self * -1
    }
}

impl fmt::Debug for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_map()
            .entry(&"Length", &self.0[0])
            .entry(&"Time", &self.0[1])
            .entry(&"Mass", &self.0[2])
            .entry(&"PlaneAngle", &self.0[3])
            .entry(&"Temperature", &self.0[4])
            .entry(&"ElectricCharge", &self.0[5])
            .entry(&"LuminousIntensity", &self.0[6])
            .finish()
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut chunks: Vec<String> = Vec::with_capacity(7);

        if let Some(l) = self.0[0] {
            chunks.push(format!("L{l}"));
        }

        if let Some(t) = self.0[1] {
            chunks.push(format!("T{t}"));
        }

        if let Some(m) = self.0[2] {
            chunks.push(format!("M{m}"));
        }

        if let Some(a) = self.0[3] {
            chunks.push(format!("A{a}"));
        }

        if let Some(c) = self.0[4] {
            chunks.push(format!("C{c}"));
        }

        if let Some(q) = self.0[5] {
            chunks.push(format!("Q{q}"));
        }

        if let Some(f) = self.0[6] {
            chunks.push(format!("F{f}"));
        }

        write!(f, "{}", chunks.join("."))
    }
}

impl Mul<i8> for Dimension {
    type Output = Self;

    fn mul(self, exponent: i8) -> Self::Output {
        Self([
            self.0[0].maybe_add(exponent),
            self.0[1].maybe_add(exponent),
            self.0[2].maybe_add(exponent),
            self.0[3].maybe_add(exponent),
            self.0[4].maybe_add(exponent),
            self.0[5].maybe_add(exponent),
            self.0[6].maybe_add(exponent),
        ])
    }
}

impl Mul for Dimension {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0].maybe_add(rhs.0[0]),
            self.0[1].maybe_add(rhs.0[1]),
            self.0[2].maybe_add(rhs.0[2]),
            self.0[3].maybe_add(rhs.0[3]),
            self.0[4].maybe_add(rhs.0[4]),
            self.0[5].maybe_add(rhs.0[5]),
            self.0[6].maybe_add(rhs.0[6]),
        ])
    }
}

impl Div for Dimension {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let r = rhs.into_inverted();

        Self([
            self.0[0].maybe_add(r.0[0]),
            self.0[1].maybe_add(r.0[1]),
            self.0[2].maybe_add(r.0[2]),
            self.0[3].maybe_add(r.0[3]),
            self.0[4].maybe_add(r.0[4]),
            self.0[5].maybe_add(r.0[5]),
            self.0[6].maybe_add(r.0[6]),
        ])
    }
}

/// Used for parsing strings in the format:
///
/// * `L` for length
/// * `L1` for length
/// * `L2` for area (length^2)
/// * `L/T` for length / time
/// * `L.T` for length * time
///
impl FromStr for Dimension {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dims = s
            .split('.')
            .map(|dim_str| {
                let number_part = &dim_str[1..];

                let num = if number_part.is_empty() {
                    Ok(1)
                } else {
                    number_part.parse::<i8>()
                };

                match (str_to_dim(dim_str.chars().next().unwrap()), num) {
                    (Ok(dim), Ok(exponent)) => Ok(dim * exponent),
                    (_, Err(_)) | (Err(_), _) => Err(()),
                }
            })
            .collect::<Result<Vec<Self>, ()>>()?;

        Ok(dims.into_iter().product::<Self>())
    }
}

fn str_to_dim(s: char) -> Result<Dimension, ()> {
    match s {
        'L' => Ok(LENGTH),
        'T' => Ok(TIME),
        'M' => Ok(MASS),
        'A' => Ok(PLANE_ANGLE),
        'C' => Ok(TEMPERATURE),
        'Q' => Ok(ELECTRIC_CHARGE),
        'F' => Ok(LUMINOUS_INTENSITY),
        _ => {
            eprintln!("Unknown base unit dimension: {s}");
            Err(())
        }
    }
}

impl std::iter::Product for Dimension {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut dim = Self::new_dimensionless();

        for d in iter {
            dim = dim * d;
        }

        dim
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod maybe_add_i8 {
        use super::*;

        #[test]
        fn self_is_none_test() {
            let value: Option<NonZeroI8> = None;
            let result = value.maybe_add(42);
            assert!(result.is_none());
        }

        #[test]
        fn self_is_positive_1_test() {
            let value = NonZeroI8::new(1);

            let pos_result = value.maybe_add(42);
            assert_eq!(pos_result, NonZeroI8::new(42));

            let neg_result = value.maybe_add(-42);
            assert_eq!(neg_result, NonZeroI8::new(-42));

            let pos_result = value.maybe_add(1);
            assert_eq!(pos_result, NonZeroI8::new(1));

            let neg_result = value.maybe_add(-1);
            assert_eq!(neg_result, NonZeroI8::new(-1));
        }

        #[test]
        fn self_is_negative_1_test() {
            let value = NonZeroI8::new(-1);

            let neg_result = value.maybe_add(42);
            assert_eq!(neg_result, NonZeroI8::new(-42));

            let pos_result = value.maybe_add(-42);
            assert_eq!(pos_result, NonZeroI8::new(42));

            let neg_result = value.maybe_add(1);
            assert_eq!(neg_result, NonZeroI8::new(-1));

            let pos_result = value.maybe_add(-1);
            assert_eq!(pos_result, NonZeroI8::new(1));
        }

        #[test]
        fn rhs_is_positive_test() {
            let value = NonZeroI8::new(-42);

            let neg_result = value.maybe_add(1);
            assert_eq!(neg_result, NonZeroI8::new(-42));

            let neg_result = value.maybe_add(2);
            assert_eq!(neg_result, NonZeroI8::new(-40));
        }

        #[test]
        fn rhs_is_negative_test() {
            let value = NonZeroI8::new(-42);

            let pos_result = value.maybe_add(-1);
            assert_eq!(pos_result, NonZeroI8::new(42));

            let pos_result = value.maybe_add(-2);
            assert_eq!(pos_result, NonZeroI8::new(-44));
        }

        #[test]
        fn rhs_is_zero_test() {
            let value = NonZeroI8::new(42);
            let pos_result = value.maybe_add(0);
            assert_eq!(pos_result, NonZeroI8::new(42));

            let value = NonZeroI8::new(-42);
            let pos_result = value.maybe_add(0);
            assert_eq!(pos_result, NonZeroI8::new(-42));
        }
    }

    mod maybe_add_option_non_zero {
        use super::*;

        #[test]
        fn self_is_none_rhs_is_none_test() {
            let lhs: Option<NonZeroI8> = None;
            let rhs: Option<NonZeroI8> = None;
            let result = lhs.maybe_add(rhs);
            assert!(result.is_none());
        }

        #[test]
        fn self_is_none_rhs_is_some_test() {
            let lhs: Option<NonZeroI8> = None;
            let rhs = NonZeroI8::new(42);
            let result = lhs.maybe_add(rhs);
            assert_eq!(result, rhs);
        }

        #[test]
        fn self_is_some_rhs_is_none_test() {
            let lhs = NonZeroI8::new(42);
            let rhs: Option<NonZeroI8> = None;
            let result = lhs.maybe_add(rhs);
            assert_eq!(result, lhs);
        }

        #[test]
        fn self_is_some_rhs_is_some_test() {
            let lhs = NonZeroI8::new(42);
            let rhs = NonZeroI8::new(42);
            let result = lhs.maybe_add(rhs);
            assert_eq!(result, NonZeroI8::new(84));

            let rhs = NonZeroI8::new(-42);
            let result = lhs.maybe_add(rhs);
            assert!(result.is_none());
        }
    }

    mod mul_self {
        use super::*;

        #[test]
        fn test_single_base_dimension() {
            let d = Dimension::new_dimensionless() * LENGTH;
            let expected = Dimension([NonZeroI8::new(1), None, None, None, None, None, None]);
            assert_eq!(d, expected);

            let d = LENGTH * LENGTH;
            let expected = Dimension([NonZeroI8::new(2), None, None, None, None, None, None]);
            assert_eq!(d, expected);

            let d = expected * LENGTH;
            let expected = Dimension([NonZeroI8::new(3), None, None, None, None, None, None]);
            assert_eq!(d, expected);
        }

        #[test]
        fn test_two_base_dimension2() {
            let d = LENGTH * MASS;
            let expected = Dimension([
                NonZeroI8::new(1),
                None,
                NonZeroI8::new(1),
                None,
                None,
                None,
                None,
            ]);
            assert_eq!(d, expected);

            let d = expected * expected;
            let expected = Dimension([
                NonZeroI8::new(2),
                None,
                NonZeroI8::new(2),
                None,
                None,
                None,
                None,
            ]);
            assert_eq!(d, expected);

            let d = expected * expected;
            let expected = Dimension([
                NonZeroI8::new(4),
                None,
                NonZeroI8::new(4),
                None,
                None,
                None,
                None,
            ]);
            assert_eq!(d, expected);
        }
    }

    mod mul_i8 {
        use super::*;

        #[test]
        fn test_single_base_dimension() {
            let d = Dimension::new_dimensionless() * 1;
            assert_eq!(d, DIMENSIONLESS);

            let d = Dimension::new_dimensionless() * -1;
            assert_eq!(d, DIMENSIONLESS);

            let d = LENGTH * 2;
            let expected = Dimension([NonZeroI8::new(2), None, None, None, None, None, None]);
            assert_eq!(d, expected);

            let d = LENGTH * 3;
            let expected = Dimension([NonZeroI8::new(3), None, None, None, None, None, None]);
            assert_eq!(d, expected);
        }

        #[test]
        fn test_two_base_dimension2() {
            let subject = LENGTH * MASS;

            let d = subject * 1;
            assert_eq!(d, subject);

            let d = subject * -1;
            let expected = Dimension([
                NonZeroI8::new(-1),
                None,
                NonZeroI8::new(-1),
                None,
                None,
                None,
                None,
            ]);
            assert_eq!(d, expected);

            #[allow(clippy::erasing_op)]
            let d = subject * 0;
            assert_eq!(d, DIMENSIONLESS);

            let d = subject * 2;
            let expected = Dimension([
                NonZeroI8::new(2),
                None,
                NonZeroI8::new(2),
                None,
                None,
                None,
                None,
            ]);
            assert_eq!(d, expected);

            let d = subject * 4;
            let expected = Dimension([
                NonZeroI8::new(4),
                None,
                NonZeroI8::new(4),
                None,
                None,
                None,
                None,
            ]);
            assert_eq!(d, expected);
        }
    }

    mod div_self {
        use super::*;

        #[test]
        fn test_single_base_dimension() {
            let d = Dimension::new_dimensionless() / LENGTH;
            let expected = Dimension([NonZeroI8::new(-1), None, None, None, None, None, None]);
            assert_eq!(d, expected);

            let d = LENGTH / LENGTH;
            assert_eq!(d, DIMENSIONLESS);

            let d = LENGTH / MASS;
            let expected = Dimension([
                NonZeroI8::new(1),
                None,
                NonZeroI8::new(-1),
                None,
                None,
                None,
                None,
            ]);
            assert_eq!(d, expected);
        }

        #[test]
        fn test_two_base_dimension2() {
            let subject = LENGTH * MASS;

            let d = subject / TIME;
            let expected = Dimension([
                NonZeroI8::new(1),
                NonZeroI8::new(-1),
                NonZeroI8::new(1),
                None,
                None,
                None,
                None,
            ]);
            assert_eq!(d, expected);

            let d = subject / DIMENSIONLESS;
            assert_eq!(d, subject);
        }
    }

    mod from_str {
        use super::*;

        #[test]
        fn single_dim_no_number_test() {
            assert_eq!(Dimension::from_str("L").unwrap(), LENGTH);
        }

        #[test]
        fn single_dim_with_0_test() {
            assert_eq!(Dimension::from_str("L0").unwrap(), DIMENSIONLESS);
        }

        #[test]
        fn single_dim_with_positive_one_test() {
            assert_eq!(Dimension::from_str("L1").unwrap(), LENGTH);
        }

        #[test]
        fn single_dim_with_negative_one_test() {
            let expected = Dimension([NonZeroI8::new(-1), None, None, None, None, None, None]);
            assert_eq!(Dimension::from_str("L-1").unwrap(), expected);
        }

        #[test]
        fn single_dim_with_positive_two_test() {
            let expected = Dimension([NonZeroI8::new(2), None, None, None, None, None, None]);
            assert_eq!(Dimension::from_str("L2").unwrap(), expected);
        }

        #[test]
        fn single_dim_with_negative_two_test() {
            let expected = Dimension([NonZeroI8::new(-2), None, None, None, None, None, None]);
            assert_eq!(Dimension::from_str("L-2").unwrap(), expected);
        }

        #[test]
        fn lots_of_dims_test() {
            let expected = Dimension([
                NonZeroI8::new(-2),
                NonZeroI8::new(2),
                NonZeroI8::new(1),
                NonZeroI8::new(1),
                NonZeroI8::new(-3),
                NonZeroI8::new(-14),
                NonZeroI8::new(7),
            ]);
            assert_eq!(
                Dimension::from_str("L-2.F7.C-3.T2.M.A.Q-14").unwrap(),
                expected
            );
        }
    }

    #[test]
    fn into_inverted_test() {
        let subject = LENGTH;
        let inverted = subject.into_inverted();
        let expected = Dimension([NonZeroI8::new(-1), None, None, None, None, None, None]);

        assert_eq!(inverted, expected);

        let subject = Dimension([
            NonZeroI8::new(-1),
            None,
            NonZeroI8::new(2),
            None,
            NonZeroI8::new(-3),
            None,
            NonZeroI8::new(4),
        ]);
        let inverted = subject.into_inverted();
        let expected = Dimension([
            NonZeroI8::new(1),
            None,
            NonZeroI8::new(-2),
            None,
            NonZeroI8::new(3),
            None,
            NonZeroI8::new(-4),
        ]);

        assert_eq!(inverted, expected);
    }

    #[test]
    fn is_dimensionless_test() {
        assert!(Dimension::new_dimensionless().is_dimensionless());
        assert!(!LENGTH.is_dimensionless());
    }

    #[test]
    fn product_test() {
        let dims = [LENGTH, TIME, MASS * -2];
        let product: Dimension = dims.iter().copied().product();
        let expected = Dimension([
            NonZeroI8::new(1),
            NonZeroI8::new(1),
            NonZeroI8::new(-2),
            None,
            None,
            None,
            None,
        ]);

        assert_eq!(product, expected);
    }
}
