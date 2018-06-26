use convertible::Convertible;
use field_eq::FieldEq;
use parser::{Composable, Error};
use reducible::Reducible;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use ucum_unit::UcumUnit;
use unit::Unit;

/// A Measurement is the prime interface for consumers of the library. It
/// consists of some scalar value and a `Unit`, where the Unit represents the
/// type of unit.
///
/// # Examples
///
/// ```
/// use wise_units::{Convertible, Measurement};
///
/// let one_km = Measurement::new(1.0, "km").unwrap();
/// let in_meters = one_km.convert_to("m").unwrap();
///
/// assert!(in_meters.value == 1000.0);
/// ```
///
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Measurement {
    pub value: f64,
    pub unit: Unit,
}

#[cfg(all(any(target_arch = "wasm32", target_os = "emscripten"), feature = "with_stdweb"))]
js_serializable!(Measurement);

#[cfg(all(any(target_arch = "wasm32", target_os = "emscripten"), feature = "with_stdweb"))]
js_deserializable!(Measurement);

impl Measurement {
    /// Creates a new `Measurement` by parsing `expression` into a `Unit`.
    ///
    pub fn new(value: f64, expression: &str) -> Result<Self, Error> {
        let unit = Unit::from_str(expression)?;

        let m = Self { value, unit };

        Ok(m)
    }

    fn converted_scalar(&self, other_unit: &Unit) -> f64 {
        if self.is_special() && other_unit.is_special() {
            let ts = self.unit.reduce_value(self.value);
            other_unit.calculate_magnitude(ts)
        } else if self.is_special() {
            self.unit.reduce_value(self.value)
        } else if other_unit.is_special() {
            other_unit.calculate_magnitude(self.value)
        } else {
            self.scalar() / other_unit.reduce_value(1.0)
        }
    }
}

//-----------------------------------------------------------------------------
// impl UcumUnit
//-----------------------------------------------------------------------------
impl UcumUnit for Measurement {
    /// Checks if the associated Unit is "special". "Special" units are ones
    /// that must be converted using a function in combination with some other
    /// non-special units. For example, Celsius is special since it must be
    /// first converted to Kelvin before converting to the requested unit.
    ///
    fn is_special(&self) -> bool {
        self.unit.is_special()
    }

    fn is_arbitrary(&self) -> bool {
        self.unit.is_arbitrary()
    }

    fn is_metric(&self) -> bool {
        self.unit.is_metric()
    }

    /// This scalar is the Measurement's value combined with any scalars that
    /// are part of the Unit's designation.
    ///
    /// # Examples
    ///
    /// ```
    /// use wise_units::{Measurement, UcumUnit};
    ///
    /// let five_meters = Measurement::new(5.0, "m").unwrap();
    /// assert_eq!(five_meters.scalar(), 5.0);
    ///
    /// let five_meters_squared = Measurement::new(5.0, "m2").unwrap();
    /// assert_eq!(five_meters_squared.scalar(), 5.0);
    ///
    /// let five_three_meters = Measurement::new(5.0, "[pi].m").unwrap();
    /// assert_eq!(five_three_meters.scalar(), 15.707_963_267_948_966);
    ///
    /// let sixty_five_f = Measurement::new(65.0, "[degF]").unwrap();
    /// assert!((sixty_five_f.scalar() - 291.483_333).abs() < 0.000_001);
    /// ```
    ///
    fn scalar(&self) -> f64 {
        self.reduce_value(self.value)
    }

    /// This magnitude is the Measurement's value combined with any magnitude
    /// that is part of the Unit's designation.
    ///
    /// # Examples
    ///
    /// ```
    /// use wise_units::{Measurement, UcumUnit};
    ///
    /// let five_meters = Measurement::new(5.0, "m").unwrap();
    /// assert_eq!(five_meters.magnitude(), 5.0);
    ///
    /// let five_meters_squared = Measurement::new(5.0, "m2").unwrap();
    /// assert_eq!(five_meters_squared.magnitude(), 5.0);
    ///
    /// let five_three_meters = Measurement::new(5.0, "[pi].m").unwrap();
    /// assert_eq!(five_three_meters.magnitude(), 5.0);
    ///
    /// let sixty_five_f = Measurement::new(65.0, "[degF]").unwrap();
    /// assert!((sixty_five_f.magnitude() - 65.0).abs() < 0.000_001);
    /// ```
    ///
    fn magnitude(&self) -> f64 {
        self.calculate_magnitude(self.value)
    }
}

//-----------------------------------------------------------------------------
// impl Reducible
//-----------------------------------------------------------------------------
impl Reducible for Measurement {
    fn reduce_value(&self, value: f64) -> f64 {
        if self.is_special() {
            self.unit.reduce_value(value)
        } else {
            value * self.unit.reduce_value(1.0)
        }
    }

    fn calculate_magnitude(&self, value: f64) -> f64 {
        if self.is_special() {
            let scalar = self.scalar();
            self.unit.calculate_magnitude(scalar)
        } else {
            value * self.unit.calculate_magnitude(1.0)
        }
    }
}

//-----------------------------------------------------------------------------
// impl Convertible
//-----------------------------------------------------------------------------
/// This implementation of `Convertible` lets you pass in a `&str` for the
/// `Unit`, which will parse the chars and convert accordingly. If `expression`
/// is invalid, you'll get an `Error`. If `self`'s `Unit` and `other_unit` are
/// incompatible, you'll get an `Error`.
///
impl<'a> Convertible<&'a str> for Measurement {
    fn convert_to(&self, expression: &'a str) -> Result<Self, Error> {
        let other_unit = Unit::from_str(expression)?;

        convert_measurement(self, &other_unit)
    }
}

/// This implementation of `Convertible` skips any string parsing and gets
/// right to converting to `other_unit`. If `self`'s `Unit` and `other_unit`
/// are incompatible, you'll get an `Error`.
///
impl<'a> Convertible<&'a Unit> for Measurement {
    fn convert_to(&self, other_unit: &'a Unit) -> Result<Self, Error> {
        convert_measurement(self, other_unit)
    }
}

fn convert_measurement(lhs: &Measurement, dest_unit: &Unit) -> Result<Measurement, Error> {
    // Short-circuit if `dest_unit` is the same as the Measurement's Unit.
    if lhs.unit.field_eq(dest_unit) {
        return Ok(lhs.clone());
    }

    let source_unit = &lhs.unit;

    if !source_unit.is_compatible_with(&dest_unit) {
        let e = Error::IncompatibleUnitTypes {
            lhs: source_unit.expression(),
            rhs: dest_unit.expression(),
        };
        return Err(e);
    }

    let new_measurement = Measurement {
        value: lhs.converted_scalar(dest_unit),
        unit: dest_unit.clone(),
    };

    Ok(new_measurement)
}

//-----------------------------------------------------------------------------
// impl Display
//-----------------------------------------------------------------------------
impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

//-----------------------------------------------------------------------------
// impl FieldEq
//-----------------------------------------------------------------------------
/// This is for comparing `Measurement`s to see if they have both the same
/// `value` *and* the same underlying `Unit` defined in the exact same terms.
///
/// ```rust
/// use wise_units::{FieldEq, Measurement};
///
/// // Both of these have the same value and are defined in the same terms.
/// let measurement = Measurement::new(1.0, "m").unwrap();
/// let other = Measurement::new(1.0, "m").unwrap();
/// assert!(measurement.field_eq(&other));
///
/// // These have the same magnitude, but otherwise are never equal.
/// let measurement = Measurement::new(1.0, "m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert!(!measurement.field_eq(&other));
///
/// // These scalar values are equal, but since the units are in different
/// // terms, they are not `field_eq`.
/// let measurement = Measurement::new(1.0, "1000m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert!(!measurement.field_eq(&other));
///
/// // Neither the values nor unit terms are equal, thus are not `field_eq` here.
/// let measurement = Measurement::new(1000.0, "m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert!(!measurement.field_eq(&other));
/// ```
///
impl<'a> FieldEq<'a> for Measurement {
    fn field_eq(&self, other: &'a Self) -> bool {
        self.value == other.value && self.unit.field_eq(&other.unit)
    }
}

//-----------------------------------------------------------------------------
// impl PartialEq
//-----------------------------------------------------------------------------
/// `Measurement`s are `PartialEq` if
///
/// a) their `Unit`s are compatible
/// b) their `scalar()` values are equal
///
/// ```rust
/// use wise_units::Measurement;
///
/// let measurement = Measurement::new(1.0, "m").unwrap();
/// let other = Measurement::new(1.0, "m").unwrap();
/// assert!(measurement == other);
///
/// let measurement = Measurement::new(1.0, "m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert!(measurement != other);
///
/// let measurement = Measurement::new(1.0, "1000m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert!(measurement == other);
///
/// let measurement = Measurement::new(1000.0, "m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert!(measurement == other);
/// ```
///
impl PartialEq for Measurement {
    fn eq(&self, other: &Self) -> bool {
        if !self.unit.is_compatible_with(&other.unit) {
            return false;
        }

        self.scalar() == other.scalar()
    }
}

//-----------------------------------------------------------------------------
// impl PartialOrd
//-----------------------------------------------------------------------------
/// This allows for comparing `Measurement`s based on their reduced scalar
/// values.
///
/// ```rust
/// use std::cmp::Ordering;
/// use wise_units::Measurement;
///
/// let measurement = Measurement::new(1.0, "m").unwrap();
/// let other = Measurement::new(1.0, "m").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Equal);
///
/// let measurement = Measurement::new(1.0, "m").unwrap();
/// let other = Measurement::new(2.0, "m").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Less);
/// assert!(measurement < other);
///
/// let measurement = Measurement::new(2.0, "m").unwrap();
/// let other = Measurement::new(1.0, "m").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Greater);
/// assert!(measurement > other);
///
/// let measurement = Measurement::new(1000.0, "m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Equal);
///
/// let measurement = Measurement::new(1000.0, "m").unwrap();
/// let other = Measurement::new(2.0, "km").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Less);
/// assert!(measurement < other);
///
/// let measurement = Measurement::new(1000.1, "m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Greater);
/// assert!(measurement > other);
///
/// let measurement = Measurement::new(1.0, "1000m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Equal);
///
/// let measurement = Measurement::new(1.0, "1000m").unwrap();
/// let other = Measurement::new(2.0, "km").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Less);
/// assert!(measurement < other);
///
/// let measurement = Measurement::new(1.1, "1000m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Greater);
/// assert!(measurement > other);
/// ```
///
impl PartialOrd for Measurement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if !self.unit.is_compatible_with(&other.unit) {
            return None;
        }

        let other_scalar = other.scalar();
        let my_scalar = self.scalar();

        my_scalar.partial_cmp(&other_scalar)
    }
}

//-----------------------------------------------------------------------------
// impl Add
//-----------------------------------------------------------------------------
impl Add for Measurement {
    type Output = Result<Self, Error>;

    fn add(self, other: Self) -> Self::Output {
        add_measurements(&self, &other)
    }
}

impl<'a> Add for &'a Measurement {
    type Output = Result<Measurement, Error>;

    fn add(self, other: &'a Measurement) -> Self::Output {
        add_measurements(self, other)
    }
}

impl<'a> Add for &'a mut Measurement {
    type Output = Result<Measurement, Error>;

    fn add(self, other: &'a mut Measurement) -> Self::Output {
        add_measurements(self, other)
    }
}

fn add_measurements(lhs: &Measurement, rhs: &Measurement) -> Result<Measurement, Error> {
    let rhs_converted = rhs.convert_to(&lhs.unit)?;
    let new_value = lhs.value + rhs_converted.value;

    Ok(Measurement {
        value: new_value,
        unit: lhs.unit.clone(),
    })
}

//-----------------------------------------------------------------------------
// impl Sub
//-----------------------------------------------------------------------------
impl Sub for Measurement {
    type Output = Result<Self, Error>;

    fn sub(self, other: Self) -> Self::Output {
        sub_measurements(&self, &other)
    }
}

impl<'a> Sub for &'a Measurement {
    type Output = Result<Measurement, Error>;

    fn sub(self, other: &'a Measurement) -> Self::Output {
        sub_measurements(self, other)
    }
}

impl<'a> Sub for &'a mut Measurement {
    type Output = Result<Measurement, Error>;

    fn sub(self, other: &'a mut Measurement) -> Self::Output {
        sub_measurements(self, other)
    }
}

fn sub_measurements(lhs: &Measurement, rhs: &Measurement) -> Result<Measurement, Error> {
    let rhs_converted = rhs.convert_to(&lhs.unit)?;
    let new_value = lhs.value - rhs_converted.value;

    Ok(Measurement {
        value: new_value,
        unit: lhs.unit.clone(),
    })
}

//-----------------------------------------------------------------------------
// impl Mul
//-----------------------------------------------------------------------------
impl Mul for Measurement {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        mul_measurements(&self, &other)
    }
}

impl<'a> Mul for &'a Measurement {
    type Output = Measurement;

    fn mul(self, other: &'a Measurement) -> Self::Output {
        mul_measurements(self, other)
    }
}

impl<'a> Mul for &'a mut Measurement {
    type Output = Measurement;

    fn mul(self, other: &'a mut Measurement) -> Self::Output {
        mul_measurements(self, other)
    }
}

fn mul_measurements(lhs: &Measurement, rhs: &Measurement) -> Measurement {
    let new_value = lhs.value * rhs.value;
    let new_unit = &lhs.unit * &rhs.unit;

    Measurement {
        value: new_value,
        unit: new_unit,
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

//-----------------------------------------------------------------------------
// impl Div
//-----------------------------------------------------------------------------
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

impl<'a> Div for &'a mut Measurement {
    type Output = Measurement;

    fn div(self, other: &'a mut Measurement) -> Self::Output {
        div_measurements(self, other)
    }
}

fn div_measurements(lhs: &Measurement, rhs: &Measurement) -> Measurement {
    let new_value = lhs.value / rhs.value;
    let new_unit = &lhs.unit / &rhs.unit;

    Measurement {
        value: new_value,
        unit: new_unit,
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

#[cfg(test)]
mod tests {
    use super::super::parser::{Atom, Term};
    use super::*;
    use unit::Unit;

    #[test]
    fn validate_new() {
        let m = Measurement::new(1.0, "m").unwrap();

        let expected_unit = Unit {
            terms: vec![term!(Meter)],
        };

        assert_eq!(m.value, 1.0);
        assert_eq!(m.unit, expected_unit);
    }

    #[test]
    fn validate_scalar() {
        let m = Measurement::new(1.0, "m").unwrap();
        assert_ulps_eq!(m.scalar(), 1.0);

        let m = Measurement::new(2.3, "m").unwrap();
        assert_ulps_eq!(m.scalar(), 2.3);

        let m = Measurement::new(1.0, "km").unwrap();
        assert_ulps_eq!(m.scalar(), 1000.0);

        let m = Measurement::new(2.3, "km").unwrap();
        assert_ulps_eq!(m.scalar(), 2300.0);

        let m = Measurement::new(1.0, "g/L").unwrap();
        assert_ulps_eq!(m.scalar(), 1000.0);

        let m = Measurement::new(2.3, "g/L").unwrap();
        assert_ulps_eq!(m.scalar(), 2300.0);

        let m = Measurement::new(20.0, "Cel").unwrap();
        assert_ulps_eq!(m.scalar(), 293.15);
    }

    #[test]
    fn validate_magnitude() {
        let m = Measurement::new(1.0, "m").unwrap();
        assert_ulps_eq!(m.magnitude(), 1.0);

        let m = Measurement::new(2.3, "m").unwrap();
        assert_ulps_eq!(m.magnitude(), 2.3);

        let m = Measurement::new(1.0, "km").unwrap();
        assert_ulps_eq!(m.magnitude(), 1000.0);

        let m = Measurement::new(2.3, "km").unwrap();
        assert_ulps_eq!(m.magnitude(), 2300.0);

        let m = Measurement::new(1.0, "g/L").unwrap();
        assert_ulps_eq!(m.magnitude(), 1.0);

        let m = Measurement::new(2.3, "g/L").unwrap();
        assert_ulps_eq!(m.magnitude(), 2.3);

        let m = Measurement::new(20.0, "g/10L").unwrap();
        assert_ulps_eq!(m.magnitude(), 2.0);

        let m = Measurement::new(20.0, "Cel").unwrap();
        assert_ulps_eq!(m.magnitude(), 20.0);
    }

    // TODO: moar tests
    mod convertible {
        use super::*;

        #[test]
        fn validate_convert_to_meter_to_meter() {
            let meter = Measurement::new(1.0, "m").unwrap();
            let other = meter.convert_to("m").unwrap();
            assert_eq!(other, meter);
            assert_eq!(other.value, 1.0);

            let other = meter.convert_to(&meter.unit).unwrap();
            assert_eq!(other, meter);
            assert_eq!(other.value, 1.0);
        }

        #[test]
        fn validate_convert_to_meter_to_2meter() {
            let meter = Measurement::new(1.0, "m").unwrap();
            let mut other = meter.convert_to("m").unwrap();
            other.value = 2.0;
            assert_ne!(other, meter);
        }

        #[test]
        fn validate_convert_to_meter_to_km() {
            let meter = Measurement::new(1000.0, "m").unwrap();
            let other = meter.convert_to("km").unwrap();
            assert_eq!(other, meter);
            assert_eq!(other.value, 1.0);

            let other = meter.convert_to(&other.unit).unwrap();
            assert_eq!(other, meter);
            assert_eq!(other.value, 1.0);
        }

        #[test]
        fn validate_convert_to_meter_to_2km() {
            let meter = Measurement::new(1.0, "m").unwrap();
            let mut other = meter.convert_to("km").unwrap();
            other.value = 2.0;
            assert_ne!(other, meter);
        }
    }

    mod display {
        use super::*;

        #[test]
        fn validate_display() {
            assert_eq!(
                Measurement::new(1.1, "m").unwrap().to_string(),
                "1.1m".to_string()
            );
            assert_eq!(
                Measurement::new(1.1, "m2").unwrap().to_string(),
                "1.1m2".to_string()
            );
            assert_eq!(
                Measurement::new(1.1, "km2").unwrap().to_string(),
                "1.1km2".to_string()
            );
            assert_eq!(
                Measurement::new(1.1, "km2/s").unwrap().to_string(),
                "1.1km2/s".to_string()
            );
            assert_eq!(
                Measurement::new(1.1, "km2/rad.s").unwrap().to_string(),
                "1.1km2/rad.s".to_string()
            );
        }
    }

    mod field_eq {
        use super::*;

        #[test]
        fn validate_field_eq() {
            let measurement = Measurement::new(1.0, "ar").unwrap();
            let other = Measurement::new(1.0, "ar").unwrap();
            assert!(measurement.field_eq(&other));

            let measurement = Measurement::new(1.0, "ar").unwrap();
            let other = Measurement::new(1.0, "har").unwrap();
            assert!(!measurement.field_eq(&other));

            let measurement = Measurement::new(1.0, "100ar").unwrap();
            let other = Measurement::new(1.0, "har").unwrap();
            assert!(!measurement.field_eq(&other));

            let measurement = Measurement::new(100.0, "ar").unwrap();
            let other = Measurement::new(1.0, "har").unwrap();
            assert!(!measurement.field_eq(&other));
        }
    }

    mod partial_eq {
        use super::*;

        #[test]
        fn validate_eq_same_unit() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(1.0, "m").unwrap();
            assert!(&m1 == &m2);

            let m2 = Measurement::new(1.1, "m").unwrap();
            assert!(m1 != m2);
        }

        #[test]
        fn validate_eq_unit_with_prefix() {
            let m = Measurement::new(1000.0, "m").unwrap();
            let km = Measurement::new(1.0, "km").unwrap();
            assert!(&m == &km);

            let km = Measurement::new(1.1, "km").unwrap();
            assert!(&m != &km);
        }

        #[test]
        fn validate_eq_unit_with_derived() {
            let m2 = Measurement::new(10_000.0, "m2").unwrap();
            let har = Measurement::new(1.0, "har").unwrap();
            assert!(m2 == har);

            let har = Measurement::new(1.1, "har").unwrap();
            assert!(m2 != har);
        }

        #[test]
        fn validate_eq_incompatible_unit() {
            let m = Measurement::new(1.0, "m").unwrap();
            let s = Measurement::new(1.0, "s").unwrap();
            assert!(&m != &s);
        }
    }

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
        fn validate_add_mut_borrowed() {
            let mut m1 = Measurement::new(1.0, "m").unwrap();
            let mut m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(3.0, "m").unwrap();

            assert_eq!((&mut m1 + &mut m2).unwrap(), expected);
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
        fn validate_sub_mut_borrowed() {
            let mut m1 = Measurement::new(1.0, "m").unwrap();
            let mut m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(-1.0, "m").unwrap();

            assert_eq!((&mut m1 - &mut m2).unwrap(), expected);
        }
    }

    mod mul {
        use super::*;

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
        fn validate_mul_mut_borrowed() {
            let mut m1 = Measurement::new(2.0, "m").unwrap();
            let mut m2 = Measurement::new(3.0, "m").unwrap();
            let r = &mut m1 * &mut m2;

            assert_eq!(r.value, 6.0);

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);
            assert_eq!(terms[1], first_term);
        }

        #[test]
        fn validate_mul_integer() {
            let m = Measurement::new(10.0, "m").unwrap();
            let expected = Measurement::new(200.0, "m").unwrap();

            assert_eq!(m.mul(20.0), expected);
        }
    }

    mod div {
        use super::*;

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
        fn validate_div_mut_borrowed() {
            let mut m1 = Measurement::new(10.0, "m").unwrap();
            let mut m2 = Measurement::new(2.0, "m").unwrap();
            let r = &mut m1 / &mut m2;

            assert_eq!(r.value, 5.0);

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);

            let last_term = term!(Meter, exponent: -1);
            assert_eq!(terms[1], last_term);
        }

        #[test]
        fn validate_div_scalar() {
            let m = Measurement::new(10.0, "m").unwrap();
            let expected = Measurement::new(2.0, "m").unwrap();

            assert_eq!(m.div(5.0), expected);
        }
    }

    #[cfg(feature = "with_serde")]
    mod with_serde {
        use super::super::Measurement;
        use parser::{Atom, Prefix, Term};
        use serde_json;
        use unit::Unit;

        #[test]
        fn validate_serialization_empty_terms() {
            let unit = Unit { terms: vec![] };
            let measurement = Measurement {
                value: 123.4,
                unit: unit,
            };
            let expected_json = r#"{"value":123.4,"unit":{"terms":[]}}"#;

            let j =
                serde_json::to_string(&measurement).expect("Couldn't convert Unit to JSON String");

            assert_eq!(expected_json, j);
        }

        #[test]
        fn validate_serialization_full_terms() {
            let expected_json = r#"{
                "value":123.4,
                "unit":{
                    "terms":[{
                        "atom": "Meter",
                        "prefix": "Centi",
                        "factor": 100,
                        "exponent": 456,
                        "annotation": "stuff"
                    }, {
                        "atom": "Gram",
                        "prefix": null,
                        "factor": 1,
                        "exponent": -4,
                        "annotation": null
                    }]
                }
            }"#.replace("\n", "")
                .replace(" ", "");

            let term1 = term!(Centi, Meter, factor: 100, exponent: 456, annotation: Some("stuff".to_string()));
            let term2 = term!(Gram, exponent: -4);

            let unit = Unit {
                terms: vec![term1, term2],
            };
            let measurement = Measurement {
                value: 123.4,
                unit: unit,
            };

            let j =
                serde_json::to_string(&measurement).expect("Couldn't convert Unit to JSON String");

            assert_eq!(expected_json, j);
        }

        #[test]
        fn validate_deserialization_empty_terms() {
            let json = r#"{"value":1.0, "unit":{"terms": []}}"#;

            let k = serde_json::from_str(json).expect("Couldn't convert JSON String to Unit");

            let unit = Unit { terms: vec![] };
            let expected_measurement = Measurement {
                value: 1.0,
                unit: unit,
            };

            assert_eq!(expected_measurement, k);
        }

        #[test]
        fn validate_deserialization_full_terms() {
            let json = r#"{
                "value":432.1,
                "unit":{
                    "terms":[{
                        "atom": "Meter",
                        "prefix": "Centi",
                        "factor": 100,
                        "exponent": 456,
                        "annotation": "stuff"
                    }, {
                        "atom": "Gram",
                        "prefix": null,
                        "factor": 1,
                        "exponent": -4,
                        "annotation": null
                    }]
                }
            }"#;

            let k = serde_json::from_str(json).expect("Couldn't convert JSON String to Unit");

            let term1 = term!(Centi, Meter, factor: 100, exponent: 456, annotation: Some("stuff".to_string()));
            let term2 = term!(Gram, exponent: -4);

            let unit = Unit {
                terms: vec![term1, term2],
            };
            let expected_measurement = Measurement {
                value: 432.1,
                unit: unit,
            };

            assert_eq!(expected_measurement, k);
        }
    }
}
