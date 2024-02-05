#[macro_use]
extern crate criterion;
extern crate wise_units;

mod common;

use criterion::{BenchmarkId, Criterion};
use std::str::FromStr;
use wise_units::{Convertible, Measurement, Unit};

fn new_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("Measurement::try_new");

    for unit_string in common::UNIT_STRINGS {
        group.bench_with_input(
            BenchmarkId::new("try_new", unit_string),
            unit_string,
            |b, unit_string| {
                b.iter(|| Measurement::try_new(1.0, unit_string));
            },
        );
    }

    group.finish()
}

//-----------------------------------------------------------------------------
// impl Convertible
//-----------------------------------------------------------------------------
fn convert_to_str_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("Measurement::convert_to<str>");

    for pair in common::UNIT_PAIRS {
        group.bench_with_input(
            BenchmarkId::new("convert_to", format!("{}->{}", pair.0, pair.1)),
            &pair,
            |b, (lhs_string, rhs_string)| {
                let lhs = Measurement::try_new(2.0, *lhs_string).unwrap();

                b.iter(|| lhs.convert_to(*rhs_string));
            },
        );
    }

    group.finish()
}

fn convert_to_unit_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("Measurement::convert_to<Unit>");

    for pair in common::UNIT_PAIRS {
        group.bench_with_input(
            BenchmarkId::new("convert_to", format!("{}->{}", pair.0, pair.1)),
            &pair,
            |b, (lhs_string, rhs_string)| {
                let lhs = Measurement::try_new(2.0, *lhs_string).unwrap();
                let rhs = &Unit::from_str(rhs_string).unwrap();

                b.iter(|| lhs.convert_to(rhs));
            },
        );
    }

    group.finish()
}

criterion_group!(
    measurement_benches,
    new_group,
    convert_to_str_group,
    convert_to_unit_group,
);
criterion_main!(measurement_benches);
