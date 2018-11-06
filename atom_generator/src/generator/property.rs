use crate::rust_structs::{RustAtomList, RustPropertyList};

pub(super) static HBS_TEMPLATE: &'static str = include_str!("../templates/property.rs.hbs");

/// Uses the associated handlebars template to generate the Rust code for the
/// `Property` enum.
///
pub(super) fn generate_file_body(atom_list: &RustAtomList) -> String {
    let property_list = RustPropertyList::from(atom_list);

    super::HANDLEBARS
        .render("property", &property_list)
        .unwrap()
}
