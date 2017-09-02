use composition::Composition;
use unit::Unit;
use interpreter::Interpreter;

/// Represents the UCUM "definition" of a Unit (since units are defined in terms
/// of some scalar and another, more basic unit). For example a "degree" is:
///
/// ```
/// use wise_units::definition::Definition;
///
/// Definition::new(2.0, "[pi].rad/360");
/// ```
///
/// In that case the `Degree` struct will have some other functions/data to
/// describe it, but this `definition` is how the unit is compared to other
/// units.
///
#[derive(Debug)]
pub struct Definition {
    pub value: f64,
    pub unit: Unit,
}

impl Definition {
    pub fn new<'a>(value: f64, expression: &'a str) -> Self {
        let mut interpreter = Interpreter;
        let su = interpreter.interpret(expression);

        Definition {
            value: value,
            unit: su,
        }
    }

    pub fn is_special(&self) -> bool {
        self.unit.is_special()
    }

    pub fn scalar(&self) -> f64 {
        // let unit_comp = Composition::new_unity();

        // // Don't call (possibly) recursively if the Term is TheUnity (since that
        // // is the base of all units).
        // if self.unit.composition() == unit_comp {
        //     self.value
        // } else {
        //     self.value * self.unit.scalar()
        // }
        if self.is_special() {
            let magnitude = self.value;
            self.unit.calculate_scalar(magnitude)
        } else {
            self.value * self.unit.scalar()
        }
    }

    pub fn magnitude(&self) -> f64 {
        // let unit_comp = Composition::new_unity();

        // Don't call (possibly) recursively if the Term is TheUnity (since that
        // is the base of all units).
        // if self.unit.composition() == unit_comp {
        //     self.value
        // } else {
        //     self.value * self.unit.magnitude()
        // }
        if self.is_special() {
            let scalar = self.scalar();
            self.unit.calculate_magnitude(scalar)
        } else {
            self.value * self.unit.magnitude()
        }
    }

    pub fn calculate_scalar(&self, magnitude: f64) -> f64 {
        let unit_comp = Composition::new_unity();

        if self.unit.composition() == unit_comp {
            self.value
        } else {
            self.unit.calculate_scalar(magnitude)
        }
    }

    pub fn calculate_magnitude(&self, scalar: f64) -> f64 {
        let unit_comp = Composition::new_unity();

        if self.unit.composition() == unit_comp {
            self.value
        } else {
            self.unit.calculate_magnitude(scalar)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_scalar() {
        // let d = Definition::new(1.0, "10*-2");
        // assert_eq!(d.scalar(), 0.01);
        let d = Definition::new(1.0, "m");
        assert_eq!(d.scalar(), 1.0);

        let d = Definition::new(1.0, "cm");
        assert_eq!(d.scalar(), 0.01);
    }
}
