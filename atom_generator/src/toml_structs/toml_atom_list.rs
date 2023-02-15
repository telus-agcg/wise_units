use super::{TomlBaseUnit, TomlUnit};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub(crate) struct TomlAtomList {
    //#[serde(rename = "prefix")]
    //pub(crate) prefixes: Vec<TomlPrefix>,

    #[serde(rename = "base-unit")]
    pub(crate) base_units: Vec<TomlBaseUnit>,

    #[serde(rename = "unit")]
    pub(crate) units: Vec<TomlUnit>,
}

impl FromStr for TomlAtomList {
    type Err = ::toml::de::Error;

    fn from_str(toml: &str) -> Result<Self, Self::Err> {
        ::toml::from_str(toml)
    }
}
