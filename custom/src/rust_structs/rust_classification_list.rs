use super::RustAtomList;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, Serialize)]
pub struct RustClassificationList {
    pub type_names: Vec<String>,
}

impl<'a> From<&'a RustAtomList> for RustClassificationList {
    fn from(atom_list: &'a RustAtomList) -> RustClassificationList {
        let type_names = unique_classification_names(atom_list);

        RustClassificationList { type_names }
    }
}

fn unique_classification_names(atom_list: &RustAtomList) -> Vec<String> {
    let type_names: HashSet<String> = atom_list
        .atoms
        .iter()
        .map(|rust_unit| rust_unit.classification.clone())
        .collect();

    let mut type_names = Vec::from_iter(type_names.into_iter());
    type_names.sort();

    type_names
}
