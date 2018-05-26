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

mod generator;
mod rust_structs;
mod toml_structs;

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use toml_structs::TomlAtomList;

pub fn build() {
    let contents = read_atoms_toml();
    let toml_atoms = convert_toml_to_struct(&contents);
    let rust_atoms = generator::build_rust_atoms(&toml_atoms);

    let classification_file_body = generator::build_classification_file_body(&rust_atoms);
    write_project_file("src/classification.rs", &classification_file_body);

    let property_file_body = generator::build_property_file_body(&rust_atoms);
    write_project_file("src/property.rs", &property_file_body);

    let atom_file_body = generator::build_atom_file_body(&rust_atoms);
    write_project_file("src/atom.rs", &atom_file_body);

    let symbol_file_body = generator::build_symbol_file_body(&rust_atoms);
    write_project_file("src/symbols/symbol.pest", &symbol_file_body);

    let mapper_file_body = generator::build_mapper_file_body(&rust_atoms);
    write_project_file("src/symbols/mapper.rs", &mapper_file_body);
}

fn read_atoms_toml() -> String {
    let src_dir = env::current_dir().unwrap();
    debug!("[read_atoms_toml] src_dir: {:?}", &src_dir);

    let src_path = Path::new(&src_dir).join("../atom_generator/Atoms.toml");

    debug!("[read_atoms_toml] src_path: {:?}", src_path.to_str());

    let mut atoms_file = File::open(src_path).expect("file not found");

    let mut read_contents = String::new();

    atoms_file
        .read_to_string(&mut read_contents)
        .expect("something went wrong reading the file");
    debug!("[read_atoms_toml] Read Atoms.toml");

    read_contents
}

fn convert_toml_to_struct(toml: &str) -> TomlAtomList {
    let atoms: TomlAtomList = toml::from_str(&toml).expect("unable to deserialize Atoms.toml");

    atoms
}

fn write_project_file(file_name: &str, file_body: &str) {
    let dest_dir = env::current_dir().unwrap();
    let dest_path = Path::new(&dest_dir).join(file_name);
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(file_body.as_bytes())
        .expect("Problem writing the file")
}
