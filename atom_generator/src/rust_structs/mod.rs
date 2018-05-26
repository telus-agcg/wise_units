pub mod pest_symbol_list;
pub mod rust_classification_list;
pub mod rust_function_set;
pub mod rust_mapper_list;
pub mod rust_property_list;
pub mod rust_unit;

pub use self::pest_symbol_list::PestSymbolList;
pub use self::rust_classification_list::RustClassificationList;
pub use self::rust_function_set::RustFunctionSet;
pub use self::rust_mapper_list::RustMapperList;
pub use self::rust_property_list::RustPropertyList;
pub use self::rust_unit::RustUnit;

use heck::SnakeCase;

#[derive(Debug, Serialize)]
pub struct RustAtomList {
    pub atoms: Vec<RustUnit>,
}

pub fn build_pest_rule_name(prefix: &str, symbol: &str) -> String {
    let symbol = symbol.to_snake_case();

    format!("{}_{}", prefix, symbol)
}
