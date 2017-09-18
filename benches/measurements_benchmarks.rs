#![feature(test)]

extern crate test;
extern crate wise_units;

use wise_units::Measurement;
use test::Bencher;

#[bench]
fn create_meter(b: &mut Bencher) {
    b.iter(|| Measurement::new(1.0, "m"));
}

#[bench]
fn create_km(b: &mut Bencher) {
    b.iter(|| Measurement::new(1.0, "km"));
}

#[bench]
fn create_km2(b: &mut Bencher) {
    b.iter(|| Measurement::new(1.0, "km2"));
}

#[bench]
fn create_m2_per_second(b: &mut Bencher) {
    b.iter(|| Measurement::new(1.0, "m2/s"));
}

#[bench]
fn create_km2_per_second(b: &mut Bencher) {
    b.iter(|| Measurement::new(1.0, "km2/s"));
}

#[bench]
fn create_10km2_per_100second(b: &mut Bencher) {
    b.iter(|| Measurement::new(1.0, "10km2/100s"));
}

#[bench]
fn create_millimole_per_hour(b: &mut Bencher) {
    b.iter(|| Measurement::new(1.0, "mmol/h"));
}

#[bench]
fn create_milliliter_per_kilogram_and_hour(b: &mut Bencher) {
    b.iter(|| Measurement::new(1.0, "mL/(kg.h)"));
}

#[bench]
fn convert_meter_to_km(b: &mut Bencher) {
    let measurement = Measurement::new(1.0, "m");
    b.iter(|| measurement.convert_to("km").unwrap());
}

#[bench]
fn convert_km_to_meter(b: &mut Bencher) {
    b.iter(|| Measurement::new(1.0, "km").convert_to("m").unwrap());
}

#[bench]
fn add_km_to_meter(b: &mut Bencher) {
    b.iter(|| Measurement::new(1.0, "km") + Measurement::new(1.0, "m"))
}

#[bench]
fn sub_km_to_meter(b: &mut Bencher) {
    b.iter(|| Measurement::new(1.0, "km") - Measurement::new(1.0, "m"))
}

#[bench]
fn mul_km_to_meter(b: &mut Bencher) {
    b.iter(|| Measurement::new(1.0, "km") * Measurement::new(1.0, "m"))
}

#[bench]
fn div_km_to_meter(b: &mut Bencher) {
    b.iter(|| Measurement::new(1.0, "km") / Measurement::new(1.0, "m"))
}
