#![feature(test)]

extern crate test;

extern crate wise_units;

use wise_units::Unit;
use std::ops::{Div, Mul};
use std::str::FromStr;
use test::Bencher;

// is_special

#[bench]
fn is_special_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    b.iter(|| unit.is_special());
}

#[bench]
fn is_special_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("[gal_us]").unwrap();
    b.iter(|| unit.is_special());
}

#[bench]
fn is_special_derived_unit_number(b: &mut Bencher) {
    let unit = Unit::from_str("mole").unwrap();
    b.iter(|| unit.is_special());
}

#[bench]
fn is_special_derived_unit_special(b: &mut Bencher) {
    let unit = Unit::from_str("C").unwrap();
    b.iter(|| unit.is_special());
}

#[bench]
fn is_special_derived_unit_with_factor(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]").unwrap();
    b.iter(|| unit.is_special());
}

#[bench]
fn is_special_derived_unit_with_factor_and_exponent(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3").unwrap();
    b.iter(|| unit.is_special());
}

#[bench]
fn is_special_derived_unit_with_factor_and_exponent_and_denominator(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
    b.iter(|| unit.is_special());
}

// is_unity

#[bench]
fn is_unity_true(b: &mut Bencher) {
    let unit = Unit::from_str("1").unwrap();
    b.iter(|| unit.is_unity());
}

#[bench]
fn is_unity_false(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    b.iter(|| unit.is_unity());
}

// scalar

#[bench]
fn scalar_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    b.iter(|| unit.scalar());
}

#[bench]
fn scalar_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("[gal_us]").unwrap();
    b.iter(|| unit.scalar());
}

#[bench]
fn scalar_derived_unit_number(b: &mut Bencher) {
    let unit = Unit::from_str("mole").unwrap();
    b.iter(|| unit.scalar());
}

#[bench]
fn scalar_derived_unit_special(b: &mut Bencher) {
    let unit = Unit::from_str("C").unwrap();
    b.iter(|| unit.scalar());
}

#[bench]
fn scalar_derived_unit_with_factor(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]").unwrap();
    b.iter(|| unit.scalar());
}

#[bench]
fn scalar_derived_unit_with_factor_and_exponent(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3").unwrap();
    b.iter(|| unit.scalar());
}

#[bench]
fn scalar_derived_unit_with_factor_and_exponent_and_denominator(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
    b.iter(|| unit.scalar());
}

// magnitude

#[bench]
fn magnitude_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    b.iter(|| unit.magnitude());
}

#[bench]
fn magnitude_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("[gal_us]").unwrap();
    b.iter(|| unit.magnitude());
}

#[bench]
fn magnitude_derived_unit_number(b: &mut Bencher) {
    let unit = Unit::from_str("mole").unwrap();
    b.iter(|| unit.magnitude());
}

#[bench]
fn magnitude_derived_unit_special(b: &mut Bencher) {
    let unit = Unit::from_str("C").unwrap();
    b.iter(|| unit.magnitude());
}

#[bench]
fn magnitude_derived_unit_with_factor(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]").unwrap();
    b.iter(|| unit.magnitude());
}

#[bench]
fn magnitude_derived_unit_with_factor_and_exponent(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3").unwrap();
    b.iter(|| unit.magnitude());
}

#[bench]
fn magnitude_derived_unit_with_factor_and_exponent_and_denominator(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
    b.iter(|| unit.magnitude());
}

// composition

#[bench]
fn composition_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    b.iter(|| unit.composition());
}

#[bench]
fn composition_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("[gal_us]").unwrap();
    b.iter(|| unit.composition());
}

#[bench]
fn composition_derived_unit_number(b: &mut Bencher) {
    let unit = Unit::from_str("mole").unwrap();
    b.iter(|| unit.composition());
}

#[bench]
fn composition_derived_unit_special(b: &mut Bencher) {
    let unit = Unit::from_str("C").unwrap();
    b.iter(|| unit.composition());
}

#[bench]
fn composition_derived_unit_with_factor(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]").unwrap();
    b.iter(|| unit.composition());
}

#[bench]
fn composition_derived_unit_with_factor_and_exponent(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3").unwrap();
    b.iter(|| unit.composition());
}

#[bench]
fn composition_derived_unit_with_factor_and_exponent_and_denominator(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
    b.iter(|| unit.composition());
}

// is_compatible_with

#[bench]
fn is_compatible_with_base_unit_and_base_unit_true(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    let other = Unit::from_str("km").unwrap();
    b.iter(|| unit.is_compatible_with(&other));
}

#[bench]
fn is_compatible_with_base_unit_and_base_unit_false(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    let other = Unit::from_str("s").unwrap();
    b.iter(|| unit.is_compatible_with(&other));
}

#[bench]
fn is_compatible_with_base_unit_and_derived_unit_true(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    let other = Unit::from_str("a").unwrap();
    b.iter(|| unit.is_compatible_with(&other));
}

#[bench]
fn is_compatible_with_base_unit_and_derived_unit_false(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    let other = Unit::from_str("[foz_us]").unwrap();
    b.iter(|| unit.is_compatible_with(&other));
}

#[bench]
fn is_compatible_with_derived_unit_and_derived_unit_true(b: &mut Bencher) {
    let unit = Unit::from_str("[gal_us]").unwrap();
    let other = Unit::from_str("[in_i]3").unwrap();
    b.iter(|| unit.is_compatible_with(&other));
}

#[bench]
fn is_compatible_with_derived_unit_and_derived_unit_false(b: &mut Bencher) {
    let unit = Unit::from_str("[gal_us]").unwrap();
    let other = Unit::from_str("[ft_us]2").unwrap();
    b.iter(|| unit.is_compatible_with(&other));
}

#[bench]
fn is_compatible_with_derived_unit_number_other_number(b: &mut Bencher) {
    let unit = Unit::from_str("mole").unwrap();
    let other = Unit::from_str("10*10").unwrap();
    b.iter(|| unit.is_compatible_with(&other));
}

#[bench]
fn is_compatible_with_derived_unit_number_not_other_number(b: &mut Bencher) {
    let unit = Unit::from_str("mole").unwrap();
    let other = Unit::from_str("[qt_us]").unwrap();
    b.iter(|| unit.is_compatible_with(&other));
}

#[bench]
fn is_compatible_with_derived_unit_special_and_other_special_true(b: &mut Bencher) {
    let unit = Unit::from_str("C").unwrap();
    let other = Unit::from_str("[degF]").unwrap();
    b.iter(|| unit.is_compatible_with(&other));
}

#[bench]
fn is_compatible_with_derived_unit_special_and_other_special_false(b: &mut Bencher) {
    let unit = Unit::from_str("C").unwrap();
    let other = Unit::from_str("[p'diop]").unwrap();
    b.iter(|| unit.is_compatible_with(&other));
}

#[bench]
fn is_compatible_with_derived_unit_with_factor_and_exponent_true(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3").unwrap();
    let other = Unit::from_str("100[pt_us]3").unwrap();
    b.iter(|| unit.is_compatible_with(&other));
}

#[bench]
fn is_compatible_with_derived_unit_with_factor_and_exponent_false(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3").unwrap();
    let other = Unit::from_str("100[pt_us]2").unwrap();
    b.iter(|| unit.is_compatible_with(&other));
}

// expression

#[bench]
fn expression_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    b.iter(|| unit.expression());
}

#[bench]
fn expression_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("[gal_us]").unwrap();
    b.iter(|| unit.expression());
}

#[bench]
fn expression_derived_unit_number(b: &mut Bencher) {
    let unit = Unit::from_str("mole").unwrap();
    b.iter(|| unit.expression());
}

#[bench]
fn expression_derived_unit_special(b: &mut Bencher) {
    let unit = Unit::from_str("C").unwrap();
    b.iter(|| unit.expression());
}

#[bench]
fn expression_derived_unit_with_factor(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]").unwrap();
    b.iter(|| unit.expression());
}

#[bench]
fn expression_derived_unit_with_factor_and_exponent(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3").unwrap();
    b.iter(|| unit.expression());
}

#[bench]
fn expression_derived_unit_with_factor_and_exponent_and_denominator(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
    b.iter(|| unit.expression());
}

// expression_reduced

#[bench]
fn expression_reduced_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    b.iter(|| unit.expression_reduced());
}

#[bench]
fn expression_reduced_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("[gal_us]").unwrap();
    b.iter(|| unit.expression_reduced());
}

#[bench]
fn expression_reduced_derived_unit_number(b: &mut Bencher) {
    let unit = Unit::from_str("mole").unwrap();
    b.iter(|| unit.expression_reduced());
}

#[bench]
fn expression_reduced_derived_unit_special(b: &mut Bencher) {
    let unit = Unit::from_str("C").unwrap();
    b.iter(|| unit.expression_reduced());
}

#[bench]
fn expression_reduced_derived_unit_with_factor(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]").unwrap();
    b.iter(|| unit.expression_reduced());
}

#[bench]
fn expression_reduced_derived_unit_with_factor_and_exponent(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3").unwrap();
    b.iter(|| unit.expression_reduced());
}

#[bench]
fn expression_reduced_derived_unit_with_factor_and_exponent_and_denominator(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
    b.iter(|| unit.expression_reduced());
}

// div_u32

#[bench]
fn div_u32_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    b.iter(|| unit.div_u32(3));
}

#[bench]
fn div_u32_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("[gal_us]").unwrap();
    b.iter(|| unit.div_u32(3));
}

#[bench]
fn div_u32_derived_unit_number(b: &mut Bencher) {
    let unit = Unit::from_str("mole").unwrap();
    b.iter(|| unit.div_u32(3));
}

#[bench]
fn div_u32_derived_unit_special(b: &mut Bencher) {
    let unit = Unit::from_str("C").unwrap();
    b.iter(|| unit.div_u32(3));
}

#[bench]
fn div_u32_derived_unit_with_factor(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]").unwrap();
    b.iter(|| unit.div_u32(3));
}

#[bench]
fn div_u32_derived_unit_with_factor_and_exponent(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3").unwrap();
    b.iter(|| unit.div_u32(3));
}

#[bench]
fn div_u32_derived_unit_with_factor_and_exponent_and_denominator(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
    b.iter(|| unit.div_u32(3));
}

// div

#[bench]
fn div_base_unit_and_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    let other = Unit::from_str("m").unwrap();
    b.iter(|| unit.div(&other));
}

#[bench]
fn div_base_unit_and_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    let other = Unit::from_str("[acr_us]").unwrap();
    b.iter(|| unit.div(&other));
}

#[bench]
fn div_derived_unit_and_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("[gal_us]").unwrap();
    let other = Unit::from_str("s").unwrap();
    b.iter(|| unit.div(&other));
}

#[bench]
fn div_derived_unit_and_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("[gal_us]").unwrap();
    let other = Unit::from_str("[acr_us]").unwrap();
    b.iter(|| unit.div(&other));
}

#[bench]
fn div_derived_unit_number_and_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("mole").unwrap();
    let other = Unit::from_str("s").unwrap();
    b.iter(|| unit.div(&other));
}

#[bench]
fn div_derived_unit_number_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("mole").unwrap();
    let other = Unit::from_str("[ft_i]").unwrap();
    b.iter(|| unit.div(&other));
}

#[bench]
fn div_derived_unit_with_factor_and_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]").unwrap();
    let other = Unit::from_str("m").unwrap();
    b.iter(|| unit.div(&other));
}

#[bench]
fn div_derived_unit_with_factor_and_exponent_and_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3").unwrap();
    let other = Unit::from_str("[ft_i]").unwrap();
    b.iter(|| unit.div(&other));
}

#[bench]
fn div_derived_unit_with_factor_and_exponent_and_denominator_and_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
    let other = Unit::from_str("[ft_i]").unwrap();
    b.iter(|| unit.div(&other));
}

// mul_u32

#[bench]
fn mul_u32_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    b.iter(|| unit.mul_u32(3));
}

#[bench]
fn mul_u32_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("[gal_us]").unwrap();
    b.iter(|| unit.mul_u32(3));
}

#[bench]
fn mul_u32_derived_unit_number(b: &mut Bencher) {
    let unit = Unit::from_str("mole").unwrap();
    b.iter(|| unit.mul_u32(3));
}

#[bench]
fn mul_u32_derived_unit_special(b: &mut Bencher) {
    let unit = Unit::from_str("C").unwrap();
    b.iter(|| unit.mul_u32(3));
}

#[bench]
fn mul_u32_derived_unit_with_factor(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]").unwrap();
    b.iter(|| unit.mul_u32(3));
}

#[bench]
fn mul_u32_derived_unit_with_factor_and_exponent(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3").unwrap();
    b.iter(|| unit.mul_u32(3));
}

#[bench]
fn mul_u32_derived_unit_with_factor_and_exponent_and_denominator(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
    b.iter(|| unit.mul_u32(3));
}

// mul

#[bench]
fn mul_base_unit_and_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    let other = Unit::from_str("m").unwrap();
    b.iter(|| unit.mul(&other));
}

#[bench]
fn mul_base_unit_and_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("m").unwrap();
    let other = Unit::from_str("[acr_us]").unwrap();
    b.iter(|| unit.mul(&other));
}

#[bench]
fn mul_derived_unit_and_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("[gal_us]").unwrap();
    let other = Unit::from_str("s").unwrap();
    b.iter(|| unit.mul(&other));
}

#[bench]
fn mul_derived_unit_and_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("[gal_us]").unwrap();
    let other = Unit::from_str("[acr_us]").unwrap();
    b.iter(|| unit.mul(&other));
}

#[bench]
fn mul_derived_unit_number_and_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("mole").unwrap();
    let other = Unit::from_str("s").unwrap();
    b.iter(|| unit.mul(&other));
}

#[bench]
fn mul_derived_unit_number_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("mole").unwrap();
    let other = Unit::from_str("[ft_i]").unwrap();
    b.iter(|| unit.mul(&other));
}

#[bench]
fn mul_derived_unit_with_factor_and_base_unit(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]").unwrap();
    let other = Unit::from_str("m").unwrap();
    b.iter(|| unit.mul(&other));
}

#[bench]
fn mul_derived_unit_with_factor_and_exponent_and_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3").unwrap();
    let other = Unit::from_str("[ft_i]").unwrap();
    b.iter(|| unit.mul(&other));
}

#[bench]
fn mul_derived_unit_with_factor_and_exponent_and_denominator_and_derived_unit(b: &mut Bencher) {
    let unit = Unit::from_str("10[gal_us]3/[acr_us]").unwrap();
    let other = Unit::from_str("[ft_i]").unwrap();
    b.iter(|| unit.mul(&other));
}


// decompose

#[bench]
fn decompose_10km2_per_100second(b: &mut Bencher) {
    b.iter(|| Unit::from_str("10km2/100s").unwrap());
}
