pub mod toml_atom;
pub mod toml_atom_list;
pub mod toml_base_unit;
pub mod toml_definition;
pub mod toml_prefix;
pub mod toml_unit;

pub use self::toml_atom::TomlAtom;
pub use self::toml_atom_list::TomlAtomList;
pub use self::toml_base_unit::TomlBaseUnit;
pub use self::toml_definition::TomlDefinition;
pub use self::toml_prefix::TomlPrefix;
pub use self::toml_unit::TomlUnit;

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
