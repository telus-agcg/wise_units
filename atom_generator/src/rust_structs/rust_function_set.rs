#[derive(Debug, Serialize)]
pub(crate) struct RustFunctionSet {
    // AKA "calculate_magnitude"
    pub convert_from: String,

    // AKA "reduce_value"
    pub convert_to: String,
}
