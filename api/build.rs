#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct AtomList {
    atoms: Vec<Atom>,
}

#[derive(Debug, Deserialize)]
struct Atom {
    primary_code: String,
    variant: String,
    definition: Option<Definition>,
}

#[derive(Debug, Deserialize)]
struct Definition {
    value: f64,
    unit: String,
}

fn main() {
    let contents = read_atoms_toml();

    let atoms: AtomList = toml::from_str(&contents).expect("unable to deserialize Atoms.toml");

    write_atoms_rs(atoms);
}

fn read_atoms_toml() -> String {
    let src_dir = env::current_dir().unwrap();
    let src_path = Path::new(&src_dir).join("Atoms.toml");

    let mut atoms_file = File::open(src_path).expect("file not found");

    let mut contents = String::new();

    atoms_file
        .read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

fn write_atoms_rs(atom_list: AtomList) {
    let dest_dir = env::current_dir().unwrap();
    let dest_path = Path::new(&dest_dir).join("atoms.rs");
    let mut f = File::create(&dest_path).unwrap();

    let atom_string = format!("{:?}", atom_list);
    f.write_all(atom_string.as_bytes()).unwrap();
}
