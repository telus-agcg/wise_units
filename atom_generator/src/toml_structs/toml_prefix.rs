#[derive(Debug, Deserialize)]
pub struct TomlPrefix {
    #[serde(rename = "Code")]
    pub primary_code: String,

    #[serde(rename = "printSymbol")]
    pub print_symbol: String,

    pub names: Vec<String>,

    #[serde(rename = "CODE")]
    pub secondary_code: String,

    pub value: f64,
}
