#[macro_use]
extern crate criterion;
extern crate wise_units;

mod common;

use criterion::Criterion;
use std::str::FromStr;
use wise_units::{Convertible, Measurement, Unit};

fn new_group(c: &mut Criterion) {
    c.bench_function_over_inputs("Measurement::new", |b, &unit_string| {
        b.iter(|| Measurement::new(1.0, unit_string));
    }, &common::UNIT_STRINGS);
}

//-----------------------------------------------------------------------------
// impl Convertible
//-----------------------------------------------------------------------------
fn convert_to_str_group(c: &mut Criterion) {
    c.bench_function_over_inputs("Measurement::convert_to(str)", |b, &(lhs_string, rhs_string)| {
        let lhs = Measurement::new(2.0, lhs_string).unwrap();

        b.iter(|| lhs.convert_to(*rhs_string));
    }, &common::UNIT_PAIRS);
}

fn convert_to_unit_group(c: &mut Criterion) {
    c.bench_function_over_inputs("Measurement::convert_to(Unit)", |b, &(lhs_string, rhs_string)| {
        let lhs = Measurement::new(2.0, lhs_string).unwrap();
        let rhs = &Unit::from_str(rhs_string).unwrap();

        b.iter(|| lhs.convert_to(rhs));
    }, &common::UNIT_PAIRS);
}

criterion_group!(
    measurement_benches,

    new_group,
    convert_to_str_group,
    convert_to_unit_group,
);
criterion_main!(measurement_benches);
