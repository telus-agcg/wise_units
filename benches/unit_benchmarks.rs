#![feature(test)]

extern crate test;

extern crate wise_units;

use wise_units::Unit;
use std::str::FromStr;
use test::Bencher;

#[bench]
fn decompose_10km2_per_100second(b: &mut Bencher) {
    b.iter(|| Unit::from_str("10km2/100s").unwrap())
}
