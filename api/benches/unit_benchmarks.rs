#[macro_use]
extern crate criterion;
extern crate wise_units;

mod common;

use criterion::{BenchmarkId, Criterion};
use std::str::FromStr;
use wise_units::{Composable, IsCompatibleWith, UcumUnit, Unit};

macro_rules! bench_over_inputs_method {
    ($function_name:ident, $test_name:expr, $method_name:ident) => {
        fn $function_name(c: &mut Criterion) {
            let mut group = c.benchmark_group($test_name);

            for unit_string in common::UNIT_STRINGS {
                group.bench_with_input(
                    BenchmarkId::new("from_str", &unit_string),
                    unit_string,
                    |b, unit_string| {
                        let unit = Unit::from_str(unit_string).unwrap();

                        b.iter(|| unit.$method_name());
                    },
                );
            }

            group.finish()
        }
    };
}

macro_rules! bench_over_inputs_math {
    ($function_name:ident, $test_name:expr, $method_name:tt) => {
        fn $function_name(c: &mut Criterion) {
            let mut group = c.benchmark_group($test_name);

            for pair in common::UNIT_PAIRS {
                group.bench_with_input(
                    BenchmarkId::new(stringify!($method_name), format!("{}->{}", pair.0, pair.1)),
                    &pair,
                    |b, (lhs_string, rhs_string)| {
                        let lhs = &Unit::from_str(lhs_string).unwrap();
                        let rhs = &Unit::from_str(rhs_string).unwrap();

                        b.iter(|| lhs $method_name rhs);
                    },
                );
            }

            group.finish()
        }
    };
}

bench_over_inputs_method!(is_special_group, "Unit::is_special()", is_special);

bench_over_inputs_method!(scalar_group, "Unit::scalar()", scalar);
bench_over_inputs_method!(magnitude_group, "Unit::magnitude()", magnitude);
bench_over_inputs_method!(expression_group, "Unit::expression()", expression);
bench_over_inputs_method!(
    expression_reduced_group,
    "Unit::expression_reduced()",
    expression_reduced
);

//-----------------------------------------------------------------------------
// impl Composable
//-----------------------------------------------------------------------------
bench_over_inputs_method!(composition_group, "Unit::composition()", composition);

fn is_compatible_with_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("Unit::is_compatible_with()");

    for pair in common::UNIT_PAIRS {
        group.bench_with_input(
            BenchmarkId::new("is_compatible_with", format!("{}->{}", pair.0, pair.1)),
            &pair,
            |b, (lhs_string, rhs_string)| {
                let lhs = &Unit::from_str(lhs_string).unwrap();
                let rhs = &Unit::from_str(rhs_string).unwrap();

                b.iter(|| lhs.is_compatible_with(rhs));
            },
        );
    }

    group.finish()
}

//-----------------------------------------------------------------------------
// impl Display
//-----------------------------------------------------------------------------
bench_over_inputs_method!(display_group, "Unit::to_string()", to_string);

//-----------------------------------------------------------------------------
// impl FromStr
//-----------------------------------------------------------------------------
fn from_str_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("Unit::from_str()");

    for unit_string in common::UNIT_STRINGS {
        group.bench_with_input(
            BenchmarkId::new("from_str", &unit_string),
            unit_string,
            |b, unit_string| {
                b.iter(|| Unit::from_str(unit_string));
            },
        );
    }

    group.finish()
}

//-----------------------------------------------------------------------------
// impl PartialEq
//-----------------------------------------------------------------------------
bench_over_inputs_math!(partial_eq_group, "Unit::partial_eq()", ==);

//-----------------------------------------------------------------------------
// impl PartialOrd
//-----------------------------------------------------------------------------
bench_over_inputs_math!(partial_ord_gt_group, "Unit::partial_ord(>)", >);

//-----------------------------------------------------------------------------
// impl Mul
//-----------------------------------------------------------------------------
bench_over_inputs_math!(mul_group, "Unit::mul()", *);

//-----------------------------------------------------------------------------
// impl Div
//-----------------------------------------------------------------------------
bench_over_inputs_math!(div_group, "Unit::div()", /);

criterion_group!(
    unit_benches,
    is_special_group,
    scalar_group,
    magnitude_group,
    expression_group,
    expression_reduced_group,
    composition_group,
    is_compatible_with_group,
    display_group,
    from_str_group,
    partial_eq_group,
    mul_group,
    div_group,
    partial_ord_gt_group,
);
criterion_main!(unit_benches);
