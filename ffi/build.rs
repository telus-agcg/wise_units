use cbindgen::{Builder, Language};
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    Builder::new()
        .with_crate(crate_dir)
        .with_language(Language::C)
        .with_parse_deps(true)
        .with_parse_include(&["wise_units"])
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings.h");
}
