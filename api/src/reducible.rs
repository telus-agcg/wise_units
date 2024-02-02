pub(crate) trait Reducible {
    /// Calculates `value` count of `self` in terms of `self`'s base-unit.
    ///
    // TODO: Extract to v2 trait.
    fn reduce_value(&self, value: f64) -> f64;

    fn calculate_magnitude(&self, value: f64) -> f64;
}
