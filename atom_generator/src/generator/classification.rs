use rust_structs::{RustAtomList, RustClassificationList};

pub(super) static HBS_TEMPLATE: &'static str = include_str!("../templates/classification.rs.hbs");

/// Uses the associated handlebars template to generate the Rust code for the
/// `Classification` enum.
///
pub(super) fn generate_file_body(atom_list: &RustAtomList) -> String {
    let classification_list = RustClassificationList::from(atom_list);

    super::HANDLEBARS
        .render("classification", &classification_list)
        .unwrap()
}
