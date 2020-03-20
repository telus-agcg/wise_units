use crate::rust_structs::RustAtomList;

pub(super) static HBS_TEMPLATE: &str = include_str!("../templates/atom.rs.hbs");

/// Uses the associated handlebars template to generate the Rust code for the
/// `Atom` enum.
///
pub(super) fn generate_file_body(atom_list: &RustAtomList) -> String {
    super::HANDLEBARS.render("atom", atom_list).unwrap()
}
