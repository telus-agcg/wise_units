use composition::Composition;
use interpreter::Interpreter;
use measurable::Measurable;
use unit::Unit;

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

    pub fn calculate_scalar(&self, other_value: f64) -> f64 {
        let unit_comp = Composition::new_unity();

        if self.unit.composition() == unit_comp {
            self.value
        } else {
            self.value * self.unit.calculate_scalar(other_value)
        }
    }

    pub fn calculate_magnitude(&self, other_value: f64) -> f64 {
        let unit_comp = Composition::new_unity();

        if self.unit.composition() == unit_comp {
            self.value
        } else {
            self.unit.calculate_magnitude(other_value)
        }
    }
}

impl Measurable for Definition {
    fn get_unit(&self) -> &Unit {
        &self.unit
    }

    fn get_value(&self) -> f64 {
        self.value
    }

    fn scalar(&self) -> f64 {
        if self.is_special() {
            let magnitude = self.value;
            self.unit.calculate_scalar(magnitude)
        } else {
            self.value * self.unit.calculate_scalar(1.0)
        }
    }

    fn magnitude(&self) -> f64 {
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
            self.value * self.unit.calculate_magnitude(1.0)
        }
    }
}
