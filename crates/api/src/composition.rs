use std::{fmt, ops::Mul};

use crate::{term::Exponent, Dimension};

pub const DIMLESS: Composition = Composition::new_dimless();

pub const ELECTRIC_CHARGE: Composition = Composition::new_electric_charge(1);

pub const LENGTH: Composition = Composition::new_length(1);
pub const AREA: Composition = Composition::new_length(2);
pub const VOLUME: Composition = Composition::new_length(3);

pub const MASS: Composition = Composition::new_mass(1);
pub const LUMINOUS_INTENSITY: Composition = Composition::new_luminous_intensity(1);
pub const TEMPERATURE: Composition = Composition::new_temperature(1);
pub const TIME: Composition = Composition::new_time(1);

// L.T-1
pub const VELOCITY: Composition =
    Composition::new_any(None, Some(1), None, None, None, None, Some(-1));

// L.T-2
pub const ACCELERATION: Composition =
    Composition::new_any(None, Some(1), None, None, None, None, Some(-2));

// M.L-3
pub const DENSITY: Composition =
    Composition::new_any(None, Some(-3), None, Some(1), None, None, None);

// M.L.T-2
pub const FORCE: Composition =
    Composition::new_any(None, Some(1), None, Some(1), None, None, Some(-2));

// M.L-1.T-2
pub const PRESSURE: Composition =
    Composition::new_any(None, Some(-1), None, Some(1), None, None, Some(-2));

// M.L2.T-2
pub const ENEGERY: Composition =
    Composition::new_any(None, Some(2), None, Some(1), None, None, Some(-2));

// M.L2.T-3
pub const POWER: Composition =
    Composition::new_any(None, Some(2), None, Some(1), None, None, Some(-3));

// M.L-1.T-1
pub const DYNAMIC_VISCOSITY: Composition =
    Composition::new_any(None, Some(-1), None, Some(1), None, None, Some(-1));

// L2.T-1
pub const KINEMATIC_VISCOSITY: Composition =
    Composition::new_any(None, Some(2), None, None, None, None, Some(-1));

// L2.Q-1.T-2
pub const SPECIFIC_HEAT: Composition =
    Composition::new_any(None, Some(2), None, None, None, Some(-1), Some(-2));

/// A `Composition` represents the makeup of a `Unit`'s dimensions; only
/// dimensions and each `Unit`s `Term`'s exponent. For example, "m" would
/// effectively have the composition string of "L"; "m2" would be "L2"; "1/m2"
/// would be "L-2". This continues on when a Unit has multiple `Term`s (ex.
/// "mL/(kg.d)").
///
/// For more info, see [https://en.wikipedia.org/wiki/Dimensional_analysis](https://en.wikipedia.org/wiki/Dimensional_analysis).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash, Default)]
pub struct Composition {
    electric_charge: Option<Exponent>,
    length: Option<Exponent>,
    luminous_intensity: Option<Exponent>,
    mass: Option<Exponent>,
    plane_angle: Option<Exponent>,
    temperature: Option<Exponent>,
    time: Option<Exponent>,
}

macro_rules! def_mul_exponent {
    ($meth_name:ident, $composition_method:ident) => {
        fn $meth_name(
            original_value: Option<Exponent>,
            exponent: Exponent,
            new_composition: &mut Composition,
        ) {
            if let Some(self_exponent) = original_value {
                let new_exponent = self_exponent * exponent;

                new_composition.$composition_method = set_exponent(new_exponent);
            }
        }
    };
}

macro_rules! insert_exponent {
    ($composition:expr, $method:ident, $exponent:expr) => {
        match $composition.$method {
            Some(value) => {
                let new_exponent = $exponent + value;

                set_exponent(new_exponent)
            }
            None => Some($exponent),
        }
    };
}

macro_rules! def_add_dimension {
    ($meth_name:ident, $composition_method:ident) => {
        fn $meth_name(
            original_value: Option<Exponent>,
            rhs_composition: Composition,
            new_composition: &mut Composition,
        ) {
            new_composition.$composition_method = if let Some(self_value) = original_value {
                insert_exponent!(rhs_composition, $composition_method, self_value)
            } else {
                rhs_composition.$composition_method
            }
        }
    };
}

impl Composition {
    #[must_use]
    pub const fn new(dimension: Dimension, exponent: Exponent) -> Self {
        match dimension {
            Dimension::ElectricCharge => Self::new_electric_charge(exponent),
            Dimension::Length => Self::new_length(exponent),
            Dimension::LuminousIntensity => Self::new_luminous_intensity(exponent),
            Dimension::Mass => Self::new_mass(exponent),
            Dimension::PlaneAngle => Self::new_plane_angle(exponent),
            Dimension::Temperature => Self::new_temperature(exponent),
            Dimension::Time => Self::new_time(exponent),
        }
    }

    #[must_use]
    pub const fn new_dimless() -> Self {
        Self {
            electric_charge: None,
            length: None,
            luminous_intensity: None,
            mass: None,
            plane_angle: None,
            temperature: None,
            time: None,
        }
    }

    #[must_use]
    pub const fn new_electric_charge(exponent: Exponent) -> Self {
        Self {
            electric_charge: Some(exponent),
            length: None,
            luminous_intensity: None,
            mass: None,
            plane_angle: None,
            temperature: None,
            time: None,
        }
    }

    #[must_use]
    pub const fn new_length(exponent: Exponent) -> Self {
        Self {
            electric_charge: None,
            length: Some(exponent),
            luminous_intensity: None,
            mass: None,
            plane_angle: None,
            temperature: None,
            time: None,
        }
    }

    #[must_use]
    pub const fn new_luminous_intensity(exponent: Exponent) -> Self {
        Self {
            electric_charge: None,
            length: None,
            luminous_intensity: Some(exponent),
            mass: None,
            plane_angle: None,
            temperature: None,
            time: None,
        }
    }

    #[must_use]
    pub const fn new_mass(exponent: Exponent) -> Self {
        Self {
            electric_charge: None,
            length: None,
            luminous_intensity: None,
            mass: Some(exponent),
            plane_angle: None,
            temperature: None,
            time: None,
        }
    }

    #[must_use]
    pub const fn new_plane_angle(exponent: Exponent) -> Self {
        Self {
            electric_charge: None,
            length: None,
            luminous_intensity: None,
            mass: None,
            plane_angle: Some(exponent),
            temperature: None,
            time: None,
        }
    }

    #[must_use]
    pub const fn new_temperature(exponent: Exponent) -> Self {
        Self {
            electric_charge: None,
            length: None,
            luminous_intensity: None,
            mass: None,
            plane_angle: None,
            temperature: Some(exponent),
            time: None,
        }
    }

    #[must_use]
    pub const fn new_time(exponent: Exponent) -> Self {
        Self {
            electric_charge: None,
            length: None,
            luminous_intensity: None,
            mass: None,
            plane_angle: None,
            temperature: None,
            time: Some(exponent),
        }
    }

    #[must_use]
    pub const fn new_any(
        electric_charge: Option<Exponent>,
        length: Option<Exponent>,
        luminous_intensity: Option<Exponent>,
        mass: Option<Exponent>,
        plane_angle: Option<Exponent>,
        temperature: Option<Exponent>,
        time: Option<Exponent>,
    ) -> Self {
        Self {
            electric_charge,
            length,
            luminous_intensity,
            mass,
            plane_angle,
            temperature,
            time,
        }
    }

    pub fn insert(&mut self, dimension: Dimension, exponent: Exponent) {
        if exponent == 0 {
            return;
        }

        match dimension {
            Dimension::ElectricCharge => self.insert_electric_charge(exponent),
            Dimension::Length => self.insert_length(exponent),
            Dimension::LuminousIntensity => self.insert_luminous_intensity(exponent),
            Dimension::Mass => self.insert_mass(exponent),
            Dimension::PlaneAngle => self.insert_plane_angle(exponent),
            Dimension::Temperature => self.insert_temperature(exponent),
            Dimension::Time => self.insert_time(exponent),
        }
    }

    fn insert_electric_charge(&mut self, exponent: Exponent) {
        self.electric_charge = insert_exponent!(self, electric_charge, exponent);
    }

    fn insert_length(&mut self, exponent: Exponent) {
        self.length = insert_exponent!(self, length, exponent);
    }

    fn insert_luminous_intensity(&mut self, exponent: Exponent) {
        self.luminous_intensity = insert_exponent!(self, luminous_intensity, exponent);
    }

    fn insert_mass(&mut self, exponent: Exponent) {
        self.mass = insert_exponent!(self, mass, exponent);
    }

    fn insert_plane_angle(&mut self, exponent: Exponent) {
        self.plane_angle = insert_exponent!(self, plane_angle, exponent);
    }

    fn insert_temperature(&mut self, exponent: Exponent) {
        self.temperature = insert_exponent!(self, temperature, exponent);
    }

    fn insert_time(&mut self, exponent: Exponent) {
        self.time = insert_exponent!(self, time, exponent);
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.electric_charge.is_none()
            && self.length.is_none()
            && self.luminous_intensity.is_none()
            && self.mass.is_none()
            && self.plane_angle.is_none()
            && self.temperature.is_none()
            && self.time.is_none()
    }
}

// impl Display
impl fmt::Display for Composition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "");
        }

        let mut expressions = Vec::<String>::new();

        push_display_expression(self.electric_charge, &mut expressions, "Q");
        push_display_expression(self.length, &mut expressions, "L");
        push_display_expression(self.luminous_intensity, &mut expressions, "F");
        push_display_expression(self.mass, &mut expressions, "M");
        push_display_expression(self.plane_angle, &mut expressions, "A");
        push_display_expression(self.temperature, &mut expressions, "C");
        push_display_expression(self.time, &mut expressions, "T");

        write!(f, "{}", expressions.join("."))
    }
}

fn push_display_expression(
    composition_value: Option<Exponent>,
    expressions: &mut Vec<String>,
    dimension_str: &str,
) {
    if let Some(value) = composition_value {
        if value == 1 {
            expressions.push(dimension_str.to_string());
        } else {
            expressions.push(format!("{dimension_str}{value}"));
        }
    }
}

// impl Mul
/// Used for combining two `Compositions`.
///
#[cfg_attr(feature = "cargo-clippy", allow(clippy::suspicious_arithmetic_impl))]
impl Mul for Composition {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_composition = Self::default();

        add_electric_charge(self.electric_charge, rhs, &mut new_composition);
        add_length(self.length, rhs, &mut new_composition);
        add_luminous_intensity(self.luminous_intensity, rhs, &mut new_composition);
        add_mass(self.mass, rhs, &mut new_composition);
        add_plane_angle(self.plane_angle, rhs, &mut new_composition);
        add_temperature(self.temperature, rhs, &mut new_composition);
        add_time(self.time, rhs, &mut new_composition);

        new_composition
    }
}

def_add_dimension!(add_electric_charge, electric_charge);
def_add_dimension!(add_length, length);
def_add_dimension!(add_luminous_intensity, luminous_intensity);
def_add_dimension!(add_mass, mass);
def_add_dimension!(add_plane_angle, plane_angle);
def_add_dimension!(add_temperature, temperature);
def_add_dimension!(add_time, time);

/// Used essentially for calculating the `Composition` of a `Term`. When a `Term` has an exponent
/// set, the `Term`'s `Atom`'s `Composition` must be multiplied by it. For example, if a `Term`
/// has `Atom` `Are` (which is a square meter) and `exponent` 3, the `Composition` should be `L6`
/// which is the `3[the term's exponent]` * `2[the term's atom's length composition]`.
///
/// ```rust
/// use wise_units::{Atom, Composable, Term};
///
/// let t = Term {
///     prefix: None,
///     atom: Some(Atom::Are),
///     exponent: Some(3),
///     factor: None,
///     annotation: None
/// };
///
/// assert_eq!(&t.composition().to_string(), "L6");
/// ```
///
impl Mul<Exponent> for Composition {
    type Output = Self;

    fn mul(self, rhs: Exponent) -> Self::Output {
        let mut new_composition = Self::default();

        mul_electric_charge(self.electric_charge, rhs, &mut new_composition);
        mul_length(self.length, rhs, &mut new_composition);
        mul_luminous_intensity(self.luminous_intensity, rhs, &mut new_composition);
        mul_mass(self.mass, rhs, &mut new_composition);
        mul_plane_angle(self.plane_angle, rhs, &mut new_composition);
        mul_temperature(self.temperature, rhs, &mut new_composition);
        mul_time(self.time, rhs, &mut new_composition);

        new_composition
    }
}

def_mul_exponent!(mul_electric_charge, electric_charge);
def_mul_exponent!(mul_length, length);
def_mul_exponent!(mul_luminous_intensity, luminous_intensity);
def_mul_exponent!(mul_mass, mass);
def_mul_exponent!(mul_plane_angle, plane_angle);
def_mul_exponent!(mul_temperature, temperature);
def_mul_exponent!(mul_time, time);

/// Used internally for disallowing setting any of the dimensions' exponents to 0 (it should
/// be `None` in that case).
///
const fn set_exponent(exponent: Exponent) -> Option<Exponent> {
    if exponent == 0 {
        None
    } else {
        Some(exponent)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::Dimension, Composition};

    #[test]
    fn validate_default() {
        let composition = Composition::default();

        let expected = Composition {
            electric_charge: None,
            length: None,
            luminous_intensity: None,
            mass: None,
            plane_angle: None,
            temperature: None,
            time: None,
        };

        assert_eq!(composition, expected);
    }

    #[test]
    fn validate_new() {
        let composition = Composition::new(Dimension::ElectricCharge, -6);
        assert_eq!(composition.electric_charge, Some(-6));

        let composition = Composition::new(Dimension::Length, -6);
        assert_eq!(composition.length, Some(-6));

        let composition = Composition::new(Dimension::LuminousIntensity, -6);
        assert_eq!(composition.luminous_intensity, Some(-6));

        let composition = Composition::new(Dimension::Mass, -6);
        assert_eq!(composition.mass, Some(-6));

        let composition = Composition::new(Dimension::PlaneAngle, -6);
        assert_eq!(composition.plane_angle, Some(-6));

        let composition = Composition::new(Dimension::Temperature, -6);
        assert_eq!(composition.temperature, Some(-6));

        let composition = Composition::new(Dimension::Time, -6);
        assert_eq!(composition.time, Some(-6));
    }

    #[test]
    fn validate_insert() {
        let mut composition = Composition::default();
        composition.insert(Dimension::Mass, 3);
        assert_eq!(composition.to_string(), "M3");

        composition.insert(Dimension::Mass, 3);
        assert_eq!(composition.to_string(), "M6");

        composition.insert(Dimension::Mass, -6);
        assert_eq!(composition.to_string(), "");

        let mut composition = Composition::default();
        composition.insert(Dimension::Mass, -1);
        composition.insert(Dimension::Temperature, -2);
        composition.insert(Dimension::ElectricCharge, -3);
        composition.insert(Dimension::Time, -4);
        composition.insert(Dimension::Length, -5);
        composition.insert(Dimension::PlaneAngle, -6);
        composition.insert(Dimension::LuminousIntensity, -7);
        assert_eq!(composition.to_string(), "Q-3.L-5.F-7.M-1.A-6.C-2.T-4");
    }

    #[test]
    fn validate_is_empty() {
        let composition = Composition {
            electric_charge: None,
            length: None,
            luminous_intensity: None,
            mass: None,
            plane_angle: None,
            temperature: None,
            time: None,
        };

        assert!(composition.is_empty());

        let composition = Composition {
            electric_charge: Some(1),
            length: None,
            luminous_intensity: None,
            mass: None,
            plane_angle: None,
            temperature: None,
            time: None,
        };

        assert!(!composition.is_empty());
    }

    #[test]
    fn validate_display() {
        use std::convert::AsRef;

        let subject = Composition::default();
        assert_eq!(&subject.to_string(), "");

        let subject = Composition::new(Dimension::ElectricCharge, 1);
        assert_eq!(&subject.to_string(), Dimension::ElectricCharge.as_ref());

        let subject = Composition::new(Dimension::ElectricCharge, 2);
        assert_eq!(&subject.to_string(), "Q2");

        let subject = Composition::new(Dimension::Length, 1);
        assert_eq!(&subject.to_string(), Dimension::Length.as_ref());

        let subject = Composition::new(Dimension::Length, 2);
        assert_eq!(&subject.to_string(), "L2");

        let subject = Composition::new(Dimension::LuminousIntensity, 1);
        assert_eq!(&subject.to_string(), Dimension::LuminousIntensity.as_ref());

        let subject = Composition::new(Dimension::LuminousIntensity, 2);
        assert_eq!(&subject.to_string(), "F2");

        let subject = Composition::new(Dimension::Mass, 1);
        assert_eq!(&subject.to_string(), Dimension::Mass.as_ref());

        let subject = Composition::new(Dimension::Mass, 2);
        assert_eq!(&subject.to_string(), "M2");

        let subject = Composition::new(Dimension::PlaneAngle, 1);
        assert_eq!(&subject.to_string(), Dimension::PlaneAngle.as_ref());

        let subject = Composition::new(Dimension::PlaneAngle, 2);
        assert_eq!(&subject.to_string(), "A2");

        let subject = Composition::new(Dimension::Temperature, 1);
        assert_eq!(&subject.to_string(), Dimension::Temperature.as_ref());

        let subject = Composition::new(Dimension::Temperature, 2);
        assert_eq!(&subject.to_string(), "C2");

        let subject = Composition::new(Dimension::Time, 1);
        assert_eq!(&subject.to_string(), Dimension::Time.as_ref());

        let subject = Composition::new(Dimension::Time, 2);
        assert_eq!(&subject.to_string(), "T2");

        let mut subject = Composition::new(Dimension::ElectricCharge, -2);
        subject.insert(Dimension::Length, -3);
        subject.insert(Dimension::LuminousIntensity, -4);
        subject.insert(Dimension::Mass, 1);
        subject.insert(Dimension::PlaneAngle, 2);
        subject.insert(Dimension::Temperature, 3);
        subject.insert(Dimension::Time, 4);
        assert_eq!(&subject.to_string(), "Q-2.L-3.F-4.M.A2.C3.T4");
    }

    #[test]
    fn validate_mul_composition_lhs_empty() {
        let subject = Composition::default();
        let product = subject * subject;
        assert!(product.is_empty());

        let other = Composition::new(Dimension::Mass, 1);
        let product = subject * other;
        assert_eq!(product.mass, Some(1));

        let other = Composition::new(Dimension::Mass, 2);
        let product = subject * other;
        assert_eq!(product.mass, Some(2));

        let other = Composition::new(Dimension::Mass, -1);
        let product = subject * other;
        assert_eq!(product.mass, Some(-1));

        let other = Composition::new(Dimension::Mass, -2);
        let product = subject * other;
        assert_eq!(product.mass, Some(-2));
    }

    #[test]
    fn validate_mul_composition_lhs_1() {
        let subject = Composition::new(Dimension::Mass, 1);
        let other = Composition::default();
        let product = subject * other;
        assert_eq!(product.mass, Some(1));

        let product = subject * subject;
        assert_eq!(product.mass, Some(2));

        let other = Composition::new(Dimension::Mass, 2);
        let product = subject * other;
        assert_eq!(product.mass, Some(3));

        let other = Composition::new(Dimension::Mass, -1);
        let product = subject * other;
        assert_eq!(product.mass, None);

        let other = Composition::new(Dimension::Mass, -2);
        let product = subject * other;
        assert_eq!(product.mass, Some(-1));
    }

    #[test]
    fn validate_mul_composition_lhs_2() {
        let subject = Composition::new(Dimension::Mass, 2);
        let other = Composition::default();
        let product = subject * other;
        assert_eq!(product.mass, Some(2));

        let other = Composition::new(Dimension::Mass, 1);
        let product = subject * other;
        assert_eq!(product.mass, Some(3));

        let product = subject * subject;
        assert_eq!(product.mass, Some(4));

        let other = Composition::new(Dimension::Mass, -1);
        let product = subject * other;
        assert_eq!(product.mass, Some(1));

        let other = Composition::new(Dimension::Mass, -2);
        let product = subject * other;
        assert_eq!(product.mass, None);

        let other = Composition::new(Dimension::Mass, -3);
        let product = subject * other;
        assert_eq!(product.mass, Some(-1));
    }

    #[test]
    #[allow(clippy::erasing_op, clippy::identity_op)]
    fn validate_mul_i32_lhs_empty() {
        let subject = Composition::default();
        let product = subject * 0;
        assert!(product.is_empty());

        let product = subject * 1;
        assert_eq!(product.mass, None);

        let product = subject * 2;
        assert_eq!(product.mass, None);

        let product = subject * -1;
        assert_eq!(product.mass, None);
    }

    #[test]
    #[allow(clippy::erasing_op, clippy::identity_op)]
    fn validate_mul_i32_lhs_1() {
        let subject = Composition::new(Dimension::Mass, 1);
        let product = subject * 0;
        assert!(product.is_empty());

        let product = subject * 1;
        assert_eq!(product.mass, Some(1));

        let product = subject * 2;
        assert_eq!(product.mass, Some(2));

        let product = subject * -1;
        assert_eq!(product.mass, Some(-1));

        let product = subject * -2;
        assert_eq!(product.mass, Some(-2));
    }

    #[test]
    #[allow(clippy::erasing_op, clippy::identity_op)]
    fn validate_mul_i32_lhs_2() {
        let subject = Composition::new(Dimension::Mass, 2);
        let product = subject * 0;
        assert!(product.is_empty());

        let product = subject * 1;
        assert_eq!(product.mass, Some(2));

        let product = subject * 2;
        assert_eq!(product.mass, Some(4));

        let product = subject * -1;
        assert_eq!(product.mass, Some(-2));

        let product = subject * -2;
        assert_eq!(product.mass, Some(-4));
    }
}
