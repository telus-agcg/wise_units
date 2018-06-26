pub(crate) mod toml_atom;
pub(crate) mod toml_atom_list;
pub(crate) mod toml_base_unit;
pub(crate) mod toml_custom_atom_list;
pub(crate) mod toml_definition;
pub(crate) mod toml_prefix;
pub(crate) mod toml_unit;

pub(crate) use self::toml_atom::TomlAtom;
pub(crate) use self::toml_atom_list::TomlAtomList;
pub(crate) use self::toml_base_unit::TomlBaseUnit;
pub(crate) use self::toml_custom_atom_list::TomlCustomAtomList;
pub(crate) use self::toml_definition::TomlDefinition;
pub(crate) use self::toml_prefix::TomlPrefix;
pub(crate) use self::toml_unit::TomlUnit;

use serde::de::{Deserialize, Deserializer};

fn de_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(string) => {
            let result = match string.as_str() {
                "yes" => true,
                "no" => false,
                _ => false,
            };

            Ok(result)
        }
        Err(_) => Ok(false),
    }
}

fn de_bool_default() -> bool {
    false
}
