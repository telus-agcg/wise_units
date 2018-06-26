use super::{TomlBaseUnit, TomlPrefix, TomlUnit};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub(crate) struct TomlAtomList {
    #[serde(rename = "prefix")]
    pub prefixes: Vec<TomlPrefix>,

    #[serde(rename = "base-unit")]
    pub base_units: Vec<TomlBaseUnit>,

    #[serde(rename = "unit")]
    pub units: Vec<TomlUnit>,
}

impl FromStr for TomlAtomList {
    type Err = ::toml::de::Error;

    fn from_str(toml: &str) -> Result<Self, Self::Err> {
        let toml_atom_list: TomlAtomList = ::toml::from_str(&toml)?;

        Ok(toml_atom_list)
    }
}
