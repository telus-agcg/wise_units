use super::{TomlBaseUnit, TomlPrefix, TomlUnit};

#[derive(Debug, Deserialize)]
pub struct TomlAtomList {
    #[serde(rename = "prefix")]
    pub prefixes: Vec<TomlPrefix>,

    #[serde(rename = "base-unit")]
    pub base_units: Vec<TomlBaseUnit>,

    #[serde(rename = "unit")]
    pub units: Vec<TomlUnit>,
}
