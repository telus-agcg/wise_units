use super::RustAtomList;
use std::collections::HashMap;

type AtomTypeName = String;

#[derive(Debug, Serialize)]
pub struct RustMapperList {
    pub mappings: HashMap<AtomTypeName, RustMappingValue>,
}

impl<'a> From<&'a RustAtomList> for RustMapperList {
    fn from(atom_list: &'a RustAtomList) -> RustMapperList {
        let mut mappings: HashMap<AtomTypeName, RustMappingValue> = HashMap::new();

        for rust_unit in &atom_list.atoms {
            let key = rust_unit.type_name.clone();
            let primary_rule_name = super::build_pest_rule_name("pri", &key);

            let secondary_rule_name = match rust_unit.secondary_code {
                Some(ref _sc) => Some(super::build_pest_rule_name("sec", &key)),
                None => None,
            };

            let mapping_value = RustMappingValue {
                primary_rule_name,
                secondary_rule_name,
            };

            mappings.entry(key).or_insert(mapping_value);
        }

        RustMapperList { mappings }
    }
}

#[derive(Debug, Serialize)]
pub struct RustMappingValue {
    pub primary_rule_name: String,
    pub secondary_rule_name: Option<String>,
}
