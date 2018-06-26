use super::RustAtomList;
use heck::CamelCase;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

type PropertyTypeName = String;

#[derive(Debug, Serialize)]
pub(crate) struct RustPropertyList {
    pub properties: HashMap<PropertyTypeName, RustProperty>,
}

#[derive(Debug, Serialize)]
pub(crate) struct RustProperty {
    pub atom_type_names: Vec<String>,
    pub description: String,
}

impl<'a> From<&'a RustAtomList> for RustPropertyList {
    fn from(atom_list: &'a RustAtomList) -> RustPropertyList {
        let mut properties: HashMap<PropertyTypeName, RustProperty> = HashMap::new();

        for atom in &atom_list.atoms {
            let key = atom.property.to_camel_case();

            match properties.entry(key) {
                Entry::Vacant(entry) => {
                    let property = RustProperty {
                        atom_type_names: vec![atom.type_name.clone()],
                        description: atom.property.clone(),
                    };

                    entry.insert(property);
                }
                Entry::Occupied(mut entry) => {
                    let new_atom_type_name = atom.type_name.clone();
                    let mut e = entry.get_mut();

                    e.atom_type_names.push(new_atom_type_name);
                }
            }
        }

        RustPropertyList { properties }
    }
}
