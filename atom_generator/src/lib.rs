#![warn(missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_extern_crates)]
#![forbid(unused_imports)]

#[macro_use(handlebars_helper)]
extern crate handlebars;
extern crate heck;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod from_toml;
mod generator;
mod rust_structs;
mod toml_structs;

/// Used by standard `wise_units` to define only UCUM atoms/units as part of
/// the library. If you're not defining custom units, there's no reason to call
/// this (unless you're `wise_units`).
///
pub fn build_ucum_atoms() {
    let rust_atom_list = from_toml::atoms::build_rust_atom_list();

    generator::generate_files(&rust_atom_list);
}

/// Use this to read your project-root-level `CustomAtoms.toml` file to
/// generate code from those and add them to the list of units to be used in
/// your project.
///
pub fn build_with_custom_atoms() {
    let rust_atom_list = from_toml::custom_atoms::build_rust_atom_list();

    generator::generate_files(&rust_atom_list);
}
