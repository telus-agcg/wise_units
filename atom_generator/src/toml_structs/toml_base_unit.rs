use toml_structs::TomlAtom;

#[derive(Debug, Deserialize)]
pub struct TomlBaseUnit {
    #[serde(rename = "Code")]
    pub primary_code: String,

    #[serde(rename = "CODE")]
    pub secondary_code: String,

    pub dim: String,

    pub names: Vec<String>,

    #[serde(rename = "printSymbol")]
    pub print_symbol: String,

    pub property: String,
}

impl<'a> TomlAtom for &'a TomlBaseUnit {
    fn primary_code(&self) -> String {
        self.primary_code.clone()
    }

    fn names(&self) -> Vec<String> {
        self.names.clone()
    }
}
