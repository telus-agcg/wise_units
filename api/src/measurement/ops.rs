use convertible::Convertible;
use measurement::Measurement;
use parser::Error;
use std::ops::{Add, Div, Mul, Sub};

//-----------------------------------------------------------------------------
// impl Add
//-----------------------------------------------------------------------------
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

    fn add(self, other: Self) -> Self::Output {
        add_measurements(&self, &other)
    }
}

impl<'a> Add<&'a Measurement> for Measurement {
    type Output = Result<Self, Error>;

    fn add(self, other: &'a Self) -> Self::Output {
        add_measurements(&self, other)
    }
}

impl<'a> Add for &'a Measurement {
    type Output = Result<Measurement, Error>;

    fn add(self, other: &'a Measurement) -> Self::Output {
        add_measurements(self, other)
    }
}

impl<'a> Add<Measurement> for &'a Measurement {
    type Output = Result<Measurement, Error>;

    fn add(self, other: Measurement) -> Self::Output {
        add_measurements(self, &other)
    }
}

//-----------------------------------------------------------------------------
// impl Sub
//-----------------------------------------------------------------------------
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

    fn sub(self, other: Self) -> Self::Output {
        sub_measurements(&self, &other)
    }
}

impl<'a> Sub<&'a Measurement> for Measurement {
    type Output = Result<Self, Error>;

    fn sub(self, other: &'a Self) -> Self::Output {
        sub_measurements(&self, other)
    }
}

impl<'a> Sub for &'a Measurement {
    type Output = Result<Measurement, Error>;

    fn sub(self, other: &'a Measurement) -> Self::Output {
        sub_measurements(self, other)
    }
}

impl<'a> Sub<Measurement> for &'a Measurement {
    type Output = Result<Measurement, Error>;

    fn sub(self, other: Measurement) -> Self::Output {
        sub_measurements(self, &other)
    }
}

//-----------------------------------------------------------------------------
// impl Mul
//-----------------------------------------------------------------------------
fn mul_measurements(lhs: &Measurement, rhs: &Measurement) -> Measurement {
    let new_value = lhs.value * rhs.value;
    let new_unit = &lhs.unit * &rhs.unit;

    Measurement {
        value: new_value,
        unit: new_unit,
    }
}

impl Mul for Measurement {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        mul_measurements(&self, &other)
    }
}

impl<'a> Mul<&'a Measurement> for Measurement {
    type Output = Self;

    fn mul(self, other: &'a Self) -> Self::Output {
        mul_measurements(&self, other)
    }
}

impl<'a> Mul for &'a Measurement {
    type Output = Measurement;

    fn mul(self, other: &'a Measurement) -> Self::Output {
        mul_measurements(self, other)
    }
}

impl<'a> Mul<Measurement> for &'a Measurement {
    type Output = Measurement;

    fn mul(self, other: Measurement) -> Self::Output {
        mul_measurements(self, &other)
    }
}

/// Multiplies the `Measurement`'s scalar by `other` and returns a new
/// `Measurement`.
///
impl Mul<f64> for Measurement {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        let new_value = self.value * other;

        Self {
            value: new_value,
            unit: self.unit.clone(),
        }
    }
}

impl<'a> Mul<f64> for &'a Measurement {
    type Output = Measurement;

    fn mul(self, other: f64) -> Self::Output {
        let new_value = self.value * other;

        Measurement {
            value: new_value,
            unit: self.unit.clone(),
        }
    }
}

//-----------------------------------------------------------------------------
// impl Div
//-----------------------------------------------------------------------------
fn div_measurements(lhs: &Measurement, rhs: &Measurement) -> Measurement {
    let new_value = lhs.value / rhs.value;
    let new_unit = &lhs.unit / &rhs.unit;

    Measurement {
        value: new_value,
        unit: new_unit,
    }
}

impl Div for Measurement {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        div_measurements(&self, &other)
    }
}

impl<'a> Div for &'a Measurement {
    type Output = Measurement;

    fn div(self, other: &'a Measurement) -> Self::Output {
        div_measurements(self, other)
    }
}

impl<'a> Div<&'a Measurement> for Measurement {
    type Output = Self;

    fn div(self, other: &'a Self) -> Self::Output {
        div_measurements(&self, other)
    }
}

impl<'a> Div<Measurement> for &'a Measurement {
    type Output = Measurement;

    fn div(self, other: Measurement) -> Self::Output {
        div_measurements(self, &other)
    }
}

/// Divides the `Measurement`'s scalar by `other` and returns a new
/// `Measurement`.
///
impl Div<f64> for Measurement {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        let new_value = self.value / other;

        Self {
            value: new_value,
            unit: self.unit.clone(),
        }
    }
}

impl<'a> Div<f64> for &'a Measurement {
    type Output = Measurement;

    fn div(self, other: f64) -> Self::Output {
        let new_value = self.value / other;

        Measurement {
            value: new_value,
            unit: self.unit.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use measurement::Measurement;
    use parser::{Atom, Term};

    mod add {
        use super::*;

        #[test]
        fn validate_add_owned() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(3.0, "m").unwrap();

            assert_eq!((m1 + m2).unwrap(), expected);
        }

        #[test]
        fn validate_add_borrowed() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(3.0, "m").unwrap();

            assert_eq!((&m1 + &m2).unwrap(), expected);
        }

        #[test]
        fn validate_add_owned_and_borrowed() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(3.0, "m").unwrap();

            assert_eq!((m1 + &m2).unwrap(), expected);
        }

        #[test]
        fn validate_add_borrowed_and_owned() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(3.0, "m").unwrap();

            assert_eq!((&m1 + m2).unwrap(), expected);
        }
    }

    mod sub {
        use super::*;

        #[test]
        fn validate_sub_owned() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(-1.0, "m").unwrap();

            assert_eq!((m1 - m2).unwrap(), expected);
        }

        #[test]
        fn validate_sub_borrowed() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(-1.0, "m").unwrap();

            assert_eq!((&m1 - &m2).unwrap(), expected);
        }

        #[test]
        fn validate_sub_owned_and_borrowed() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(-1.0, "m").unwrap();

            assert_eq!((m1 - &m2).unwrap(), expected);
        }

        #[test]
        fn validate_sub_borrowed_and_owned() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(-1.0, "m").unwrap();

            assert_eq!((&m1 - m2).unwrap(), expected);
        }
    }

    mod mul {
        use super::*;
        use std::ops::Mul;

        #[test]
        fn validate_mul_owned() {
            let m1 = Measurement::new(2.0, "m").unwrap();
            let m2 = Measurement::new(3.0, "m").unwrap();
            let r = m1 * m2;

            assert_eq!(r.value, 6.0);

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);
            assert_eq!(terms[1], first_term);
        }

        #[test]
        fn validate_mul_borrowed() {
            let m1 = Measurement::new(2.0, "m").unwrap();
            let m2 = Measurement::new(3.0, "m").unwrap();
            let r = &m1 * &m2;

            assert_eq!(r.value, 6.0);

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);
            assert_eq!(terms[1], first_term);
        }

        #[test]
        fn validate_mul_owned_and_borrowed() {
            let m1 = Measurement::new(2.0, "m").unwrap();
            let m2 = Measurement::new(3.0, "m").unwrap();
            let r = m1 * &m2;

            assert_eq!(r.value, 6.0);

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);
            assert_eq!(terms[1], first_term);
        }

        #[test]
        fn validate_mul_borrowed_and_owned() {
            let m1 = Measurement::new(2.0, "m").unwrap();
            let m2 = Measurement::new(3.0, "m").unwrap();
            let r = &m1 * m2;

            assert_eq!(r.value, 6.0);

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);
            assert_eq!(terms[1], first_term);
        }

        #[test]
        fn validate_mul_f64() {
            let m = Measurement::new(10.0, "m").unwrap();
            let expected = Measurement::new(200.0, "m").unwrap();

            assert_eq!(m.mul(20.0), expected);
        }
    }

    mod div {
        use super::*;
        use std::ops::Div;

        #[test]
        fn validate_div_owned() {
            let m1 = Measurement::new(10.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let r = m1 / m2;

            assert_eq!(r.value, 5.0);

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);

            let last_term = term!(Meter, exponent: -1);
            assert_eq!(terms[1], last_term);
        }

        #[test]
        fn validate_div_borrowed() {
            let m1 = Measurement::new(10.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let r = &m1 / &m2;

            assert_eq!(r.value, 5.0);

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);

            let last_term = term!(Meter, exponent: -1);
            assert_eq!(terms[1], last_term);
        }

        #[test]
        fn validate_div_owned_and_borrowed() {
            let m1 = Measurement::new(10.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let r = m1 / &m2;

            assert_eq!(r.value, 5.0);

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);

            let last_term = term!(Meter, exponent: -1);
            assert_eq!(terms[1], last_term);
        }

        #[test]
        fn validate_div_borrowed_and_owned() {
            let m1 = Measurement::new(10.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let r = &m1 / m2;

            assert_eq!(r.value, 5.0);

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);

            let last_term = term!(Meter, exponent: -1);
            assert_eq!(terms[1], last_term);
        }

        #[test]
        fn validate_div_f64() {
            let m = Measurement::new(10.0, "m").unwrap();
            let expected = Measurement::new(2.0, "m").unwrap();

            assert_eq!(m.div(5.0), expected);
        }
    }
}
