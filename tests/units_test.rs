extern crate wise_units;

use std::str::FromStr;
use wise_units::Unit;
// use wise_units::UnitLike;

#[test]
fn validate_compatibility() {
    let subject = Unit::from_str("g.[acr_us]/[acr_us]").unwrap();
    let other = Unit::from_str("g").unwrap();

    assert!(subject.is_compatible_with(&other));
    assert!(other.is_compatible_with(&subject));
}
