use reducible::Reducible;
use unit::Unit;

//-----------------------------------------------------------------------------
// impl Reducible
//-----------------------------------------------------------------------------
impl Reducible for Unit {
    fn reduce_value(&self, value: f64) -> f64 {
        self.terms.reduce_value(value)
    }

    /// Calculates `value` count of `self` in terms of `self`'s unit.
    ///
    fn calculate_magnitude(&self, value: f64) -> f64 {
        self.terms.calculate_magnitude(value)
    }
}
