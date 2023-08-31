use crate::reducible::Reducible;

use super::Definition;

impl Reducible for Definition {
    fn reduce_value(&self, other_value: f64) -> f64 {
        self.function_set.as_ref().map_or_else(
            || {
                if self.is_unity() {
                    self.value
                } else {
                    self.value * self.terms.reduce_value(other_value)
                }
            },
            |f| (f.convert_to)(other_value),
        )
    }

    fn calculate_magnitude(&self, other_value: f64) -> f64 {
        self.function_set.as_ref().map_or_else(
            || {
                if self.is_unity() {
                    self.value
                } else {
                    self.value * self.terms.calculate_magnitude(other_value)
                }
            },
            |f| (f.convert_from)(other_value),
        )
    }
}
