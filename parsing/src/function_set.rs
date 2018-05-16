#[derive(Debug)]
pub struct FunctionSet {
    // AKA "calculate_magnitude"
    pub convert_from: fn(value: f64) -> f64,

    // AKA "calculate_scalar"
    pub convert_to: fn(value: f64) -> f64,
}
