# wise_units

![Build Status](http://teamcity-build.agrian.com/app/rest/builds/buildType%3Aid%3ACrates_WiseUnits_Test/statusIcon?guest=1)

A Rust library for measuring things in [UCUM](http://unitsofmeasure.org/ucum.html)
terms. It's similar to Ruby's [unitwise](https://github.com/joshwlewis/unitwise).

## Prerequisites

- Rust 1.56
  - Get it [here](https://rustup.rs/)

## Usage

### Feature `serde`

wise_units provides the ability to (de)serialize `Measurement`s and its children
using the `serde` feature (using [serde](https://serde.rs)). This lets you
convert wise_units objects into (and from!) any of the many formats supported by
serde. For example, deserializing a `Measurement` looks like:

```rust
let m = Measurement::new(123.4, "m2").unwrap();
let json = serde_json::to_string(&m).unwrap();
let expected_json = r#"{"value":123.4,"unit":"m2"}";

assert_eq!(json, expected_json);
```

This feature is disabled by default. To enable it:

```toml
[dependencies]
wise_units = { version = "0.22", features = ["serde"] }
```

## Examples

A `Measurement` is made up of some quantifier, the `value`, and the unit of measure,
the `unit`. Because the UCUM allows for combining its terms in almost infinite
combinations, it's most ergonomic to create a new `Measurement` (or more
specifically, its `Unit`) by letting the constructor accept a `str` slice that
gets parsed accordingly.

```rust
let subject = Measurement::new(5.0, "[pi].m2")
    .expect("These are valid UCUM terms so this shouldn't fail");

let converted = subject.convert_to("m2").unwrap();

assert_floats_eq(converted.value, 15.707_963_267);
```

Many more examples in [tests/measurements_test.rs](tests/measurements_test.rs).

## Development

### Running tests

To run the whole suite... Because of optional features in wise_units, to run all
tests, you should use the `--all-features` flag: `cargo test --all-features`.

### Static Analysis

Please run [clippy](https://github.com/rust-lang-nursery/rust-clippy):
`cargo clippy`.

Please also keep things tidy with
[rustfmt](https://github.com/rust-lang-nursery/rustfmt):
`cargo fmt`

### Generating Things

A bunch of code is generated from `Atoms.toml` using the `atom_generator`
executable. This needs to be run each time `Atoms.toml` gets updated (which is
hardly ever).
