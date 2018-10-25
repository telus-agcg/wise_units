//! This module is responsible for generating the Rust code that built into the
//! main `wise_units` crate. Each part of it expects some data that was
//! extracted from the UCUM TOML file (and any custom units defined by 3rd
//! party libraries) and then generates all code that defines the
//! parsing, interpreting, and types of atoms (units) that can be used by a
//! consuming library or application.
//!

pub(self) mod atom;
pub(self) mod classification;
pub(self) mod handlebars;
pub(self) mod mapper;
pub(self) mod property;
pub(self) mod symbol_grammar;
pub(self) mod symbol_parser;

pub(self) use self::handlebars::HANDLEBARS;

use crate::rust_structs::RustAtomList;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Main function for generating all files that need to be generated from a
/// `RustAtomList`.
///
pub(crate) fn generate_files(rust_atom_list: &RustAtomList) {
    generate_classification_file(rust_atom_list);
    generate_property_file(rust_atom_list);
    generate_atom_file(rust_atom_list);
    generate_symbol_grammar_file(rust_atom_list);
    generate_symbol_parser_file();
    generate_mapper_file(rust_atom_list);
}

fn generate_classification_file(rust_atom_list: &RustAtomList) {
    let file_body = self::classification::generate_file_body(rust_atom_list);
    let file_path = build_file_path("classification.rs");
    write_project_file(&file_path, &file_body);
}

fn generate_property_file(rust_atom_list: &RustAtomList) {
    let file_body = self::property::generate_file_body(rust_atom_list);
    let file_path = build_file_path("property.rs");
    write_project_file(&file_path, &file_body);
}

fn generate_atom_file(rust_atom_list: &RustAtomList) {
    let file_body = self::atom::generate_file_body(rust_atom_list);
    let file_path = build_file_path("atom.rs");
    write_project_file(&file_path, &file_body);
}

fn generate_symbol_grammar_file(rust_atom_list: &RustAtomList) {
    let file_body = self::symbol_grammar::generate_file_body(rust_atom_list);
    let file_path = build_file_path("symbol.pest");
    write_project_file(&file_path, &file_body);
}

fn generate_symbol_parser_file() {
    let grammar_file_path = build_file_path("symbol.pest");
    let file_body =
        self::symbol_parser::generate_file_body(grammar_file_path.display().to_string());
    let file_path = build_file_path("symbol_parser.rs");
    write_project_file(&file_path, &file_body);
}

fn generate_mapper_file(rust_atom_list: &RustAtomList) {
    let file_body = self::mapper::generate_file_body(rust_atom_list);
    let file_path = build_file_path("mapper.rs");
    write_project_file(&file_path, &file_body);
}

fn build_file_path(file_name: &str) -> PathBuf {
    let dest_dir = env::var("OUT_DIR").unwrap();

    Path::new(&dest_dir).join(file_name)
}

fn write_project_file(file_path: &PathBuf, file_body: &str) {
    let mut f = File::create(file_path).unwrap();

    f.write_all(file_body.as_bytes())
        .expect("Problem writing the file")
}
