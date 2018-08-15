use super::Dimension;
use std::fmt;
use std::ops::Mul;

type Exponent = i32;

/// A `Composition` represents the makeup of a `Unit`'s dimensions; only
/// dimensions and each `Unit`s `Term`'s exponent. For example, "m" would
/// effectively have the composition string of "L"; "m2" would be "L2"; "1/m2"
/// would be "L-2". This continues on when a Unit has multiple `Term`s (ex.
/// "mL/(kg.d)").
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Composition {
    electric_charge:    Option<Exponent>,
    length:             Option<Exponent>,
    luminous_intensity: Option<Exponent>,
    mass:               Option<Exponent>,
    plane_angle:        Option<Exponent>,
    temperature:        Option<Exponent>,
    time:               Option<Exponent>,
}

macro_rules! mul_exponent {
    ($self_:ident, $method:ident, $exponent:expr, $new_composition:ident) => {
        if let Some(self_exponent) = $self_.$method {
            let new_exponent = self_exponent * $exponent;

            $new_composition.$method = set_exponent(new_exponent);
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
            None => Some($exponent)
        }
    };
}

macro_rules! add_dimension {
    ($composition:ident, $method:ident, $rhs_composition:expr) => {
        if let Some(self_value) = $composition.$method {
            insert_exponent!($rhs_composition, $method, self_value)
        } else {
            $rhs_composition.$method
        }
    };
}

impl Composition {
    pub fn new(dimension: Dimension, exponent: i32) -> Self {
        let mut c = Composition::default();

        match dimension {
            Dimension::ElectricCharge    => c.electric_charge = Some(exponent),
            Dimension::Length            => c.length = Some(exponent),
            Dimension::LuminousIntensity => c.luminous_intensity = Some(exponent),
            Dimension::Mass              => c.mass = Some(exponent),
            Dimension::PlaneAngle        => c.plane_angle = Some(exponent),
            Dimension::Temperature       => c.temperature = Some(exponent),
            Dimension::Time              => c.time = Some(exponent),
        }

        c
    }

    pub fn insert(&mut self, dimension: Dimension, exponent: i32) {
        if exponent == 0 { return; }

        match dimension {
            Dimension::ElectricCharge    => self.insert_electric_charge(exponent),
            Dimension::Length            => self.insert_length(exponent),
            Dimension::LuminousIntensity => self.insert_luminous_intensity(exponent),
            Dimension::Mass              => self.insert_mass(exponent),
            Dimension::PlaneAngle        => self.insert_plane_angle(exponent),
            Dimension::Temperature       => self.insert_temperature(exponent),
            Dimension::Time              => self.insert_time(exponent),
        }
    }

    fn insert_electric_charge(&mut self, exponent: i32) {
        self.electric_charge = insert_exponent!(self, electric_charge, exponent);
    }

    fn insert_length(&mut self, exponent: i32) {
        self.length = insert_exponent!(self, length, exponent);
    }

    fn insert_luminous_intensity(&mut self, exponent: i32) {
        self.luminous_intensity = insert_exponent!(self, luminous_intensity, exponent);
    }

    fn insert_mass(&mut self, exponent: i32) {
        self.mass = insert_exponent!(self, mass, exponent);
    }

    fn insert_plane_angle(&mut self, exponent: i32) {
        self.plane_angle = insert_exponent!(self, plane_angle, exponent);
    }

    fn insert_temperature(&mut self, exponent: i32) {
        self.temperature = insert_exponent!(self, temperature, exponent);
    }

    fn insert_time(&mut self, exponent: i32) {
        self.time = insert_exponent!(self, time, exponent);
    }

    pub fn is_empty(&self) -> bool {
        self.electric_charge.is_none()
            && self.length.is_none()
            && self.luminous_intensity.is_none()
            && self.mass.is_none()
            && self.plane_angle.is_none()
            && self.temperature.is_none()
            && self.time.is_none()
    }
}

// impl Default
impl Default for Composition {
    fn default() -> Self {
        Composition {
            electric_charge: None,
            length: None,
            luminous_intensity: None,
            mass: None,
            plane_angle: None,
            temperature: None,
            time: None,
        }
    }
}

// impl Display
impl fmt::Display for Composition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "");
        }

        let mut expressions = Vec::<String>::new();

        if let Some(value) = self.electric_charge {
            expressions.push(format!("Q{}", value));
        }

        if let Some(value) = self.length {
            expressions.push(format!("L{}", value));
        }

        if let Some(value) = self.luminous_intensity {
            expressions.push(format!("F{}", value));
        }

        if let Some(value) = self.mass {
            expressions.push(format!("M{}", value));
        }

        if let Some(value) = self.plane_angle {
            expressions.push(format!("A{}", value));
        }

        if let Some(value) = self.temperature {
            expressions.push(format!("C{}", value));
        }

        if let Some(value) = self.time {
            expressions.push(format!("T{}", value));
        }

        write!(f, "{}", expressions.join("."))
    }
}

// impl Mul
/// Used for combining two `Compositions`.
///
impl Mul for Composition {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_composition = Composition::default();

        new_composition.electric_charge = add_dimension!(self, electric_charge, rhs);
        new_composition.length = add_dimension!(self, length, rhs);
        new_composition.luminous_intensity = add_dimension!(self, luminous_intensity, rhs);
        new_composition.mass = add_dimension!(self, mass, rhs);
        new_composition.plane_angle = add_dimension!(self, plane_angle, rhs);
        new_composition.temperature = add_dimension!(self, temperature, rhs);
        new_composition.time = add_dimension!(self, time, rhs);

        new_composition
    }
}

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
impl Mul<i32> for Composition {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        let mut new_composition = Composition::default();

        mul_exponent!(self, electric_charge, rhs, new_composition);
        mul_exponent!(self, length, rhs, new_composition);
        mul_exponent!(self, luminous_intensity, rhs, new_composition);
        mul_exponent!(self, mass, rhs, new_composition);
        mul_exponent!(self, plane_angle, rhs, new_composition);
        mul_exponent!(self, temperature, rhs, new_composition);
        mul_exponent!(self, time, rhs, new_composition);

        new_composition
    }
}

/// Used internally for disallowing setting any of the dimensions' exponents to 0 (it should
/// be `None` in that case).
///
fn set_exponent(exponent: i32) -> Option<i32> {
    if exponent == 0 {
        None
    } else {
        Some(exponent)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Dimension;
    use super::Composition;

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
        assert_eq!(composition.to_string().as_str(), "M3");

        composition.insert(Dimension::Mass, 3);
        assert_eq!(composition.to_string().as_str(), "M6");

        composition.insert(Dimension::Mass, -6);
        assert_eq!(composition.to_string().as_str(), "");

        let mut composition = Composition::default();
        composition.insert(Dimension::Mass, -1);
        composition.insert(Dimension::Temperature, -2);
        composition.insert(Dimension::ElectricCharge, -3);
        composition.insert(Dimension::Time, -4);
        composition.insert(Dimension::Length, -5);
        composition.insert(Dimension::PlaneAngle, -6);
        composition.insert(Dimension::LuminousIntensity, -7);
        assert_eq!(
            composition.to_string().as_str(),
            "Q-3.L-5.F-7.M-1.A-6.C-2.T-4"
        );
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
    fn validate_mul_composition_lhs_empty() {
        let subject = Composition::default();
        let other = subject.clone();
        let product = subject * other;
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

        let other = subject.clone();
        let product = subject * other;
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

        let other = subject.clone();
        let product = subject * other;
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
