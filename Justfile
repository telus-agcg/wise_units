codegen-and-check:
  cd atom_generator && cargo run
  cargo fmt
  cd api && cargo check --all-features
