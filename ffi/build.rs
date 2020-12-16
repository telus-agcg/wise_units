use cbindgen::{self, Builder, Language};
use std::env;

fn main() {
    let crate_dir = env::var("OUT_DIR").unwrap();

    Builder::new()
        .with_crate(crate_dir)
        .with_language(Language::C)
        .generate()
        .map(|bindings| bindings.write_to_file("bindings.h"))
        .unwrap_or_else(|e| {
            eprintln!("Unable to generate bindings: {}", e);
            false
        });
}
