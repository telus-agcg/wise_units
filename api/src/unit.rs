use decomposer::{Decomposable, ReductionDecomposer, SimpleDecomposer};
use parser::{Composable, Composition, Error, Term};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Div, Mul};
use std::str::FromStr;

#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Unit {
    pub terms: Vec<Term>,
}

#[cfg(all(any(target_arch = "wasm32", target_os = "emscripten"), feature = "with_stdweb"))]
js_serializable!(Unit);

#[cfg(all(any(target_arch = "wasm32", target_os = "emscripten"), feature = "with_stdweb"))]
js_deserializable!(Unit);

/// A `Unit` is the piece of data that represents a *valid* UCUM unit or custom-defined unit. A
/// `Unit` is defined as a number of `Term`s and thus all methods defined on `Unit` rely on values
/// from its `Terms`.
///
/// The easiest way to create a new `Unit` is via its implementation of `std::str::FromStr`. This
/// parses the given `&str` and returns a `wise_units::parser::Error` if it fails to parse:
///
/// ```rust
/// use wise_units::Unit;
/// use std::str::FromStr;
///
/// let m = Unit::from_str("m2").unwrap();
/// assert_eq!(m.scalar(), 1.0);
///
/// let bad_unit = Unit::from_str("not_a_unit");
/// assert!(bad_unit.is_err());
/// ```
///
impl Unit {
    /// The UCUM defines "special units" as:
    ///
    /// > units that imply a measurement on a scale other than a ratio scale
    ///
    /// Each atom in `Atoms.toml` has the `isSpecial` attribute; a `Unit` is special if any of its
    /// `Term`s has an `Atom` that is special.
    ///
    pub fn is_special(&self) -> bool {
        self.terms.iter().any(|term| term.is_special())
    }

    /// The UCUM defines "metric units" using four points. First:
    ///
    /// > Only metric unit atoms may be combined with a prefix.
    ///
    /// Second:
    ///
    /// > To be metric or not to be metric is a predicate assigned to each unit atom where that unit
    /// > atom is defined.
    ///
    /// Third:
    ///
    /// > All base units are metric. No non-metric unit can be part of the basis.
    ///
    /// Fourth:
    ///
    /// > A unit must be a quantity on a ratio scale in order to be metric.
    ///
    /// This library laxes the first rule and allows any atom/unit to use `Prefix`es. Also this
    /// method only returns `true` when *all* of its `Term`s are metric.
    ///
    pub fn is_metric(&self) -> bool {
        self.terms.iter().all(|term| term.is_metric())
    }

    /// Checks if this unit is really just a wrapper around `Atom::TheUnity`.
    /// This is helpful for knowing, internally, when to stop recursively
    /// calling some functions.
    ///
    pub fn is_unity(&self) -> bool {
        self.terms.len() == 1 && self.terms[0].is_unity()
    }

    /// This gives the scalar value of `self` in terms of `self`'s base-unit(s). It takes account
    /// for each of `self`'s `Term`'s `factor` and `exponent`.
    ///
    /// ```rust
    /// use wise_units::Unit;
    /// use std::str::FromStr;
    ///
    /// // A "km" is 1000 meters.
    /// let unit = Unit::from_str("km").unwrap();
    /// assert_eq!(unit.scalar(), 1000.0);
    ///
    /// // A "10km" is 10_000 meters.
    /// let unit = Unit::from_str("10km").unwrap();
    /// assert_eq!(unit.scalar(), 10_000.0);
    ///
    /// // A "km-1" is 0.001 meters.
    /// let unit = Unit::from_str("km-1").unwrap();
    /// assert_eq!(unit.scalar(), 0.001);
    ///
    /// // A "10km-1" is 0.000_1 meters.
    /// let unit = Unit::from_str("10km-1").unwrap();
    /// assert_eq!(unit.scalar(), 0.000_1);
    ///
    /// // A "km/h" is 0.2777777777777778 meters/second.
    /// let unit = Unit::from_str("km/h").unwrap();
    /// assert_eq!(unit.scalar(), 0.277_777_777_777_777_8);
    ///
    pub fn scalar(&self) -> f64 {
        self.calculate_scalar(1.0)
    }

    /// The scalar value of `self` in terms of `self`'s actual unit(s).
    ///
    /// ```rust
    /// use wise_units::Unit;
    /// use std::str::FromStr;
    ///
    /// // A "km" is 1000 meters.
    /// let unit = Unit::from_str("km").unwrap();
    /// assert_eq!(unit.magnitude(), 1000.0);
    ///
    /// // A "10km" is 10_000 meters.
    /// let unit = Unit::from_str("10km").unwrap();
    /// assert_eq!(unit.magnitude(), 10_000.0);
    ///
    /// // A "km-1" is 0.001 meters.
    /// let unit = Unit::from_str("km-1").unwrap();
    /// assert_eq!(unit.magnitude(), 0.001);
    ///
    /// // A "10km-1" is 0.000_1 meters.
    /// let unit = Unit::from_str("10km-1").unwrap();
    /// assert_eq!(unit.magnitude(), 0.000_1);
    ///
    /// // A "km/h" is 1000 meters/hour.
    /// let unit = Unit::from_str("km/h").unwrap();
    /// assert_eq!(unit.magnitude(), 1000.0);
    ///
    /// // A "m3" is 1 cubic meters.
    /// let unit = Unit::from_str("m3").unwrap();
    /// assert_eq!(unit.magnitude(), 1.0);
    ///
    /// // A "L" is 1 liter.
    /// let unit = Unit::from_str("L").unwrap();
    /// assert_eq!(unit.magnitude(), 1.0);
    ///
    /// // A "10m/5s" is 2 "meters per second".
    /// let unit = Unit::from_str("10m/5s").unwrap();
    /// assert_eq!(unit.magnitude(), 2.0);
    ///
    /// // A "10m/5s2" is 0.4 "meters per second".
    /// let unit = Unit::from_str("10m/5s2").unwrap();
    /// assert_eq!(unit.magnitude(), 0.4);
    ///
    pub fn magnitude(&self) -> f64 {
        self.calculate_magnitude(self.scalar())
    }

    /// Calculates `value` count of `self` in terms of `self`'s base-unit.
    ///
    pub fn calculate_scalar(&self, value: f64) -> f64 {
        self.terms
            .iter()
            .fold(1.0, |acc, term| acc * term.calculate_scalar(value))
    }

    /// Calculates `value` count of `self` in terms of `self`'s unit.
    ///
    pub fn calculate_magnitude(&self, value: f64) -> f64 {
        self.terms
            .iter()
            .fold(1.0, |acc, term| acc * term.calculate_magnitude(value))
    }

    /// Turns the Unit's Terms into Strings and combines them accordingly.
    /// This always returns a String that is parsable back into the same Unit.
    ///
    /// Ex. terms that would normally render `[acr_us].[in_i]/[acr_us]` would
    /// render the same result.
    ///
    /// ```rust
    /// use wise_units::Unit;
    /// use std::str::FromStr;
    ///
    /// let u = Unit::from_str("[acr_us].[in_i]/[acr_us]").unwrap();
    /// assert_eq!(u.expression().as_str(), "[acr_us].[in_i]/[acr_us]");
    /// ```
    ///
    pub fn expression(&self) -> String {
        SimpleDecomposer::new(&self.terms).expression()
    }

    /// If the unit terms are a fraction and can be reduced, this returns those
    /// as a string. Ex. terms that would normally render
    /// `[acr_us].[in_i]/[acr_us]` would simply render `[in_i]`.
    ///
    /// ```rust
    /// use wise_units::Unit;
    /// use std::str::FromStr;
    ///
    /// let u = Unit::from_str("[acr_us].[in_i]/[acr_us]").unwrap();
    /// assert_eq!(u.expression_reduced().as_str(), "[in_i]");
    /// ```
    ///
    pub fn expression_reduced(&self) -> String {
        ReductionDecomposer::new(&self.terms).expression()
    }
}

//-----------------------------------------------------------------------------
// impl Composable
//-----------------------------------------------------------------------------
impl Composable for Unit {
    fn composition(&self) -> Composition {
        self.terms.composition()
    }
}

impl<'a> Composable for &'a Unit {
    fn composition(&self) -> Composition {
        self.terms.composition()
    }
}

//-----------------------------------------------------------------------------
// impl Display
//-----------------------------------------------------------------------------
impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expression())
    }
}

//-----------------------------------------------------------------------------
// impl FromStr
//-----------------------------------------------------------------------------
impl FromStr for Unit {
    type Err = Error;

    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        let terms = super::parser::parse(expression)?;

        Ok(Self { terms })
    }
}

//-----------------------------------------------------------------------------
// impl PartialEq
//-----------------------------------------------------------------------------
/// `Unit`s are `PartialEq` if
///
/// a) they are compatible
/// b) their `scalar()` values are equal
///
impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        if !self.is_compatible_with(other) {
            return false;
        }

        self.scalar() == other.scalar()
    }
}

//-----------------------------------------------------------------------------
// impl PartialOrd
//-----------------------------------------------------------------------------
impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if !self.is_compatible_with(other) {
            return None;
        }

        let other_scalar = other.scalar();
        let my_scalar = self.scalar();

        my_scalar.partial_cmp(&other_scalar)
    }
}

//-----------------------------------------------------------------------------
// impl Div
//-----------------------------------------------------------------------------
impl Div for Unit {
    type Output = Self;

    fn div(self, other: Unit) -> Self::Output {
        let terms = divide_terms(&self.terms, &other.terms);

        Self { terms }
    }
}

impl<'a> Div<&'a Unit> for Unit {
    type Output = Self;

    fn div(self, other: &'a Unit) -> Self::Output {
        let terms = divide_terms(&self.terms, &other.terms);

        Self { terms }
    }
}

impl<'a> Div for &'a Unit {
    type Output = Unit;

    fn div(self, other: &'a Unit) -> Self::Output {
        let terms = divide_terms(&self.terms, &other.terms);

        Unit { terms }
    }
}

impl<'a> Div for &'a mut Unit {
    type Output = Unit;

    fn div(self, other: &'a mut Unit) -> Self::Output {
        let terms = divide_terms(&self.terms, &other.terms);

        Unit { terms }
    }
}

fn divide_terms(lhs: &Vec<Term>, rhs: &Vec<Term>) -> Vec<Term> {
    let mut terms = Vec::with_capacity(lhs.len() + rhs.len());

    for term in lhs.iter() {
        terms.push(term.clone());
    }

    for term in rhs.iter() {
        let mut new_other_term = term.clone();
        new_other_term.exponent = -new_other_term.exponent;
        terms.push(new_other_term);
    }

    terms
}

//-----------------------------------------------------------------------------
// impl Mul
//-----------------------------------------------------------------------------
impl Mul for Unit {
    type Output = Self;

    fn mul(self, other: Unit) -> Self::Output {
        let terms = multiply_terms(&self.terms, &other.terms);

        Self { terms }
    }
}

impl<'a> Mul for &'a Unit {
    type Output = Unit;

    fn mul(self, other: &'a Unit) -> Self::Output {
        let terms = multiply_terms(&self.terms, &other.terms);

        Unit { terms }
    }
}

impl<'a> Mul for &'a mut Unit {
    type Output = Unit;

    fn mul(self, other: &'a mut Unit) -> Self::Output {
        let terms = multiply_terms(&self.terms, &other.terms);

        Unit { terms }
    }
}

fn multiply_terms(lhs: &[Term], rhs: &[Term]) -> Vec<Term> {
    let mut terms = Vec::with_capacity(lhs.len() + rhs.len());

    for term in lhs.iter() {
        terms.push(term.clone());
    }

    for term in rhs.iter() {
        terms.push(term.clone());
    }

    terms
}

#[cfg(test)]
mod tests {
    macro_rules! validate_scalar {
        ($test_name:ident, $input_string:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let unit = Unit::from_str($input_string).unwrap();
                assert_relative_eq!(unit.scalar(), $expected_value);
                assert_ulps_eq!(unit.scalar(), $expected_value);
            }
        };
    }

    macro_rules! validate_scalars {
        ($($test_name: ident, $input_string: expr, $expected_value: expr);+ $(;)*) => {
            $(
                validate_scalar!($test_name, $input_string, $expected_value);
            )+
        };
    }

    macro_rules! validate_magnitude {
        ($test_name:ident, $input_string:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let unit = Unit::from_str($input_string).unwrap();
                assert_relative_eq!(unit.magnitude(), $expected_value);
                assert_ulps_eq!(unit.magnitude(), $expected_value);
            }
        };
    }

    macro_rules! validate_magnitudes {
        ($($test_name: ident, $input_string: expr, $expected_value: expr);+ $(;)*) => {
            $(
                validate_magnitude!($test_name, $input_string, $expected_value);
            )+
        };
    }

    use super::super::parser::{Composition, Dimension};
    use super::*;

    #[test]
    fn validate_from_str_error() {
        let unit = Unit::from_str("ZZZXXXXXXXXXXXXx");
        assert!(unit.is_err());
    }

    #[test]
    fn validate_is_special() {
        let unit = Unit::from_str("m").unwrap();
        assert!(!unit.is_special());
    }

    #[test]
    fn validate_is_unity() {
        // Dimless unit
        let unit = Unit::from_str("[ppth]").unwrap();
        assert!(!unit.is_unity());

        // Regular unit
        let unit = Unit::from_str("m").unwrap();
        assert!(!unit.is_unity());
    }

    validate_scalars!(
        validate_scalar_m, "m", 1.0;
        validate_scalar_km, "km", 1000.0;
        validate_scalar_m_minus_1, "m-1", 1.0;
        validate_scalar_10m, "10m", 10.0;
        validate_scalar_10km, "10km", 10_000.0;
        validate_scalar_10km_minus_1, "10km-1", 0.000_1;
        validate_scalar_10km_minus_1_m2, "10km-1.m2", 0.000_1;
        validate_scalar_km_slash_m2_dot_cm, "km/m2.cm", 100_000.0;
        validate_scalar_km_minus_1_slash_m2_dot_cm, "km-1/m2.cm", 0.1;
        validate_scalar_m_slash_s2, "m/s2", 1.0;
        validate_scalar_slash_1, "/1", 1.0;
        validate_scalar_slash_m, "/m", 1.0;
        validate_scalar_slash_annotation, "/{tot}", 1.0;

        validate_scalar_liter, "l", 0.001;
        validate_scalar_liter_caps, "L", 0.001;
        validate_scalar_pi, "[pi]", ::std::f64::consts::PI;
        validate_scalar_ten_pi, "10[pi]", 10.0 * ::std::f64::consts::PI;
        validate_scalar_hectare, "har", 10_000.0;
        validate_scalar_week, "wk", 604_800.0;
        validate_scalar_kilogram, "kg", 1000.0;
        validate_scalar_fahrenheit, "[degF]", 255.927_777_777_777_8;
    );

    validate_magnitudes!(
        validate_magnitude_m, "m", 1.0;
        validate_magnitude_km, "km", 1000.0;
        validate_magnitude_m_minus_1, "m-1", 1.0;
        validate_magnitude_10m, "10m", 10.0;
        validate_magnitude_10km, "10km", 10_000.0;
        validate_magnitude_10km_minus_1, "10km-1", 0.000_1;
        validate_magnitude_10km_minus_1_m2, "10km-1.m2", 0.000_1;
        validate_magnitude_km_slash_m2_dot_cm, "km/m2.cm", 100_000.0;
        validate_magnitude_km_minus_1_slash_m2_dot_cm, "km-1/m2.cm", 0.1;
        validate_magnitude_m_slash_s2, "m/s2", 1.0;
        validate_magnitude_slash_1, "/1", 1.0;
        validate_magnitude_slash_m, "/m", 1.0;
        validate_magnitude_slash_annotation, "/{tot}", 1.0;

        validate_magnitude_m2, "m2", 1.0;
        validate_magnitude_m3, "m3", 1.0;
        validate_magnitude_liter, "l", 1.0;
        validate_magnitude_liter_caps, "L", 1.0;
        validate_magnitude_8m_slash_4s, "8m/4s", 2.0;
        validate_magnitude_8m_slash_4s2, "8m/4s2", 0.5;

        validate_magnitude_pi, "[pi]", 1.0;
        validate_magnitude_ten_pi, "10[pi]", 10.0;

        // TODO: This doesn't parse (AGDEV-33099)
        // validate_magnitude_decare, "dar", 0.1;
        validate_magnitude_dekare, "daar", 10.0;
        validate_magnitude_hectare, "har", 100.0;
        validate_magnitude_kilare, "kar", 1000.0;

        validate_magnitude_week, "wk", 1.0;
        validate_magnitude_kilogram, "kg", 1000.0;
        validate_magnitude_fahrenheit, "[degF]", 1.000_000_000_000_056_8;
    );

    #[test]
    fn validate_magnitude() {
        let unit = Unit::from_str("m").unwrap();
        assert_eq!(unit.magnitude(), 1.0);

        let unit = Unit::from_str("km").unwrap();
        assert_eq!(unit.magnitude(), 1000.0);

        let unit = Unit::from_str("km/10m").unwrap();
        assert_eq!(unit.magnitude(), 100.0);

        let unit = Unit::from_str("m-1").unwrap();
        assert_eq!(unit.magnitude(), 1.0);

        let unit = Unit::from_str("10m").unwrap();
        assert_eq!(unit.magnitude(), 10.0);

        let unit = Unit::from_str("10km").unwrap();
        assert_eq!(unit.magnitude(), 10_000.0);

        let unit = Unit::from_str("10km-1").unwrap();
        assert_eq!(unit.magnitude(), 0.0001);

        let unit = Unit::from_str("km-1/m2").unwrap();
        assert_eq!(unit.magnitude(), 0.001);

        let unit = Unit::from_str("km/m2.cm").unwrap();
        assert_eq!(unit.magnitude(), 100_000.0);

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        assert_eq!(unit.magnitude(), 0.1);

        let unit = Unit::from_str("m/s2").unwrap();
        assert_eq!(unit.magnitude(), 1.0);

        let unit = Unit::from_str("/1").unwrap();
        assert_eq!(unit.magnitude(), 1.0);

        let unit = Unit::from_str("/m").unwrap();
        assert_eq!(unit.magnitude(), 1.0);

        let unit = Unit::from_str("/{tot}").unwrap();
        assert_eq!(unit.magnitude(), 1.0);
    }

    #[test]
    fn validate_composition() {
        let unit = Unit::from_str("[pi]").unwrap();
        assert_eq!(unit.composition(), Composition::default());

        let unit = Unit::from_str("[ppth]").unwrap();
        assert_eq!(unit.composition(), Composition::default());

        let unit = Unit::from_str("m").unwrap();
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("km").unwrap();
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("km/10m").unwrap();
        let composition = Composition::new(Dimension::Length, 0);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("m-1").unwrap();
        let composition = Composition::new(Dimension::Length, -1);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("10m").unwrap();
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("10km").unwrap();
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("10km-1").unwrap();
        let composition = Composition::new(Dimension::Length, -1);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("km-1/m2").unwrap();
        let composition = Composition::new(Dimension::Length, -3);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("km/m2.cm").unwrap();
        let composition = Composition::new(Dimension::Length, -2);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        let composition = Composition::new(Dimension::Length, -4);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("m/s2").unwrap();
        let mut composition = Composition::new(Dimension::Length, 1);
        composition.insert(Dimension::Time, -2);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("/1").unwrap();
        assert_eq!(unit.composition(), Composition::default());

        let unit = Unit::from_str("/m").unwrap();
        let composition = Composition::new(Dimension::Length, -1);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("/{tot}").unwrap();
        assert_eq!(unit.composition(), Composition::default());
    }

    #[test]
    fn validate_is_compatible_with() {
        let meter = Unit::from_str("m").unwrap();
        let km = Unit::from_str("km").unwrap();
        assert!(meter.is_compatible_with(&km));

        let km_per_10m = Unit::from_str("km/10m").unwrap();
        assert!(!meter.is_compatible_with(&km_per_10m));

        let per_meter = Unit::from_str("m-1").unwrap();
        assert!(!meter.is_compatible_with(&per_meter));

        let ten_meter = Unit::from_str("10m").unwrap();
        assert!(meter.is_compatible_with(&ten_meter));

        let ten_km = Unit::from_str("10km").unwrap();
        assert!(meter.is_compatible_with(&ten_km));

        let per_ten_km = Unit::from_str("10km-1").unwrap();
        assert!(!meter.is_compatible_with(&per_ten_km));

        let per_meter_cubed = Unit::from_str("km-1/m2").unwrap();
        assert!(!meter.is_compatible_with(&per_meter_cubed));

        let km_per_length_cubed = Unit::from_str("km/m2.cm").unwrap();
        assert!(!meter.is_compatible_with(&km_per_length_cubed));

        let km_per_length_fourthed = Unit::from_str("km-1/m2.cm").unwrap();
        assert!(!meter.is_compatible_with(&km_per_length_fourthed));

        let meter_per_second_squared = Unit::from_str("m/s2").unwrap();
        assert!(!meter.is_compatible_with(&meter_per_second_squared));

        let km_cubed_per_nanometer_squared = Unit::from_str("km3/nm2").unwrap();
        assert!(meter.is_compatible_with(&km_cubed_per_nanometer_squared));
    }

    #[test]
    fn validate_expression_reduced() {
        let unit = Unit::from_str("m").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "m");

        let unit = Unit::from_str("M").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "m");

        let unit = Unit::from_str("km/10m").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "km/10m");

        let unit = Unit::from_str("m-1").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "1/m");

        let unit = Unit::from_str("10m").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "10m");

        let unit = Unit::from_str("10km").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "10km");

        let unit = Unit::from_str("10km-1").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "1/10km");

        let unit = Unit::from_str("km-1/m2").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "1/km.m2");

        let unit = Unit::from_str("km/m2.cm").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "km/cm.m2");

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "1/cm.km.m2");

        let unit = Unit::from_str("m/s2").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "m/s2");

        let unit = Unit::from_str("km3/nm2").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "km3/nm2");
    }

    #[test]
    fn validate_display() {
        let unit = Unit::from_str("m").unwrap();
        assert_eq!(unit.to_string().as_str(), "m");

        let unit = Unit::from_str("M").unwrap();
        assert_eq!(unit.to_string().as_str(), "m");

        let unit = Unit::from_str("km/10m").unwrap();
        assert_eq!(unit.to_string().as_str(), "km/10m");

        let unit = Unit::from_str("m-1").unwrap();
        assert_eq!(unit.to_string().as_str(), "1/m");

        let unit = Unit::from_str("10m").unwrap();
        assert_eq!(unit.to_string().as_str(), "10m");

        let unit = Unit::from_str("10km").unwrap();
        assert_eq!(unit.to_string().as_str(), "10km");

        let unit = Unit::from_str("10km-1").unwrap();
        assert_eq!(unit.to_string().as_str(), "1/10km");

        let unit = Unit::from_str("km-1/m2").unwrap();
        assert_eq!(unit.to_string().as_str(), "1/km.m2");

        let unit = Unit::from_str("km/m2.cm").unwrap();
        assert_eq!(unit.to_string().as_str(), "km/m2.cm");

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        assert_eq!(unit.to_string().as_str(), "1/km.m2.cm");

        let unit = Unit::from_str("m/s2").unwrap();
        assert_eq!(unit.to_string().as_str(), "m/s2");

        let unit = Unit::from_str("km3/nm2").unwrap();
        assert_eq!(unit.to_string().as_str(), "km3/nm2");
    }

    #[test]
    fn validate_div() {
        let unit = Unit::from_str("m").unwrap();
        let other = Unit::from_str("km").unwrap();
        assert_eq!(unit.div(other).to_string().as_str(), "m/km");

        let unit = Unit::from_str("10m").unwrap();
        let other = Unit::from_str("20m").unwrap();
        assert_eq!(unit.div(other).to_string().as_str(), "10m/20m");
    }

    #[test]
    fn validate_mul() {
        let unit = Unit::from_str("m").unwrap();
        let other = Unit::from_str("km").unwrap();
        assert_eq!(unit.mul(other).to_string().as_str(), "m.km");

        let unit = Unit::from_str("10m").unwrap();
        let other = Unit::from_str("20m").unwrap();
        assert_eq!(unit.mul(other).to_string().as_str(), "10m.20m");
    }

    #[cfg(feature = "with_serde")]
    mod with_serde {
        use super::super::Unit;
        use parser::{Atom, Prefix, Term};
        use serde_json;

        #[test]
        fn validate_serialization_empty_terms() {
            let unit = Unit { terms: vec![] };
            let expected_json = r#"{"terms":[]}"#;

            let j = serde_json::to_string(&unit).expect("Couldn't convert Unit to JSON String");

            assert_eq!(expected_json, j);
        }

        #[test]
        fn validate_serialization_full_terms() {
            let expected_json = r#"{
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
        }"#.replace("\n", "")
                .replace(" ", "");

            let term1 = term!(Centi, Meter, factor: 100, exponent: 456, annotation: Some("stuff".to_string()));
            let term2 = term!(Gram, factor: 1, exponent: -4);

            let unit = Unit {
                terms: vec![term1, term2],
            };

            let j = serde_json::to_string(&unit).expect("Couldn't convert Unit to JSON String");

            assert_eq!(expected_json, j);
        }

        #[test]
        fn validate_deserialization_empty_terms() {
            let json = r#"{"terms": []}"#;

            let k = serde_json::from_str(json).expect("Couldn't convert JSON String to Unit");

            let expected_unit = Unit { terms: vec![] };

            assert_eq!(expected_unit, k);
        }

        #[test]
        fn validate_deserialization_full_terms() {
            let json = r#"{
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
            }"#;

            let k = serde_json::from_str(json).expect("Couldn't convert JSON String to Unit");

            let term1 = term!(Centi, Meter, factor: 100, exponent: 456, annotation: Some("stuff".to_string()));
            let term2 = term!(Gram, exponent: -4);

            let expected_unit = Unit {
                terms: vec![term1, term2],
            };

            assert_eq!(expected_unit, k);
        }
    }
}
