use crate::rust_structs::RustAtomList;
use crate::toml_structs::TomlAtomList;
use std::str::FromStr;

static ATOMS_FILE: &str = include_str!("../../Atoms.toml");

/// Reads the `Atoms.toml` file (containing UCUM unit definitions) that's part
/// of the repo and transforms that into a `RustAtomList`.
///
pub(crate) fn build_rust_atom_list() -> RustAtomList {
    let toml_atoms =
        TomlAtomList::from_str(ATOMS_FILE).expect("unable to deserialize Toml to TomlAtomList");

    transform(&toml_atoms)
}

/// Transforms a `TomlAtomlList` to a `RustAtomList`.
///
fn transform(toml_atom_list: &TomlAtomList) -> RustAtomList {
    let mut atoms = super::transform_base_units(&toml_atom_list.base_units);
    atoms.append(&mut super::transform_units(&toml_atom_list.units));

    RustAtomList { atoms }
}
