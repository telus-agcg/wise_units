#![deny(unused_extern_crates)]
#![warn(
    clippy::all,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    missing_copy_implementations,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#![allow(clippy::redundant_pub_crate)]

#[macro_use(handlebars_helper)]
extern crate handlebars;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod from_toml;
mod generator;
mod rust_structs;
mod toml_structs;

fn main() {
    build_ucum_atoms();
}

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
