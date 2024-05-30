use std::collections::HashSet;

use super::RustAtomList;

#[derive(Debug, serde::Serialize)]
pub(crate) struct RustClassificationList {
    pub(crate) type_names: Vec<String>,
}

impl<'a> From<&'a RustAtomList> for RustClassificationList {
    fn from(atom_list: &'a RustAtomList) -> Self {
        Self {
            type_names: unique_classification_names(atom_list),
        }
    }
}

fn unique_classification_names(atom_list: &RustAtomList) -> Vec<String> {
    let type_names: HashSet<String> = atom_list
        .atoms
        .iter()
        .map(|rust_unit| rust_unit.classification.clone())
        .collect();

    let mut type_names = type_names.into_iter().collect::<Vec<_>>();
    type_names.sort();

    type_names
}
