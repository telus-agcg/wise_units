# wise_units-atom_generator

A library for generating `wise_units-parser` code from `Atoms.toml`, accommodating
custom units ("atoms") as needed. Since it generates code, it's really only
useful as a `build-dependency`.

## Usage

If you're reading this, you probably only care about getting your custom units
defined and into `wise_units`. First, update your crate's `Cargo.toml` with:

```toml
[build-dependencies]
wise_units-atom_generator = "0.1"
```

Now define your custom atoms in a `CustomAtoms.toml` file in the root of your
crate. The syntax should follow that of this crate's `Atoms.toml`; examples:

```toml
# A dimensionless unit is really just a number:
[[unit]]
Code = '[stuff]'
isMetric = 'no'
class = 'dimless'
names = ['a stuff is lots of something']
printSymbol = 'stuff'
property = 'number'

  [unit.value]
  Unit = '[pi]2/2'
  value = 100_000

# An arbitrary unit is one that shouldn't be able to be converted to another:
[[unit]]
Code = '[thingy]'
isArbitrary = 'yes'
isMetric = 'no'
class = 'dimless'
names = ['thingy']
printSymbol = 'thingy'
property = 'arbitrary'

  [unit.value]
  Unit = '1'
  value = 1
```

Lastly, add a `build.rs` to your crate's root with the contents:

```rust
extern crate wise_units_atom_generator;

fn main() {
    ::wise_units_atom_generator::build_with_custom_atoms();
}
```

Now when you build your crate, `wise_units_atom_generator` will add your
`CustomAtoms.toml` into `wise_units`'s parser, letting you treat your units
just like any other `wise_units` unit.

## Atom Rules

1. Before you do anything, get familiar with http://unitsofmeasure.org/ucum.html.
    * All attributes of your unit must comply with UCUM syntax for that attribute.
1. Refrain from defining every "unit" you think of--or at least criticize doing
   so.
    * Consider using UCUM _annotations_ to give context to your unit instead of
      defining a new one (ex. `g{HGB}` = "gram of HGB"; a "gram" is a gram,
      regardless of what substance it measures).
    * The [SI](https://www.bipm.org/en/publications/si-brochure/) (and probably
      other sources) differentiates between a "quantity" and a "unit", where a
      quantity describes the measurement of something, but not in terms of a unit:
      ex. _m<sub>n</sub>_ represents the quantity "mass of a neutron", but does not
      imply which units it's measured in. Quantities should *NOT* be defined as
      custom units.
   * Each unit you add, enlarges the size of the resulting binary and slightly
     degrades performance, so again, scrutinize what you're doing before doing
     so.

### Atom Attribute Rules

When adding units to `CustomAtoms.toml`, follow these rules:

1. `Code`
    * is required.
    * must generate Rust code that is unique with respect to other `Code`s; put
      differently, it must *not* generate Rust code that conflicts with an
      already-defined `Code`.
1. `CODE`
    * is optional.
    * same as `Code`, but for case-sensitive environments.
1. `dim` (dimension)
    * is required.
    * must be one of:
        * `A` (plane angle)
        * `C` (temperature)
        * `F` (luminous intensity)
        * `L` (length)
        * `M` (mass)
        * `Q` (electric charge)
        * `T` (time)
1. `class` (classification)
    * is required.
    * must be one of:
        * `apoth`
        * `avoirdupois`
        * `brit-length`
        * `brit-volumes`
        * `cgs`
        * `chemical`
        * `clinical`
        * `const`
        * `dimless`
        * `heat`
        * `infotech`
        * `intcust`
        * `iso1000`
        * `levels`
        * `misc`
        * `si`
        * `troy`
        * `typeset`
        * `us-lengths`
        * `us-volumes`
    * ...although most of those should already be covered by UCUM unit definitions,
      so you should probably end up with `dimless` or `misc` unless you have a
      good reason to be adding to one of those.
1. `names`
    * is required.
    * can be any number of strings, although typically it's just a single string.
1. `property`
    * is required.
    * there are about 100 of these types defined in
      [the UCUM XML document](http://unitsofmeasure.org/ucum-essence.xml) so pick
      one of those.
    * common values are:
        * `arbitrary`
        * `area`
        * `dry volume`
        * `energy`
        * `fluid volume`
        * `fraction`
        * `length`
        * `mass`
        * `number`
        * `plane angle`
        * `pressure`
        * `temperature`
        * `time`
        * `velocity`
        * `volume`
        * `unclassified`
1. `unit.value`
    * is required.
    * `Unit`
        * is required.
        * is the unit string that makes up your unit: ex `m2`.
    * `UNIT`
        * is optional.
        * same as `Unit`, but for case-sensitive environments.
    * `value`
        * is required.
        * is the quantity of `Unit` that defines this unit.
1. `printSymbol`
    * is optional.
1. `isMetric`
    * is optional; defaults to `no`.
1. `isArbitrary`
    * is optional; defaults to `no`.
1. `isSpecial`
    * is optional; defaults to `no`.
    * (actually defining custom special units is not yet supported)