# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added

- PCC-174: Derive `Hash` for `Atom`.
- PLCC-287: `impl num_traits::Inv for Measurement`, `Unit`, and `Term`.
- PLCC-287: `impl num_traits::One for Measurement`.
- PLCC-287: `impl From<&Measurement> for f64`.
- PLCC-287: `impl num_traits::ToPrimitive for Measurement`.
- PLCC-287: `impl num_traits::FromPrimitive for Measurement`.
- PLCC-287: `impl num_traits::NumCast for Measurement`.
- Added `Unit::into_terms()` for cases where you just need the `Term`s of the `Unit`.
- Added `unit` constant: `UNITY`
- Added `term` constants: `UNITY`, `UNITY_ARRAY`, and `UNITY_ARRAY_REF`.
- Added `measurement!()` macro for wrapping `Measurement::try_new().unwrap()`.

### Changed

- Rust minimum version updated to `1.64.0`.
- (Internal) Rust Codegen is now using `quote` instead of `handlebars` (the pest parser grammar is
  still generated using `handlebars`).
- (Internal) `Definition` is now an enum and is generic over value type `V`.

### Deprecated

- PLCC-287: Deprecated traits `Invert`, `ToInverse`, `IntoInverse`; implementations of
  `num_traits::Inv` cover these now.
- The new `unit::UNITY` constant deprecates `Unit::is_unity()`.
- The new `term::UNITY*` constants deprecate `Term::is_unity()`.

## [0.22.0] — 2022-03-23

### Added

- Implemented `AsRef<Self>` for `Measurement` and `Unit`. Allows for downstream wrapper types to
  implement other functionality for all types via the `AsRef` implementation.
- Implemented `TryFrom<&str> for Unit` (which just calls `from_str()`) to allow for downstream
  wrapper implementations around `Unit`.
- New `const` `Composition` methods: `new_dimless()`, `new_any()`.
- New `composition` `const`s for common dimensional compositions.

### Changed

- Exported `parser::composition` from the crate root.
- `Measurement::try_new()` now takes `unit: U`, `where Unit: TryFrom<U, crate::Error>`. This still
  allows for passing in a unit expression as a `&str`, but also allows for any other
  implementations of type conversions to `Unit`.
- Changed `AsFraction` trait definition to allow for non-optional `Numerator` and `Denominator`.
- Updated to Rust edition `2021`.

## [0.21.1] — 2021-11-19

### Changed

- Updated `ffi_common` to 0.7.0.

### Fixed

- gh-35: Use a separate rule for parsing annotations--one that permits more characters.

## [0.21.0] — 2021-10-20

### Added

- Implemented `Hash` for:
  - `Classification`
  - `Composition`
  - `Prefix`
- Implemented `Display` for:
  - `Classification`
- Implemented `FieldEq` for:
  - `Term`

### Fixed

- Manually implemented `PartialEq` for `Term`, comparing the composition and `scalar()`
  value (like `Measurement` and `Unit`).

## [0.20.0] — 2021-09-29

### Changed

- Updated deps:
  - `ffi_common` to 0.6.0.

## [0.19.0] — 2021-09-22

### Added

- Derive `FFI` for `Measurement` and `Unit` things.

### Changed

- Updated deps:
  - `ffi_common` to 0.5.0.

## [0.18.0] — 2021-08-11

### Changed

- _BREAKING_: Renamed `Measurement::new()` to `try_new()`; add new `new()` method that takes a
  `Unit` for the `unit` param.
- Updated deps:
  - `ffi_common` to 0.4.0.

### Removed

- _BREAKING_: Removed `impl From<Vec<Term>> for Unit`; replaced with standard constructor:
  `Unit::new(terms)`.

## [0.17.1] — 2021-07-16

### Changed

- Point `ffi_derive`, `ffi_common` to 0.3.0.

## [0.17.0] — 2021-07-16

### Changed

- _BREAKING_: `Measurement::value` and `Measurement::unit` fields are now private, replaced by
  accessor methods.
- _BREAKING_: `Unit::terms` field is now private, replaced by an accessor method of the same name.
- `IsCompatibleWith` now takes references to `self` and `RHS` instead of values. This should not
  change any public use of these functions. As a result:
  - _BREAKING_ (maybe?): `impl<'a, 'b> IsCompatibleWith<&'b [Term]> for &'a Unit>` is now
    `impl IsCompatibleWith<Vec<Term>> for Unit`.
  - _BREAKING_ (maybe?): `impl<'a, 'b> IsCompatibleWith<&'b Unit> for &'a Unit>` is now
    `impl IsCompatibleWith for Unit`.
  - _BREAKING_ (maybe?): `impl<'a, 'b> IsCompatibleWith<&'b Term> for &'a Term>` is now
    `impl IsCompatibleWith for Term`.
  - _BREAKING_ (maybe?): `impl<'a, 'b> IsCompatibleWith<&'b [Term]> for &'a [Term]>` is now
    `impl IsCompatibleWith for Vec<Term>`.
  - _BREAKING_ (maybe?): `impl<'a> Composable for &'a [Term]` is now `impl Composable for Vec<Term>`.
- Updated `pest` to `2.0`
- Refactored parser.
- Made functions `const`:
  - `Atom::property()`
  - `Composition::new()`
  - `Composition::is_empty()`
  - `Term::new_unity()`
  - `Term::has_value()`
  - `Term::exponent_is_positive()`
  - `Term::exponent_is_negative()`
- Updated functions used for converting some arbitrary units to use more correct implementations:
  - `Neper`
  - `MeterPerSquareSecondsPerSquareRootOfHertz`
  - `BitLogarithmusDualis`
- Split `parser::Error::UnableToParse` into new variants:
  1. `Error::UnableToParseTerm`
  1. `Error::UnableToParseSymbol`
  1. `Error::UnableToParseInteger`
  1. `Error::BadFragment`
- `custom_ffi::clone_unit()` and `custom_ffi::get_unit_expression()` are now marked `unsafe`.
- Used [typos](https://github.com/crate-ci/typos) to find and fix typos. This included some internal
  types.

## [0.16.0] — 2020-12-16

### Added

- Add `ffi_derive` for `Measurement` and `Unit` (behind the `cffi` feature flag).

### Changed

- Updated `ffi_common` and `ffi_derive` to 0.2.
- `wise_units-ffi`'s `build.rs` now uses `OUT_DIR` instead of `CARGO_MANIFEST_DIR`.

### Fixed

- Pointed `agrian-registry` to GitHub.

## [0.15.0] — 2020-07-31

### Changed

- No longer generate code every time we build. That code now just needs to be manually generated any
  time `Atoms.toml` is updated.

## [0.14.1] — 2020-07-22

### Fixed

- Removed `serde_json` as a dep for the `atom_generator`, resolving downstream serialization issues.

## [0.14.0] — 2020-05-08

### Added

- `Composition` now exposes `const fn`s for constructing one-`Dimension`al objects.
- `Composition` now derives `Eq`.
- `Error` now derives `serde::Serialize`.

### Changed

- Switched internal error-helper crate from `failure` to `thiserror`.

## [0.13.0] — 2020-03-20

### Added

- `Measurement` had a submodule for implementing `Display`, but it wasn't being `use`d; could call
  this a "fix", but technically it's adding this impl.

### Removed

- Both `measurement` and `unit` had all submodules as `pub` that just implement traits; this is
  unnecessary.

### Fixed

- `parser::property::Property`'s implementation of `Display` wasn't working.

## [0.12.0] — 2019-11-07

### Added

- [DEV-8049] Added `wise_units-ffi` crate to expose C bindings for wise_units functions, with C-safe
  wrapper types around `Measurement` and `Unit`.
- Derive `Clone` for `error::Error` and `parser::Error`.

## [0.11.0] — 2019-07-17

### Added

### Changed

- Changed `serde` serialization and deserialization of `Measurement` and `Unit` to expect/return
  `Unit`s as `String`s. The previous implementation expected a `Vec<Term>`, which is really a
  private-ish API (or at least not ergonomic for public consumption).
- Renamed feature `with_serde` to just `serde` to follow convention.

### Fixed

- Fixed lots of clippy warnings.

### Removed

- Bad `api/clippy.toml` was causing some builds to fail; removed the file.
- Removed `derive(Serialize, Deserialize)` from `Atom`, `Prefix`, and `Term` since these no longer
  need to be (de)serialized.

## [0.10.0] — 2019-01-11

### Added

- [DEV-4385] Added new `error::Error` type that derives `Fail`. So far, all errors have been
  captured in `parser::error::Error`, but it really only makes sense to handle parsing-specific
  errors with that. The new type should provide errors for the whole crate and thus wraps the old
  `parser::error::Error` as needed. Also moved the `IncompatibleUnitTypes` variant to the new type.
  The new type also implements `From<parser::Error>` for ergonomic conversions.
- [DEV-4385] Implemented all the `invert` traits for `Measurement`. Seems I forgot to do that for
  0.9.0. This also required a new `DivideByZero` error variant in the new `error::Error`. ...which
  also required changing the `ToInverse` and `IntoInverse` traits to add an `Output` associated type
  to allow clients to handle the `DivideByZero` case.

## [0.9.0] — 2019-01-10

### Added

- Implemented `From<&[Term]> for Unit` and used in `prefix`.

### Changed

- [DEV-4413] Changed `into_reduced::IntoReduced` to `reduce::ToReduced`, added
  `reduced::IntoReduced` that consumes `self` (which is convention), and auto-derived `IntoReduced`
  for all types that implement `ToReduced`.
- [DEV-4385] Changed `invert::IntoInverse` to `invert::ToInverse`, added `invert::IntoReduced` that
  consumes `self` (which is convention), and auto-derived `IntoInverse` for all types that implement
  `ToInverse`.

### Removed

- Removed `decomposer::Deomposable` and turned the old `Simple` decomposer struct into a function,
  `decomposer::decompose()`.

## [0.8.0] — 2019-01-09

### Added

- [DEV-4413] Added `IntoReduced` trait and implemented for `Measurement` and `Unit`.

## [0.7.1] — 2019-01-08

### Changed

- Updated implementations of `Div` and `Mul` for `Unit` to be more performant.
- Updated implementation of `IntoInverse` for `Vec<Term>` to be more performant.

## [0.7.0] — 2019-01-07

### Added

- [DEV-4385] Added `invert::Invert` and `invert::IntoInverse` traits and implemented for `Term`,
  `Vec<Term>`, and `Unit`.
- Inline `Unit::numerator()` and `Unit::denominator()`.

### Changed

- [DEV-4385] `AsFraction`'s `Numerator` now returns `Option<Self::Numerator>` to handle the case of
  a per- unit Unit.
- Switched to Rust edition 2018.

### Fixed

- Fixed typo in method name: `AsFraction::as_fraction()`.

### Removed

- [DEV-4385] Removed `Term::invert_exponent` in favor of the new implementations of `Invert` and
  `IntoInverse`.

## [0.6.0] — 2019-01-04

### Added

- [DEV-2508] Added `Unit::is_unity()`.

### Fixed

- `Unit::from_str("1")` didn't parse such that `unit.terms[0].is_unity()` returned `true`; it now
  skips parsing and just returns a proper `Unit`.

## [0.5.1] — 2018-11-06

### Changed

- [DEV-3155] Reverted the change to the `Composable` trait definition to deal only with `self`; this
  caused `cannot move out of borrowed content` errors when trying to use the API normally.

## [0.5.0] — 2018-11-06

### Added

- [DEV-241] Add `AsFraction` and implement for `Unit`.
- [DEV-1013] Inlined most of the public API methods.
- [DEV-3155] Added `DefaultCompatibility` marker trait to mark types that should implement the
  default behavior of `IsCompatibleWith`, effectively paving the way to be able to define a
  different behavior for `Term`.
- [DEV-3327] Added `AnnotationComposable` and `AnnotationComposition` and implemented for `&[Term]`.
  Allows for pessimistic comparing of `Term`s with annotations.

### Changed

- [DEV-3155] Changed `Term`'s implementation of `Display` to include its `annotation`.
- [DEV-3155] Changed the `Composable` trait definition to deal only with `self` (which makes
  implementations cleaner).
- [DEV-3155] Refactored out `Composable::is_compatible_with()` into a new trait, `IsCompatibleWith`,
  and blanket-implemented that for all types that implement `Composable`. Not only does this clean
  up code, but also now lets you compare `Measurement`s and `Unit`s. Library consumers that call
  `is_compatible_with` will need to change `use wise_units::Composable` to
  `use wise_units::IsCompatibleWith`.
- [DEV-3155] Changed `Term::is_compatible_with()` to account for the `annotation` attribute of each
  `Term` being compared.

### Removed

- In preparation for the Rust 2018 edition release before the end of the year, updated to Rust
  1.30.0 features, _making this the minimum supported version_.

### Fixed

- [DEV-3300] Parsing annotations now excludes curly braces in the resulting string capture.

## [0.4.0] — 2018-10-02

### Added

- Implemented `Composable` for `Measurement` to allow checking compatibility of `Measurement`s.
- `Unit::into_reduced()`
- `unit::term_reducing` for reducing a `Unit`'s `Term`s. This, in conjunction with the
  still-existing `SimpleDecomposer`, now replaces the `ReductionDecomposer`.
- `Term::has_value()`

### Changed

- [DEV-236] `Unit`s are now reduced after being multiplied or divided.
- A `Unit` that's supposed to represent the unity ("1") used to be a `Unit` with a `Vec<Term>` with
  1 `Term` whose attributes were all `None`; now that same `Term` has a `factor` of `Some(1)`. It's
  possible that a `Unit` with 0 `Term`s may still be interpreted similarly (haven't looked into this
  yet).

## [0.3.0] — 2018-08-27

### Added

- `Decomposable` is now public.
- `UcumSymbol` is now public.
- Added `Term::factor_as_u32()`.
- `Term` `factor` and `exponent` are now wrapped in an `Option` since many `Term`s don't need these
  values.

### Changed

- Refactored trait implementations for `Measurement` and `Unit` to sub-modules. The files were
  getting too long.
- `Term::factor` and `Term::exponent` are now wrapped in an `Option` (the same goes for their
  related `parser::terms::mapper` components) to save unnecessary allocations.
- Renamed `Decomposable::expression()` to `Decomposable::decompose()` to be more conventional.
- `Decomposable::decompose()` now takes a value to let the caller decide what to pass in.
- `Decomposable` now defines associated types for more `impl` flexibility.
- Extracted some of `Decomposable::decompose()` to a new method, `Decomposable::format_output()`,
  which lets consumers customize the formatted output.
- `Decomposable::numerator()` and `Decomposable::denominator()` now return `Option<String>` instead
  of `String`.
- Refactored a number of `Decomposable` implementation methods.

### Removed

- _Removed_ `stdweb` support. This was overkill from the start.

### Fixed

- [DEV-331] Fixed a number of conversion bugs. Redesigned `Composition` to get there.
- To match SI displaying, added a space for the implementation of `Display` for `Measurement` in
  between the value and the unit.

## [0.2.0] — 2018-06-26

### Added

- Added implementations of `Add`, `Sub`, `Mul`, `Div` for `&'a Measurement` and `&'a Unit` where the
  right-hand side is an owned version of the same type.
- Added implementations of `Mul`, `Div` for `&'a Measurement` where the right-hand side is an `f64`.

### Changed

- `Convertible` trait now uses associated types for the output type and the error type, letting
  definers use their own.

### Removed

- Removed implementations of `Add`, `Sub`, `Mul`, `Div` for `&'a mut Measurement` and
  `&'a mut Unit`. Those seem like edge-cases and thus code bloat at this point.

## [0.2.0] — 2018-06-26

### Added

- [AGDEV-30315] Add serde support using "with_serde" feature flag.
- [AGDEV-30253] Add stdweb support using "with_stdweb" feature flag. Just adds `js_serializable!`
  and `js_deserializable!` for both `Measurement` and `Unit`.
- The `UcumUnit` trait now handles some core attributes defined by the UCUM spec. `Atom`,
  `Term`, `Unit`, and `Measurement` implement this.
- The `Convertible` trait allows for converting to different objects using `convert_to()`.
  `Measurement` implements this for `Unit` and `&str`.
- The `FieldEq` trait lets you define how to compare objects using `field_eq()`. Normally this would
  be handled by `PartialEq`, but since it seems more often that library users would want to check
  that `1km == 1000m` than check `1km == 1km && 1km != 1000m`. If they need to check the latter,
  that's what `field_eq()` is for. Both `Measurement` and `Unit` implement this.
- The `Reducible` trait provides methods for reducing an object down to its associated base unit(s).
  `Atom`, `Term`, `Vec<Term>`, `Definition`, `Unit`, and `Measurement` implement this.
- `Unit` can now `Deref` to `&[Term]`.
- `Unit` can now `From` into `Vec<Term>`.
- Probably some other things...

### Changed

- Lots of refactoring.
- The `PartialEq` implementation for `Unit` checked that its attributes were equal to each
  other--which isn't what we want here. It now checks that the two `Unit`s are compatible, then
  compares the result of their `scalar()` calls.
- To allow for defining custom units, parsing is now done in two stages:
  1. Parsing the outer tokens (`.`, `/`, `{` and `}`, etc.)
  1. Parsing the unit symbols (`m`, `[acr_us]`, `10^`, etc.)

### Removed

- `Measurement::mul_scalar()` and `Measurement::div_scalar()` are now solved by implementing
  [`Mul<f64>`](https://doc.rust-lang.org/std/ops/trait.Mul.html) and
  [`Div<f64>`](https://doc.rust-lang.org/std/ops/trait.Div.html).
- `Unit::mul_u32()` and `Unit::div_u32()` have been removed since dividing a `Unit` by a value would
  end up making the `Unit`'s factor a fraction, but UCUM doesn't allow that.
- `Unit::is_valid` has been removed. `Unit`s will normally be instantiated by using `from_str()`,
  which will fail if the tokens represent an invalid unit. It's still possible to make an invalid
  `Unit` by manually instantiating each `Term` then instantiating the `Unit`. You're on your own if
  you do that.
- `calculate_scalar()` has been moved into a trait and renamed `Reducible::reduce_value()`.
- The `Measurable` trait has been removed, but its methods are still implemented accordingly.
- `Measurement::unit_string()` has been removed. Depending on your need, you can use
  `measurement.unit.to_string()` or `measurement.unit.expression()`, or
  `measurement.unit.expression_reduced()`.
- `Measurement::to_f64()` has been removed. This just returned `self.value`, so there wasn't much
  value in it.
- Split into two subcrates:
  - `wise_units` (under `api/`): The main API for dealing with units via `Measurement` and `Unit`
    types.
  - `wise_units-atom_generator` (under `atom_generator/`): Code used for generating the list of
    allowed atoms during build time.
- Removed `Atom::TheUnity` in favor of dealing with this as a `Term` with a `factor` of 1.
- `Composable::composition()` now returns a `Composition` instead of `Option<Composition>`.

## [0.1.0] — 2018-01-22

### Added

- Initial release!
