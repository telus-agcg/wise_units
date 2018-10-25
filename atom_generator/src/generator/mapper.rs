use crate::rust_structs::{RustAtomList, RustMapperList};

pub(super) static HBS_TEMPLATE: &'static str = include_str!("../templates/mapper.rs.hbs");

/// Uses the associated handlebars template to generate the Rust code for the
/// `mapper` module.
///
pub(super) fn generate_file_body(atom_list: &RustAtomList) -> String {
    let mapper_list = RustMapperList::from(atom_list);

    super::HANDLEBARS.render("mapper", &mapper_list).unwrap()
}
