/// This is a struct to allow for representing functions that special units use
/// for converting to and from their base unit.
///
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FunctionSet {
    // AKA "calculate_magnitude"
    pub(crate) convert_from: fn(value: f64) -> f64,

    // AKA "reduce_value"
    pub(crate) convert_to: fn(value: f64) -> f64,
}

// TODO: Add public functions to calculate
