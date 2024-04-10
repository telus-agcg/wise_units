/// This is a struct to allow for representing functions that special units use
/// for converting to and from their base unit.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct FunctionSet<V> {
    // AKA "calculate_magnitude"
    pub(crate) convert_from: fn(value: V) -> V,

    // AKA "reduce_value"
    pub(crate) convert_to: fn(value: V) -> V,
}
