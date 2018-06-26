pub(super) static HBS_TEMPLATE: &'static str = include_str!("../templates/symbol_parser.rs.hbs");

#[derive(Serialize)]
struct RustSymbolParser {
    grammar_file_path: String,
}

/// Uses the associated handlebars template to generate the Rust code for the
/// `SymbolParser` struct.
///
pub(super) fn generate_file_body(grammar_file_path: String) -> String {
    let rust_symbol_parser = RustSymbolParser { grammar_file_path };

    super::HANDLEBARS
        .render("symbol_parser", &rust_symbol_parser)
        .unwrap()
}
