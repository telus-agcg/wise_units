# Change Log

## [0.x.y] - yyyy-mm-dd

### Added

* [AGDEV-30315] Add serde support using "with_serde" feature flag.
* [AGDEV-30253] Add stdweb support using "with_stdweb" feature flag. Just adds
  `js_serializable!` and `js_deserializable!` for both `Measurement` and `Unit`.

### Changed

* Split into two subcrates:
  * `wise_units` (under `api/`): The main API for dealing with units via `Measurement`
    and `Unit` types.
  * `wise_units-parsing` (under `parsing/`): Code used strictly for parsing unit
    strings and mapping them to types used in the api.
* To allow for defining custom units, parsing is now done in two stages:
  1. Parsing the outer tokens (`.`, `/`, `{` and `}`, etc.)
  1. Parsing the unit symbols (`m`, `[acr_us]`, `10^`, etc.)
* Removed `Atom::TheUnity` in favor of dealing with this as a `Term` with a `factor` of 1.
* `Composable::composition()` now returns a `Composition` instead of `Option<Composition>`.

## [0.1.0] - 2018-01-22

### Added

* Initial release!
