# Change Log

## [0.2.0] - 2018-06-26

### Added

* [AGDEV-30315] Add serde support using "with_serde" feature flag.
* [AGDEV-30253] Add stdweb support using "with_stdweb" feature flag. Just adds
  `js_serializable!` and `js_deserializable!` for both `Measurement` and `Unit`.
* The `UcumUnit` trait now handles some of the core attributes defined by the
  UCUM spec. `Atom`, `Term`, `Unit`, and `Measurement` implement this.
* The `Convertible` trait allows for converting to different objects using
  `convert_to()`. `Measurement` implements this for `Unit` and `&str`.
* The `FieldEq` trait lets you define how to compare objects using `field_eq()`.
  Normally this would be handled by `PartialEq`, but since it seems more often
  that library users would want to check that `1km == 1000m` than check
  `1km == 1km && 1km != 1000m`.  If they need to check the latter, that's what
  `field_eq()` is for. Both `Measurement` and `Unit` implement this.
* The `Reducible` trait provides methods for reducing an object down to its
  associated base unit(s). `Atom`, `Term`, `Vec<Term>`, `Definition`, `Unit`,
  and `Measurement` implement this.
* `Unit` can now `Deref` to `&[Term]`.
* `Unit` can now `From` into `Vec<Term>`.
* Probably some other things...

### Changed

* *Breaking changes*:
    * `Measurement::mul_scalar()` and `Measurement::div_scalar()` are now solved
      by implementing [`Mul<f64>`](https://doc.rust-lang.org/std/ops/trait.Mul.html)
      and [`Div<f64>`](https://doc.rust-lang.org/std/ops/trait.Div.html).
    * `Unit::mul_u32()` and `Unit::div_u32()` have been removed since dividing a
      `Unit` by a value would end up making the `Unit`'s factor a fraction, but
      UCUM doesn't allow that.
    * `Unit::is_valid` has been removed. `Unit`s will normally be instantiated
      by using `from_str()`, which will fail if the tokens represent an invalid
      unit. It's still possible to make an invalid `Unit` by manually
      instantiating each `Term` then instantiating the `Unit`. You're on your
      own if you do that.
    * `calculate_scalar()` has been moved into a trait and renamed
      `Reducible::reduce_value()`.
    * The `Measurable` trait has been removed, but its methods are still
      implemented accordingly.
    * `Measurement::unit_string()` has been removed. Depending on your need, you
      can use `measurement.unit.to_string()` or `measurement.unit.expression()`,
      or `measurement.unit.expression_reduced()`.
    * `Measurement::to_f64()` has been removed. This just returned `self.value`,
      so there wasn't much value in it.
* Split into two subcrates:
    * `wise_units` (under `api/`): The main API for dealing with units via `Measurement`
      and `Unit` types.
    * `wise_units-atom_generator` (under `atom_generator/`): Code used for generating
      the list of allowed atoms during build time.
* To allow for defining custom units, parsing is now done in two stages:
    1. Parsing the outer tokens (`.`, `/`, `{` and `}`, etc.)
    1. Parsing the unit symbols (`m`, `[acr_us]`, `10^`, etc.)
* Removed `Atom::TheUnity` in favor of dealing with this as a `Term` with a `factor` of 1.
* `Composable::composition()` now returns a `Composition` instead of `Option<Composition>`.
* The `PartialEq` implementation for `Unit` checked that its attributes were
  equal to each other--which isn't what we want here. It now checks that the
  two `Unit`s are compatible, then compares the result of their `scalar()` calls.
* Lots more refactoring.

## [0.1.0] - 2018-01-22

### Added

* Initial release!