pub(crate) mod pest_symbol_list;
pub(crate) mod rust_atom;
pub(crate) mod rust_classification_list;
pub(crate) mod rust_function_set;
pub(crate) mod rust_mapper_list;
pub(crate) mod rust_property_list;

pub(crate) use self::pest_symbol_list::PestSymbolList;
pub(crate) use self::rust_atom::RustAtom;
pub(crate) use self::rust_classification_list::RustClassificationList;
pub(crate) use self::rust_function_set::RustFunctionSet;
pub(crate) use self::rust_mapper_list::RustMapperList;
pub(crate) use self::rust_property_list::RustPropertyList;

use heck::SnakeCase;

#[derive(Debug, Serialize)]
pub(crate) struct RustAtomList {
    pub(crate) atoms: Vec<RustAtom>,
}

pub(self) fn build_pest_rule_name(prefix: &str, symbol: &str) -> String {
    let symbol = symbol.to_snake_case();

    format!("{}_{}", prefix, symbol)
}
