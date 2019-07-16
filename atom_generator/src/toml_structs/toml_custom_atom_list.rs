use super::{TomlBaseUnit, TomlUnit};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub(crate) struct TomlCustomAtomList {
    #[serde(rename = "base-unit")]
    pub(crate) base_units: Option<Vec<TomlBaseUnit>>,

    #[serde(rename = "unit")]
    pub(crate) units: Option<Vec<TomlUnit>>,
}

impl FromStr for TomlCustomAtomList {
    type Err = ::toml::de::Error;

    fn from_str(toml: &str) -> Result<Self, Self::Err> {
        ::toml::from_str(toml)
    }
}
