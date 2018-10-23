use super::{TomlAtom, TomlDefinition};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TomlUnit {
    #[serde(rename = "Code")]
    pub(crate) primary_code: String,

    #[serde(rename = "CODE")]
    pub(crate) secondary_code: Option<String>,

    #[serde(
        deserialize_with = "super::de_bool",
        default = "super::de_bool_default"
    )]
    pub(crate) is_metric: bool,

    #[serde(
        deserialize_with = "super::de_bool",
        default = "super::de_bool_default"
    )]
    pub(crate) is_arbitrary: bool,

    #[serde(
        deserialize_with = "super::de_bool",
        default = "super::de_bool_default"
    )]
    pub(crate) is_special: bool,

    #[serde(rename = "class")]
    pub(crate) classification: String,

    pub(crate) names: Vec<String>,
    pub(crate) print_symbol: Option<String>,
    pub(crate) property: String,

    #[serde(rename = "value")]
    pub(crate) definition: TomlDefinition,
}

impl<'a> TomlAtom for &'a TomlUnit {
    fn primary_code(&self) -> String {
        self.primary_code.clone()
    }

    fn names(&self) -> Vec<String> {
        self.names.clone()
    }
}
