#[derive(Debug, Serialize)]
pub(crate) struct RustFunctionSet {
    // AKA "calculate_magnitude"
    pub convert_from: String,

    // AKA "calculate_scalar"
    pub convert_to: String,
}
