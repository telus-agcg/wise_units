use super::{TomlBaseUnit, TomlUnit};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub(crate) struct TomlCustomAtomList {
    #[serde(rename = "base-unit")]
    pub base_units: Option<Vec<TomlBaseUnit>>,

    #[serde(rename = "unit")]
    pub units: Option<Vec<TomlUnit>>,
}

impl FromStr for TomlCustomAtomList {
    type Err = ::toml::de::Error;

    fn from_str(toml: &str) -> Result<Self, Self::Err> {
        let toml_atom_list: TomlCustomAtomList = ::toml::from_str(&toml)?;

        Ok(toml_atom_list)
    }
}
