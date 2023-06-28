use crate::{reducible::Reducible, Atom, UcumUnit};

impl Reducible for Atom {
    fn reduce_value(&self, value: f64) -> f64 {
        self.definition().reduce_value(value)
    }

    fn calculate_magnitude(&self, value: f64) -> f64 {
        if self.is_special() {
            self.definition().calculate_magnitude(value)
        } else {
            1.0
        }
    }
}
