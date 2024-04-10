pub(crate) trait Reducible<V> {
    /// Calculates `value` count of `self` in terms of `self`'s base-unit.
    ///
    fn reduce_value(&self, value: V) -> V;

    fn calculate_magnitude(&self, value: V) -> V;
}
