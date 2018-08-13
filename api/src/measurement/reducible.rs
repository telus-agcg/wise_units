use measurement::Measurement;
use reducible::Reducible;
use ucum_unit::UcumUnit;

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
