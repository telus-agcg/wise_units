pub(crate) mod pest_symbol_list;
pub(crate) mod rust_atom;
pub(crate) mod rust_classification_list;
pub(crate) mod rust_function_set;
pub(crate) mod rust_mapper_list;
pub(crate) mod rust_property_list;

pub(crate) use self::{
    pest_symbol_list::PestSymbolList, rust_atom::RustAtom,
    rust_classification_list::RustClassificationList, rust_function_set::RustFunctionSet,
    rust_mapper_list::RustMapperList, rust_property_list::RustPropertyList,
};

use heck::ToSnakeCase;

#[derive(Debug, serde::Serialize)]
pub(crate) struct RustAtomList {
    pub(crate) atoms: Vec<RustAtom>,
}

pub(self) fn build_pest_rule_name(prefix: &str, symbol: &str) -> String {
    let symbol = symbol.to_snake_case();

    format!("{prefix}_{symbol}")
}
