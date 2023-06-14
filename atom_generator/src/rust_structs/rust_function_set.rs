#[derive(Debug, serde::Serialize)]
pub(crate) struct RustFunctionSet {
    // AKA "calculate_magnitude"
    pub(crate) convert_from: String,

    // AKA "reduce_value"
    pub(crate) convert_to: String,
}
