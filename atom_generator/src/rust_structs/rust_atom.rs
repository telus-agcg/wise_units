#[derive(Debug, serde::Serialize)]
pub(crate) struct RustAtom {
    pub(crate) type_name: String,
    pub(crate) classification: String,
    pub(crate) dim: Option<String>,
    pub(crate) definition_signature: String,
    pub(crate) primary_code: String,
    pub(crate) print_symbol: Option<String>,
    pub(crate) property: String,
    pub(crate) names: Vec<String>,
    pub(crate) secondary_code: Option<String>,
    pub(crate) is_arbitrary: bool,
    pub(crate) is_metric: bool,
    pub(crate) is_special: bool,
}
