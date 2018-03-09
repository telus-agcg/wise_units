# wise_units

![Build Status](http://teamcity-build.agrian.com/app/rest/builds/buildType%3Aid%3ACrates_WiseUnits_Test/statusIcon?guest=1)

A Rust library for measuring things in [UCUM](http://unitsofmeasure.org/ucum.html)
terms. It's similar to Ruby's [unitwise](https://github.com/joshwlewis/unitwise).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
wise_units = "0.1"
```

and this to your crate root:

```
extern crate wise_units;
```

_Note_: wise_units depends on the [pest](https://github.com/pest-parser/pest)
parser, which depends on Rust 1.23.

### Feature `with_serde`

wise_units provides the ability to (de)serialize `Measurement`s and its children
using the `with_serde` feature (using [serde](https://serde.rs)). This lets you
convert wise_units objects into (and from!) any of the many formats supported by
serde.

This feature is disabled by default. To enable it:

```toml
[dependencies]
wise_units = { version = "0.1", features = ["with_serde"] }
```

### Feature `with_stdweb`

wise_units will make `Measurement` and `Unit` `js_serializable!` and
`js_deserializable!` with this enabled, which means you can use it in javascript
land, à la `[stdweb]`(https://github.com/koute/stdweb). Note that enabling this
implies the feature `with_serde`.

This feature is disabled by default. To enable it:

```toml
[dependencies]
wise_units = { version = "0.1", features = ["with_stdweb"] }
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

Please run [clippy](https://github.com/rust-lang-nursery/rust-clippy). Right now
you have to use rust nightly, so `cargo +nightly clippy`.

Please also keep things tidy with
[rustfmt](https://github.com/rust-lang-nursery/rustfmt).
