#![forbid(unused_imports)]
#![deny(unused_extern_crates)]
#![warn(
    box_pointers,
    future_incompatible,
    missing_copy_implementations,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts
)]

use std::fs::File;
use std::io::{Read, Write};

fn main() -> Result<(), ::std::io::Error> {
    simple_logger::SimpleLogger::new().init().unwrap();

    // let xml_string = fetch();
    let xml_string = read_xml_file();

    let output = transcode(&xml_string);

    let mut atoms = File::create("Atoms.toml")?;
    atoms.write_all(output.as_bytes())?;

    Ok(())
}

/// Retrieves the source XML from the UCUM website. We're not using this now,
/// however, since the XML contains printSymbols that can't be successfully
/// deserialized; they have to be manually removed, then added back to the
/// resulting TOML doc.
async fn _fetch() -> String {
    reqwest::get("http://unitsofmeasure.org/ucum-essence.xml")
        .await
        .expect("Unable to fetch XML")
        .text()
        .await
        .expect("Unable to read response")
}

fn read_xml_file() -> String {
    let mut f = File::open("ucum-essence.xml").expect("file not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the UCUM XML file");

    contents
}

fn transcode(xml_string: &str) -> String {
    let mut toml_string = String::new();

    {
        let mut deserializer = serde_xml_rs::Deserializer::new_from_reader(xml_string.as_bytes());
        let mut serializer = toml::ser::Serializer::pretty(&mut toml_string);

        serde_transcode::transcode(&mut deserializer, &mut serializer)
            .expect("Unable to translcode XML to TOML");
    }

    toml_string
}
