use super::RustAtomList;
use std::collections::HashMap;

type AtomTypeName = String;

#[derive(Debug, Serialize)]
pub(crate) struct RustMapperList {
    pub(crate) mappings: HashMap<AtomTypeName, RustMappingValue>,
}

impl<'a> From<&'a RustAtomList> for RustMapperList {
    fn from(atom_list: &'a RustAtomList) -> Self {
        let mut mappings: HashMap<AtomTypeName, RustMappingValue> = HashMap::new();

        for rust_unit in &atom_list.atoms {
            let key = rust_unit.type_name.clone();
            let primary_rule_name = super::build_pest_rule_name("pri", &key);

            let secondary_rule_name = rust_unit
                .secondary_code
                .as_ref()
                .map(|_sc| super::build_pest_rule_name("sec", &key));

            let mapping_value = RustMappingValue {
                primary_rule_name,
                secondary_rule_name,
            };

            let _ = mappings.entry(key).or_insert(mapping_value);
        }

        Self { mappings }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct RustMappingValue {
    pub(crate) primary_rule_name: String,
    pub(crate) secondary_rule_name: Option<String>,
}
