use toml_structs::TomlAtom;

#[derive(Debug, Deserialize)]
pub(crate) struct TomlBaseUnit {
    #[serde(rename = "Code")]
    pub(crate) primary_code: String,

    #[serde(rename = "CODE")]
    pub(crate) secondary_code: String,

    pub(crate) dim: String,

    pub(crate) names: Vec<String>,

    #[serde(rename = "printSymbol")]
    pub(crate) print_symbol: String,

    pub(crate) property: String,
}

impl<'a> TomlAtom for &'a TomlBaseUnit {
    fn primary_code(&self) -> String {
        self.primary_code.clone()
    }

    fn names(&self) -> Vec<String> {
        self.names.clone()
    }
}
