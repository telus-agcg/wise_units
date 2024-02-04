use crate::{
    tokens::{Dimension, UnitValue},
    UcumUnit,
};

use num_rational::Ratio;

const ONE: Ratio<u128> = Ratio::new_raw(1, 1);

#[derive(Debug, Clone, Copy)]
pub struct Meter;
impl UcumUnit for Meter {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("meter", None)
    }
    fn primary_code(&self) -> &'static str {
        "m"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("m")
    }
    fn dimension(&self) -> Dimension {
        crate::tokens::dimension::LENGTH
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        None
    }
    fn scalar(&self) -> Ratio<u128> {
        ONE
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Second;
impl UcumUnit for Second {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("second", None)
    }
    fn primary_code(&self) -> &'static str {
        "s"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("s")
    }
    fn dimension(&self) -> Dimension {
        crate::tokens::dimension::TIME
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        None
    }
    fn scalar(&self) -> Ratio<u128> {
        ONE
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Gram;
impl UcumUnit for Gram {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("gram", None)
    }
    fn primary_code(&self) -> &'static str {
        "g"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("g")
    }
    fn dimension(&self) -> Dimension {
        crate::tokens::dimension::MASS
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        None
    }
    fn scalar(&self) -> Ratio<u128> {
        ONE
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Radian;
impl UcumUnit for Radian {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("radian", None)
    }
    fn primary_code(&self) -> &'static str {
        "rad"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("rad")
    }
    fn dimension(&self) -> Dimension {
        crate::tokens::dimension::PLANE_ANGLE
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        None
    }
    fn scalar(&self) -> Ratio<u128> {
        ONE
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Kelvin;
impl UcumUnit for Kelvin {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("kelvin", None)
    }
    fn primary_code(&self) -> &'static str {
        "K"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("K")
    }
    fn dimension(&self) -> Dimension {
        crate::tokens::dimension::TEMPERATURE
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        None
    }
    fn scalar(&self) -> Ratio<u128> {
        ONE
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Coulomb;
impl UcumUnit for Coulomb {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("coulomb", None)
    }
    fn primary_code(&self) -> &'static str {
        "C"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("C")
    }
    fn dimension(&self) -> Dimension {
        crate::tokens::dimension::ELECTRIC_CHARGE
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        None
    }
    fn scalar(&self) -> Ratio<u128> {
        ONE
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Candela;
impl UcumUnit for Candela {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("candela", None)
    }
    fn primary_code(&self) -> &'static str {
        "cd"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("cd")
    }
    fn dimension(&self) -> Dimension {
        crate::tokens::dimension::LUMINOUS_INTENSITY
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        None
    }
    fn scalar(&self) -> Ratio<u128> {
        ONE
    }
}
