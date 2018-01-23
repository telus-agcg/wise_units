#[macro_use]
extern crate criterion;
extern crate wise_units;

use criterion::Criterion;
use wise_units::Unit;
use std::ops::{Div, Mul};
use std::str::FromStr;

// is_special

// fn is_special_base_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     b.iter(|| unit.is_special());
// }

// fn is_special_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("[gal_us]").unwrap();
//     b.iter(|| unit.is_special());
// }

// fn is_special_derived_unit_number(c: &mut Criterion) {
//     let unit = Unit::from_str("mole").unwrap();
//     b.iter(|| unit.is_special());
// }

// fn is_special_derived_unit_special(c: &mut Criterion) {
//     let unit = Unit::from_str("C").unwrap();
//     b.iter(|| unit.is_special());
// }

// fn is_special_derived_unit_with_factor(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]").unwrap();
//     b.iter(|| unit.is_special());
// }

// fn is_special_derived_unit_with_factor_and_exponent(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3").unwrap();
//     b.iter(|| unit.is_special());
// }

// fn is_special_derived_unit_with_factor_and_exponent_and_denominator(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
//     b.iter(|| unit.is_special());
// }

// // is_unity

// fn is_unity_true(c: &mut Criterion) {
//     let unit = Unit::from_str("1").unwrap();
//     b.iter(|| unit.is_unity());
// }

// fn is_unity_false(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     b.iter(|| unit.is_unity());
// }

// // scalar

// fn scalar_base_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     b.iter(|| unit.scalar());
// }

// fn scalar_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("[gal_us]").unwrap();
//     b.iter(|| unit.scalar());
// }

// fn scalar_derived_unit_number(c: &mut Criterion) {
//     let unit = Unit::from_str("mole").unwrap();
//     b.iter(|| unit.scalar());
// }

// fn scalar_derived_unit_special(c: &mut Criterion) {
//     let unit = Unit::from_str("C").unwrap();
//     b.iter(|| unit.scalar());
// }

// fn scalar_derived_unit_with_factor(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]").unwrap();
//     b.iter(|| unit.scalar());
// }

// fn scalar_derived_unit_with_factor_and_exponent(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3").unwrap();
//     b.iter(|| unit.scalar());
// }

// fn scalar_derived_unit_with_factor_and_exponent_and_denominator(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
//     b.iter(|| unit.scalar());
// }

// // magnitude

// fn magnitude_base_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     b.iter(|| unit.magnitude());
// }

// fn magnitude_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("[gal_us]").unwrap();
//     b.iter(|| unit.magnitude());
// }

// fn magnitude_derived_unit_number(c: &mut Criterion) {
//     let unit = Unit::from_str("mole").unwrap();
//     b.iter(|| unit.magnitude());
// }

// fn magnitude_derived_unit_special(c: &mut Criterion) {
//     let unit = Unit::from_str("C").unwrap();
//     b.iter(|| unit.magnitude());
// }

// fn magnitude_derived_unit_with_factor(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]").unwrap();
//     b.iter(|| unit.magnitude());
// }

// fn magnitude_derived_unit_with_factor_and_exponent(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3").unwrap();
//     b.iter(|| unit.magnitude());
// }

// fn magnitude_derived_unit_with_factor_and_exponent_and_denominator(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
//     b.iter(|| unit.magnitude());
// }

// // composition

// fn composition_base_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     b.iter(|| unit.composition());
// }

// fn composition_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("[gal_us]").unwrap();
//     b.iter(|| unit.composition());
// }

// fn composition_derived_unit_number(c: &mut Criterion) {
//     let unit = Unit::from_str("mole").unwrap();
//     b.iter(|| unit.composition());
// }

// fn composition_derived_unit_special(c: &mut Criterion) {
//     let unit = Unit::from_str("C").unwrap();
//     b.iter(|| unit.composition());
// }

// fn composition_derived_unit_with_factor(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]").unwrap();
//     b.iter(|| unit.composition());
// }

// fn composition_derived_unit_with_factor_and_exponent(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3").unwrap();
//     b.iter(|| unit.composition());
// }

// fn composition_derived_unit_with_factor_and_exponent_and_denominator(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
//     b.iter(|| unit.composition());
// }

// // is_compatible_with

// fn is_compatible_with_base_unit_and_base_unit_true(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     let other = Unit::from_str("km").unwrap();
//     b.iter(|| unit.is_compatible_with(&other));
// }

// fn is_compatible_with_base_unit_and_base_unit_false(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     let other = Unit::from_str("s").unwrap();
//     b.iter(|| unit.is_compatible_with(&other));
// }

// fn is_compatible_with_base_unit_and_derived_unit_true(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     let other = Unit::from_str("a").unwrap();
//     b.iter(|| unit.is_compatible_with(&other));
// }

// fn is_compatible_with_base_unit_and_derived_unit_false(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     let other = Unit::from_str("[foz_us]").unwrap();
//     b.iter(|| unit.is_compatible_with(&other));
// }

// fn is_compatible_with_derived_unit_and_derived_unit_true(c: &mut Criterion) {
//     let unit = Unit::from_str("[gal_us]").unwrap();
//     let other = Unit::from_str("[in_i]3").unwrap();
//     b.iter(|| unit.is_compatible_with(&other));
// }

// fn is_compatible_with_derived_unit_and_derived_unit_false(c: &mut Criterion) {
//     let unit = Unit::from_str("[gal_us]").unwrap();
//     let other = Unit::from_str("[ft_us]2").unwrap();
//     b.iter(|| unit.is_compatible_with(&other));
// }

// fn is_compatible_with_derived_unit_number_other_number(c: &mut Criterion) {
//     let unit = Unit::from_str("mole").unwrap();
//     let other = Unit::from_str("10*10").unwrap();
//     b.iter(|| unit.is_compatible_with(&other));
// }

// fn is_compatible_with_derived_unit_number_not_other_number(c: &mut Criterion) {
//     let unit = Unit::from_str("mole").unwrap();
//     let other = Unit::from_str("[qt_us]").unwrap();
//     b.iter(|| unit.is_compatible_with(&other));
// }

// fn is_compatible_with_derived_unit_special_and_other_special_true(c: &mut Criterion) {
//     let unit = Unit::from_str("C").unwrap();
//     let other = Unit::from_str("[degF]").unwrap();
//     b.iter(|| unit.is_compatible_with(&other));
// }

// fn is_compatible_with_derived_unit_special_and_other_special_false(c: &mut Criterion) {
//     let unit = Unit::from_str("C").unwrap();
//     let other = Unit::from_str("[p'diop]").unwrap();
//     b.iter(|| unit.is_compatible_with(&other));
// }

// fn is_compatible_with_derived_unit_with_factor_and_exponent_true(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3").unwrap();
//     let other = Unit::from_str("100[pt_us]3").unwrap();
//     b.iter(|| unit.is_compatible_with(&other));
// }

// fn is_compatible_with_derived_unit_with_factor_and_exponent_false(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3").unwrap();
//     let other = Unit::from_str("100[pt_us]2").unwrap();
//     b.iter(|| unit.is_compatible_with(&other));
// }

// // expression

// fn expression_base_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     b.iter(|| unit.expression());
// }

// fn expression_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("[gal_us]").unwrap();
//     b.iter(|| unit.expression());
// }

// fn expression_derived_unit_number(c: &mut Criterion) {
//     let unit = Unit::from_str("mole").unwrap();
//     b.iter(|| unit.expression());
// }

// fn expression_derived_unit_special(c: &mut Criterion) {
//     let unit = Unit::from_str("C").unwrap();
//     b.iter(|| unit.expression());
// }

// fn expression_derived_unit_with_factor(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]").unwrap();
//     b.iter(|| unit.expression());
// }

// fn expression_derived_unit_with_factor_and_exponent(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3").unwrap();
//     b.iter(|| unit.expression());
// }

// fn expression_derived_unit_with_factor_and_exponent_and_denominator(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
//     b.iter(|| unit.expression());
// }

// // expression_reduced

// fn expression_reduced_base_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     b.iter(|| unit.expression_reduced());
// }

// fn expression_reduced_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("[gal_us]").unwrap();
//     b.iter(|| unit.expression_reduced());
// }

// fn expression_reduced_derived_unit_number(c: &mut Criterion) {
//     let unit = Unit::from_str("mole").unwrap();
//     b.iter(|| unit.expression_reduced());
// }

// fn expression_reduced_derived_unit_special(c: &mut Criterion) {
//     let unit = Unit::from_str("C").unwrap();
//     b.iter(|| unit.expression_reduced());
// }

// fn expression_reduced_derived_unit_with_factor(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]").unwrap();
//     b.iter(|| unit.expression_reduced());
// }

// fn expression_reduced_derived_unit_with_factor_and_exponent(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3").unwrap();
//     b.iter(|| unit.expression_reduced());
// }

// fn expression_reduced_derived_unit_with_factor_and_exponent_and_denominator(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
//     b.iter(|| unit.expression_reduced());
// }

// // div_u32

// fn div_u32_base_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     b.iter(|| unit.div_u32(3));
// }

// fn div_u32_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("[gal_us]").unwrap();
//     b.iter(|| unit.div_u32(3));
// }

// fn div_u32_derived_unit_number(c: &mut Criterion) {
//     let unit = Unit::from_str("mole").unwrap();
//     b.iter(|| unit.div_u32(3));
// }

// fn div_u32_derived_unit_special(c: &mut Criterion) {
//     let unit = Unit::from_str("C").unwrap();
//     b.iter(|| unit.div_u32(3));
// }

// fn div_u32_derived_unit_with_factor(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]").unwrap();
//     b.iter(|| unit.div_u32(3));
// }

// fn div_u32_derived_unit_with_factor_and_exponent(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3").unwrap();
//     b.iter(|| unit.div_u32(3));
// }

// fn div_u32_derived_unit_with_factor_and_exponent_and_denominator(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
//     b.iter(|| unit.div_u32(3));
// }

// // div

// fn div_base_unit_and_base_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     let other = Unit::from_str("m").unwrap();
//     b.iter(|| unit / other);
// }

// fn div_base_unit_and_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     let other = Unit::from_str("[acr_us]").unwrap();
//     b.iter(|| unit / other);
// }

// fn div_derived_unit_and_base_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("[gal_us]").unwrap();
//     let other = Unit::from_str("s").unwrap();
//     b.iter(|| unit / other);
// }

// fn div_derived_unit_and_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("[gal_us]").unwrap();
//     let other = Unit::from_str("[acr_us]").unwrap();
//     b.iter(|| unit / other);
// }

// fn div_derived_unit_number_and_base_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("mole").unwrap();
//     let other = Unit::from_str("s").unwrap();
//     b.iter(|| unit / other);
// }

// fn div_derived_unit_number_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("mole").unwrap();
//     let other = Unit::from_str("[ft_i]").unwrap();
//     b.iter(|| unit / other);
// }

// fn div_derived_unit_with_factor_and_base_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]").unwrap();
//     let other = Unit::from_str("m").unwrap();
//     b.iter(|| unit / other);
// }

// fn div_derived_unit_with_factor_and_exponent_and_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3").unwrap();
//     let other = Unit::from_str("[ft_i]").unwrap();
//     b.iter(|| unit / other);
// }

// fn div_derived_unit_with_factor_and_exponent_and_denominator_and_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
//     let other = Unit::from_str("[ft_i]").unwrap();
//     b.iter(|| unit / other);
// }

// // mul_u32

// fn mul_u32_base_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     b.iter(|| unit.mul_u32(3));
// }

// fn mul_u32_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("[gal_us]").unwrap();
//     b.iter(|| unit.mul_u32(3));
// }

// fn mul_u32_derived_unit_number(c: &mut Criterion) {
//     let unit = Unit::from_str("mole").unwrap();
//     b.iter(|| unit.mul_u32(3));
// }

// fn mul_u32_derived_unit_special(c: &mut Criterion) {
//     let unit = Unit::from_str("C").unwrap();
//     b.iter(|| unit.mul_u32(3));
// }

// fn mul_u32_derived_unit_with_factor(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]").unwrap();
//     b.iter(|| unit.mul_u32(3));
// }

// fn mul_u32_derived_unit_with_factor_and_exponent(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3").unwrap();
//     b.iter(|| unit.mul_u32(3));
// }

// fn mul_u32_derived_unit_with_factor_and_exponent_and_denominator(c: &mut Criterion) {
//     let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
//     b.iter(|| unit.mul_u32(3));
// }

// // mul

// fn mul_base_unit_and_base_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     let other = Unit::from_str("m").unwrap();
//     b.iter(|| unit * other);
// }

// fn mul_base_unit_and_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("m").unwrap();
//     let other = Unit::from_str("[acr_us]").unwrap();
//     b.iter(|| unit * other);
// }

// fn mul_derived_unit_and_base_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("[gal_us]").unwrap();
//     let other = Unit::from_str("s").unwrap();
//     b.iter(|| unit * other);
// }

// fn mul_derived_unit_and_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("[gal_us]").unwrap();
//     let other = Unit::from_str("[acr_us]").unwrap();
//     b.iter(|| unit * other);
// }

// fn mul_derived_unit_number_and_base_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("mole").unwrap();
//     let other = Unit::from_str("s").unwrap();
//     b.iter(|| unit * other);
// }

// fn mul_derived_unit_number_derived_unit(c: &mut Criterion) {
//     let unit = Unit::from_str("mole").unwrap();
//     let other = Unit::from_str("[ft_i]").unwrap();
//     b.iter(|| unit * other);
// }

// fn mul_derived_unit_with_factor_and_base_unit(c: &mut Criterion) {
//     let unit = &Unit::from_str("10[gal_us]").unwrap();
//     let other = &Unit::from_str("m").unwrap();
//     b.iter(|| unit * other);
// }

fn mul_derived_unit_with_factor_and_exponent_and_derived_unit(c: &mut Criterion) {
    let unit = &Unit::from_str("10[gal_us]3").unwrap();
    let other = &Unit::from_str("[ft_i]").unwrap();

    c.bench_function("multiply derived unit with factor and exponent with a derived unit", |b| {
        b.iter(|| unit * other);
    });
}

fn mul_derived_unit_with_factor_and_exponent_and_denominator_and_derived_unit(c: &mut Criterion) {
    let unit = &Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
    let other = &Unit::from_str("[ft_i]").unwrap();

    c.bench_function(
        "multiple derive unit with factor & exponent & denominator with derived unit", |b| {
        b.iter(|| unit * other);
    });
}


// decompose

fn decompose_10km2_per_100second(c: &mut Criterion) {
    c.bench_function("decompose 10km2/100s", |b| {
        b.iter(|| Unit::from_str("10km2/100s").unwrap());
    });
}

criterion_group!(
    unit_benches,
    mul_derived_unit_with_factor_and_exponent_and_derived_unit,
    mul_derived_unit_with_factor_and_exponent_and_denominator_and_derived_unit,
    decompose_10km2_per_100second
);
criterion_main!(unit_benches);
