#[derive(Debug, Serialize)]
pub struct RustUnit {
    pub type_name: String,
    pub classification: String,
    pub dim: Option<String>,
    pub definition_signature: String,
    pub primary_code: String,
    pub print_symbol: Option<String>,
    pub property: String,
    pub names: Vec<String>,
    pub secondary_code: Option<String>,
    pub is_arbitrary: bool,
    pub is_metric: bool,
    pub is_special: bool,
}
