# wise_units-ffi

C interface for using `wise_units` in other languages.

## Usage

### iOS

* Create a universal library with `cargo lipo --release`.
* Add the library from `wise_units/target/universal/(debug or release)` and the header from `wise_units/ffi/bindings.h` to your Xcode project.

## Development

Update ffi::measurement with any new/changed interface that needs to be exposed to C. Note that if a type in wise_units will be exposed via ffi, that type must be marked with the #[repr(C)] attribute.
