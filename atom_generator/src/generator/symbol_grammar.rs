use rust_structs::{PestSymbolList, RustAtomList};

pub(super) static HBS_TEMPLATE: &'static str = include_str!("../templates/symbol.pest.hbs");

/// Uses the associated handlebars template to generate the Rust code for the
/// `symbol.pest` pest grammar.
///
pub(super) fn generate_file_body(atom_list: &RustAtomList) -> String {
    let symbol_list = PestSymbolList::from(atom_list);

    super::HANDLEBARS
        .render("symbol_grammar", &symbol_list)
        .unwrap()
}
