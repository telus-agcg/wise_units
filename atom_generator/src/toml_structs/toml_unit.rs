use super::{TomlAtom, TomlDefinition};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TomlUnit {
    #[serde(rename = "Code")]
    pub primary_code: String,

    #[serde(rename = "CODE")]
    pub secondary_code: Option<String>,

    #[serde(
        deserialize_with = "super::de_bool",
        default = "super::de_bool_default"
    )]
    pub is_metric: bool,

    #[serde(
        deserialize_with = "super::de_bool",
        default = "super::de_bool_default"
    )]
    pub is_arbitrary: bool,

    #[serde(
        deserialize_with = "super::de_bool",
        default = "super::de_bool_default"
    )]
    pub is_special: bool,

    #[serde(rename = "class")]
    pub classification: String,

    pub names: Vec<String>,
    pub print_symbol: Option<String>,
    pub property: String,

    #[serde(rename = "value")]
    pub definition: TomlDefinition,
}

impl<'a> TomlAtom for &'a TomlUnit {
    fn primary_code(&self) -> String {
        self.primary_code.clone()
    }

    fn names(&self) -> Vec<String> {
        self.names.clone()
    }
}
