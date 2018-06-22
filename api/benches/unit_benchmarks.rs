#[macro_use]
extern crate criterion;
extern crate wise_units;

use criterion::Criterion;
use std::str::FromStr;
use wise_units::{Composable, Unit};

static UNIT_STRINGS: [&str; 25] =
    [
        // The unity
        "1",

        // base, factor*base, factor*base^exponent
        "m", "10m", "10m3",

        // prefix*base, factor*prefix*base, factor*prefix*base^exponent
        "cm", "10cm", "10cm3",

        // derived, factor*derived, factor*derived^exponent
        // [in_i] is defined in terms of cm (the previous test)
        "[in_i]", "10[in_i]", "10[in_i]3",

        // derived, factor*derived, factor*derived^exponent again
        // [gal_us] is defined in terms of [in_i] (the previous test)
        "[gal_us]", "10[gal_us]", "10[gal_us]3",

        // derived, factor*derived, factor*derived^exponent
        // mol is just a number
        "mol", "10mol", "10mol3",

        // special_derived, factor*special_derived, factor*special_derived^exponent
        "Cel", "10Cel", "10Cel3",

        // base/base, factor*base/base, factor*base^exponent/base
        "m/s", "10m/s", "10m3/s",

        // base/factor*base, base/factor*base^exponent
        "m/5s", "m/5s2",

        // factor*base^exponent/factor*base^exponent
        "10m3/5s2",
    ];

static UNIT_PAIRS: [(&str, &str); 19] =
    [
        ("m", "m"), ("m", "cm"), ("m", "[in_i]"), ("m", "[gal_us]"), ("m", "mol"), ("m", "Cel"),
        ("m", "m/s"), ("m", "cm/s"), ("m", "[in_i]/s"), ("m", "[gal_us]/s"), ("m", "mol/s"), ("m", "Cel/s"),
        ("[gal_us]", "[in_i]3"),
        ("mol", "10*10"), ("mol", "[gal_us]"),
        ("Cel", "[degF]"), ("Cel", "[pH]"),
        ("10[in_i]3", "100[in_us]3"), ("10[in_i]3", "100[in_us]2"),
    ];

macro_rules! bench_over_inputs_method {
    ($function_name:ident, $test_name:expr, $method_name:ident) => {
        fn $function_name(c: &mut Criterion) {
            c.bench_function_over_inputs($test_name, |b, &unit_string| {
                let unit = Unit::from_str(unit_string).unwrap();

                b.iter(|| unit.$method_name());
            }, &UNIT_STRINGS);
        }
    };
}

macro_rules! bench_over_inputs_math {
    ($function_name:ident, $test_name:expr, $method_name:tt) => {
        fn $function_name(c: &mut Criterion) {
            c.bench_function_over_inputs($test_name, |b, &(lhs_string, rhs_string)| {
                let lhs = &Unit::from_str(lhs_string).unwrap();
                let rhs = &Unit::from_str(rhs_string).unwrap();

                b.iter(|| lhs $method_name rhs);
            }, &UNIT_PAIRS);
        }
    };
}

bench_over_inputs_method!(is_special_group, "Unit::is_special()", is_special);
bench_over_inputs_method!(is_metric_group, "Unit::is_metric()", is_metric);
bench_over_inputs_method!(is_unity_group, "Unit::is_unity()", is_unity);

bench_over_inputs_method!(scalar_group, "Unit::scalar()", scalar);
bench_over_inputs_method!(magnitude_group, "Unit::magnitude()", magnitude);
bench_over_inputs_method!(expression_group, "Unit::expression()", expression);
bench_over_inputs_method!(expression_reduced_group, "Unit::expression_reduced()", expression_reduced);

// Trait impls
bench_over_inputs_method!(composition_group, "Unit::composition()", composition);

fn is_compatible_with_group(c: &mut Criterion) {
    c.bench_function_over_inputs("Unit::is_compatible_with", |b, &(lhs_string, rhs_string)| {
        let lhs = &Unit::from_str(lhs_string).unwrap();
        let rhs = &Unit::from_str(rhs_string).unwrap();

        b.iter(|| lhs.is_compatible_with(rhs));
    }, &UNIT_PAIRS);
}

bench_over_inputs_method!(display_group, "Unit::to_string()", to_string);

fn from_str_group(c: &mut Criterion) {
    c.bench_function_over_inputs("Unit::from_str", |b, &unit_string| {
        b.iter(|| Unit::from_str(unit_string));
    }, &UNIT_STRINGS);
}

bench_over_inputs_math!(partial_eq_group, "Unit::partial_eq", ==);
bench_over_inputs_math!(mul_group, "Unit::mul", *);
bench_over_inputs_math!(div_group, "Unit::div", /);
bench_over_inputs_math!(partial_ord_gt_group, "Unit::partial_ord(>)", >);

criterion_group!(
    unit_benches,

    is_special_group,
    is_metric_group,
    is_unity_group,

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
