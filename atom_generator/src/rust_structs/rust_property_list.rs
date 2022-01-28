use super::RustAtomList;
use heck::ToUpperCamelCase;
use std::collections::HashMap;

type PropertyTypeName = String;

#[derive(Debug, Serialize)]
pub(crate) struct RustPropertyList {
    pub(crate) properties: HashMap<PropertyTypeName, RustProperty>,
}

#[derive(Debug, Serialize)]
pub(crate) struct RustProperty {
    pub(crate) atom_type_names: Vec<String>,
    pub(crate) description: String,
}

impl<'a> From<&'a RustAtomList> for RustPropertyList {
    fn from(atom_list: &'a RustAtomList) -> Self {
        let mut properties: HashMap<PropertyTypeName, RustProperty> = HashMap::new();

        for atom in &atom_list.atoms {
            let key = atom.property.to_upper_camel_case();

            let _ = properties
                .entry(key)
                .and_modify(|entry| entry.atom_type_names.push(atom.type_name.clone()))
                .or_insert_with(|| RustProperty {
                    atom_type_names: vec![atom.type_name.clone()],
                    description: atom.property.clone(),
                });
        }

        Self { properties }
    }
}
