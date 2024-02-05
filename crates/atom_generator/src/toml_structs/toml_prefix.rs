#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub(crate) struct TomlPrefix {
    #[serde(rename = "Code")]
    pub(crate) primary_code: String,

    #[serde(rename = "printSymbol")]
    pub(crate) print_symbol: String,

    pub(crate) names: Vec<String>,

    #[serde(rename = "CODE")]
    pub(crate) secondary_code: String,

    pub(crate) value: f64,
}
