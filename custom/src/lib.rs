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
    atoms: Vec<TomlAtom>,
}

#[derive(Debug, Deserialize)]
struct TomlAtom {
    classification: String,
    definition: TomlDefinition,
    primary_code: String,
    print_symbol: Option<String>,
    property: String,
    type_name: String,
    human_name: String,
    secondary_code: String,

    // always true
    // is_arbitrary

    // always false?
    // is_metric

    // support later
    // is_special
}

#[derive(Debug, Deserialize)]
struct TomlDefinition {
    value: f64,
    unit: String,
}

pub fn build() {
    let contents = read_atoms_toml();
    let atoms = convert_toml_to_struct(&contents);
    let atom_string = build_atoms_string(atoms);

    write_atoms_rs(&atom_string);
}

fn read_atoms_toml() -> String {
    let src_dir = env::current_dir().unwrap();
    let src_path = Path::new(&src_dir).join("Atoms.toml");

    let mut atoms_file = File::open(src_path).expect("file not found");

    let mut read_contents = String::new();

    atoms_file
        .read_to_string(&mut read_contents)
        .expect("something went wrong reading the file");

    read_contents
}

fn convert_toml_to_struct(toml: &str) -> AtomList {
     let atoms: AtomList = toml::from_str(&toml).expect("unable to deserialize Atoms.toml");

     atoms
}

fn build_atoms_string(atom_list: AtomList) -> String {
    let mut structs = String::new();
    let mut inserts = String::new();

    for atom in atom_list.atoms {
        structs = format!("{}\n{}", &structs, build_atom_def_string(&atom));
        inserts = format!("{}\n{}", &inserts, build_atom_insert_string(&atom));
    }

    let file_text = format!("
use custom_atoms::CustomAtoms;
use property::Property;
use definition::Definition;

{structs}

lazy_static! {{
    static ref CUSTOM_ATOMS: CustomAtoms = {{
        let mut atoms = CustomAtoms::default();
        {inserts}
        atoms
    }};
}}", structs = structs, inserts = inserts);

    file_text
}

fn build_atom_insert_string(atom: &TomlAtom) -> String {
    format!("    atoms.insert(\"{}\", Box::new({}) as Box<UcumSymbol + 'static>);", atom.primary_code, atom.type_name)
}

fn build_atom_def_string(atom: &TomlAtom) -> String {
    let ps = match atom.print_symbol {
        Some(ref p) => format!("Some(\"{}\")", p),
        None => format!("None"),
    };

    format!("
    #[derive(Clone, Copy)]
    pub struct {type_name};

    impl UcumSymbol for {type_name} {{
        fn classification(&self) -> Classification {{
            Classification::{classification}
        }}

        fn definition(&self) -> Definition {{
            Definition::new({definition_value:.64}, \"{definition_unit}\")
                .expect(\"Bad custom definition for {type_name}\")
        }}

        fn primary_code(&self) -> &'static str {{
            \"{primary_code}\"
        }}

        fn print_symbol(&self) -> Option<&'static str> {{
            {print_symbol}
        }}

        fn property(&self) -> Property {{
            Property::{property}
        }}

        fn names(&self) -> Vec<&'static str> {{
            vec![\"{human_name}\"]
        }}

        fn secondary_code(&self) -> &'static str {{
            \"{secondary_code}\"
        }}

        fn is_arbitrary(&self) -> bool {{ true }}
        fn is_metric(&self) -> bool {{ false }}
        fn is_special(&self) -> bool {{ false }}

        fn scalar(&self) -> f64 {{
            self.calculate_scalar(1.0)
        }}

        fn magnitude(&self) -> f64 {{
            self.calculate_magnitude(self.scalar())
        }}

        fn calculate_scalar(&self, value: f64) -> f64 {{
            self.definition().calculate_scalar(value)
        }}

        fn calculate_magnitude(&self, _value: f64) -> f64 {{
            1.0
        }}
    }}
    ", type_name = atom.type_name,
    classification = atom.classification,
    definition_value = atom.definition.value,
    definition_unit = atom.definition.unit,
    primary_code = atom.primary_code,
    print_symbol = ps,
    property = atom.property,
    human_name = atom.human_name,
    secondary_code = atom.secondary_code
    )
}

fn write_atoms_rs(atoms_string: &str) {
    let dest_dir = env::current_dir().unwrap();
    let dest_path = Path::new(&dest_dir).join("atoms.rs");
    let mut f = File::create(&dest_path).unwrap();


    // f.write_all(atom_string.as_bytes()).unwrap();
    f.write_all(atoms_string.as_bytes()).expect("Problem writing the file")
}
