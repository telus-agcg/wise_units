extern crate wise_units;

use wise_units::Unit;
use std::str::FromStr;

#[test]
fn meow() {
    let u = Unit::from_str("[gal_us]").unwrap();
    println!("U: {:?}", u);
    println!("A: {:?}", u.terms[0].atom);
}
