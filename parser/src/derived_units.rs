use std::num::NonZeroI8;

use num_rational::Ratio;

use crate::{
    tokens::{Dimension, UnitValue, ValueFunction},
    UcumUnit,
};

#[derive(Debug, Clone, Copy)]
pub struct TheNumberTenForArbitraryPowers10star;
impl UcumUnit for TheNumberTenForArbitraryPowers10star {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("the number ten for arbitrary powers", None)
    }
    fn primary_code(&self) -> &'static str {
        "10*"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("10")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 10.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(10, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct TheNumberTenForArbitraryPowers10caret;
impl UcumUnit for TheNumberTenForArbitraryPowers10caret {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("the number ten for arbitrary powers", None)
    }
    fn primary_code(&self) -> &'static str {
        "10^"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("10")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 10.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(10, 1)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct TheNumberPi;
impl UcumUnit for TheNumberPi {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("the number pi", None)
    }
    fn primary_code(&self) -> &'static str {
        "[pi]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("\u{3c0}")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    // NOTE: Allowing this version of pi, despite Rust stdlib defining it too, since this is what
    // the spec says. Maybe this should change?
    #[allow(clippy::approx_constant)]
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 3.141_592_653_589_793,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3_141_592_653_589_793, 1_000_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Percent;
impl UcumUnit for Percent {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("percent", None)
    }
    fn primary_code(&self) -> &'static str {
        "%"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("%")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "10*-2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 100)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PartsPerThousand;
impl UcumUnit for PartsPerThousand {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("parts per thousand", None)
    }
    fn primary_code(&self) -> &'static str {
        "[ppth]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("ppth")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "10*-3",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PartsPerMillion;
impl UcumUnit for PartsPerMillion {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("parts per million", None)
    }
    fn primary_code(&self) -> &'static str {
        "[ppm]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("ppm")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "10*-6",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PartsPerBillion;
impl UcumUnit for PartsPerBillion {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("parts per billion", None)
    }
    fn primary_code(&self) -> &'static str {
        "[ppb]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("ppb")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "10*-9",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PartsPerTrillion;
impl UcumUnit for PartsPerTrillion {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("parts per trillion", None)
    }
    fn primary_code(&self) -> &'static str {
        "[pptr]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("pptr")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "10*-12",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Mole;
impl UcumUnit for Mole {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("mole", None)
    }
    fn primary_code(&self) -> &'static str {
        "mol"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("mol")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 6.022_136_7,
            unit: "10*23",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(602_213_670_000_000_000_000_000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Steradian;
impl UcumUnit for Steradian {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("steradian", None)
    }
    fn primary_code(&self) -> &'static str {
        "sr"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("sr")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "rad2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Hertz;
impl UcumUnit for Hertz {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("hertz", None)
    }
    fn primary_code(&self) -> &'static str {
        "Hz"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Hz")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "s-1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Newton;
impl UcumUnit for Newton {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("newton", None)
    }
    fn primary_code(&self) -> &'static str {
        "N"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("N")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "kg.m/s2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Pascal;
impl UcumUnit for Pascal {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("pascal", None)
    }
    fn primary_code(&self) -> &'static str {
        "Pa"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Pa")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "N/m2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Joule;
impl UcumUnit for Joule {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("joule", None)
    }
    fn primary_code(&self) -> &'static str {
        "J"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("J")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "N.m",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Watt;
impl UcumUnit for Watt {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("watt", None)
    }
    fn primary_code(&self) -> &'static str {
        "W"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("W")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-3) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "J/s",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct Ampere;
impl UcumUnit for Ampere {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("amp\u{e8}re", None)
    }
    fn primary_code(&self) -> &'static str {
        "A"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("A")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "C/s",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Volt;
impl UcumUnit for Volt {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("volt", None)
    }
    fn primary_code(&self) -> &'static str {
        "V"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("V")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "J/C",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Farad;
impl UcumUnit for Farad {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("farad", None)
    }
    fn primary_code(&self) -> &'static str {
        "F"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("F")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "C/V",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1000)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct Ohm;
impl UcumUnit for Ohm {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("ohm", None)
    }
    fn primary_code(&self) -> &'static str {
        "Ohm"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("\u{3a9}")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "V/A",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Siemens;
impl UcumUnit for Siemens {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("siemens", None)
    }
    fn primary_code(&self) -> &'static str {
        "S"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("S")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "Ohm-1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Weber;
impl UcumUnit for Weber {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("weber", None)
    }
    fn primary_code(&self) -> &'static str {
        "Wb"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Wb")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "V.s",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct DegreeCelsius;
impl UcumUnit for DegreeCelsius {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("degree Celsius", None)
    }
    fn primary_code(&self) -> &'static str {
        "Cel"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("\u{b0}C")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        true
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "cel(1 K)",
            function: Some(ValueFunction {
                name: "Cel",
                value: Ratio::new_raw(1, 1),
                unit: "K",
            }),
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Tesla;
impl UcumUnit for Tesla {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("tesla", None)
    }
    fn primary_code(&self) -> &'static str {
        "T"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("T")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "Wb/m2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Henry;
impl UcumUnit for Henry {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("henry", None)
    }
    fn primary_code(&self) -> &'static str {
        "H"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("H")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "Wb/A",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Lumen;
impl UcumUnit for Lumen {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("lumen", None)
    }
    fn primary_code(&self) -> &'static str {
        "lm"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("lm")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "cd.sr",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Lux;
impl UcumUnit for Lux {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("lux", None)
    }
    fn primary_code(&self) -> &'static str {
        "lx"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("lx")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "lm/m2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Becquerel;
impl UcumUnit for Becquerel {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("becquerel", None)
    }
    fn primary_code(&self) -> &'static str {
        "Bq"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Bq")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "s-1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Gray;
impl UcumUnit for Gray {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("gray", None)
    }
    fn primary_code(&self) -> &'static str {
        "Gy"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Gy")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "J/kg",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Sievert;
impl UcumUnit for Sievert {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("sievert", None)
    }
    fn primary_code(&self) -> &'static str {
        "Sv"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Sv")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "J/kg",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct Gon;
impl UcumUnit for Gon {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("gon", Some("grade"))
    }
    fn primary_code(&self) -> &'static str {
        "gon"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("\u{25a1}<sup>g</sup>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.9,
            unit: "deg",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3_141_592_653_589_793, 200_000_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct Degree;
impl UcumUnit for Degree {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("degree", None)
    }
    fn primary_code(&self) -> &'static str {
        "deg"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("\u{b0}")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 2.0,
            unit: "[pi].rad/360",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3_141_592_653_589_793, 180_000_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MinuteTick;
impl UcumUnit for MinuteTick {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("minute", None)
    }
    fn primary_code(&self) -> &'static str {
        "'"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("'")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "deg/60",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3_141_592_653_589_793, 10_800_000_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SecondAngle;
impl UcumUnit for SecondAngle {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("second", None)
    }
    fn primary_code(&self) -> &'static str {
        "''"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("''")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "'/60",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3_141_592_653_589_793, 648_000_000_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LiterL;
impl UcumUnit for LiterL {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("liter", None)
    }
    fn primary_code(&self) -> &'static str {
        "l"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("l")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "dm3",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LiterUsL;
impl UcumUnit for LiterUsL {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("liter", None)
    }
    fn primary_code(&self) -> &'static str {
        "L"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("L")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "l",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Are;
impl UcumUnit for Are {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("are", None)
    }
    fn primary_code(&self) -> &'static str {
        "ar"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("a")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 100.0,
            unit: "m2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(100, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MinuteMin;
impl UcumUnit for MinuteMin {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("minute", None)
    }
    fn primary_code(&self) -> &'static str {
        "min"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("min")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 60.0,
            unit: "s",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(60, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Hour;
impl UcumUnit for Hour {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("hour", None)
    }
    fn primary_code(&self) -> &'static str {
        "h"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("h")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 60.0,
            unit: "min",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3600, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Day;
impl UcumUnit for Day {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("day", None)
    }
    fn primary_code(&self) -> &'static str {
        "d"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("d")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 24.0,
            unit: "h",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(86400, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct TropicalYear;
impl UcumUnit for TropicalYear {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("tropical year", None)
    }
    fn primary_code(&self) -> &'static str {
        "a_t"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("a<sub>t</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 365.24219,
            unit: "d",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3_944_615_652, 125)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MeanJulianYear;
impl UcumUnit for MeanJulianYear {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("mean Julian year", None)
    }
    fn primary_code(&self) -> &'static str {
        "a_j"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("a<sub>j</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 365.25,
            unit: "d",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(31_557_600, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MeanGregorianYear;
impl UcumUnit for MeanGregorianYear {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("mean Gregorian year", None)
    }
    fn primary_code(&self) -> &'static str {
        "a_g"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("a<sub>g</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 365.2425,
            unit: "d",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(31_556_952, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Year;
impl UcumUnit for Year {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("year", None)
    }
    fn primary_code(&self) -> &'static str {
        "a"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("a")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "a_j",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(31_557_600, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Week;
impl UcumUnit for Week {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("week", None)
    }
    fn primary_code(&self) -> &'static str {
        "wk"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("wk")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 7.0,
            unit: "d",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(604_800, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SynodalMonth;
impl UcumUnit for SynodalMonth {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("synodal month", None)
    }
    fn primary_code(&self) -> &'static str {
        "mo_s"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("mo<sub>s</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 29.53059,
            unit: "d",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(318_930_372, 125)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MeanJulianMonth;
impl UcumUnit for MeanJulianMonth {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("mean Julian month", None)
    }
    fn primary_code(&self) -> &'static str {
        "mo_j"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("mo<sub>j</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "a_j/12",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(2_629_800, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MeanGregorianMonth;
impl UcumUnit for MeanGregorianMonth {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("mean Gregorian month", None)
    }
    fn primary_code(&self) -> &'static str {
        "mo_g"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("mo<sub>g</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "a_g/12",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(2_629_746, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Month;
impl UcumUnit for Month {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("month", None)
    }
    fn primary_code(&self) -> &'static str {
        "mo"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("mo")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "mo_j",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(2_629_800, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Tonne;
impl UcumUnit for Tonne {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("tonne", None)
    }
    fn primary_code(&self) -> &'static str {
        "t"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("t")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1000.0,
            unit: "kg",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1_000_000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Bar;
impl UcumUnit for Bar {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("bar", None)
    }
    fn primary_code(&self) -> &'static str {
        "bar"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("bar")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 100_000.0,
            unit: "Pa",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(100_000_000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct UnifiedAtomicMassUnit;
impl UcumUnit for UnifiedAtomicMassUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("unified atomic mass unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "u"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("u")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.000_000_000_000_000_000_000_001_660_540_2,
            unit: "g",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(8_302_701, 5_000_000_000_000_000_000_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Electronvolt;
impl UcumUnit for Electronvolt {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("electronvolt", None)
    }
    fn primary_code(&self) -> &'static str {
        "eV"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("eV")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[e].V",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(160_217_733, 1_000_000_000_000_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AstronomicUnit;
impl UcumUnit for AstronomicUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("astronomic unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "AU"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("AU")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 149_597.870_691,
            unit: "Mm",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(149_597_870_691, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Parsec;
impl UcumUnit for Parsec {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("parsec", None)
    }
    fn primary_code(&self) -> &'static str {
        "pc"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("pc")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 30_856_780_000_000_000.0,
            unit: "m",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(30_856_780_000_000_000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct VelocityOfLight;
impl UcumUnit for VelocityOfLight {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("velocity of light", None)
    }
    fn primary_code(&self) -> &'static str {
        "[c]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("<i>c</i>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 299_792_458.0,
            unit: "m/s",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(299_792_458, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PlanckConstant;
impl UcumUnit for PlanckConstant {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Planck constant", None)
    }
    fn primary_code(&self) -> &'static str {
        "[h]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("<i>h</i>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.000_000_000_000_000_000_000_000_000_000_000_662_607_55,
            unit: "J.s",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(
            13_252_151,
            20_000_000_000_000_000_000_000_000_000_000_000_000,
        )
    }
}
#[derive(Debug, Clone, Copy)]
pub struct BoltzmannConstant;
impl UcumUnit for BoltzmannConstant {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Boltzmann constant", None)
    }
    fn primary_code(&self) -> &'static str {
        "[k]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("<i>k</i>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.000_000_000_000_000_000_000_013_806_58,
            unit: "J/K",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(690_329, 50_000_000_000_000_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct PermittivityOfVacuum;
impl UcumUnit for PermittivityOfVacuum {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("permittivity of vacuum", None)
    }
    fn primary_code(&self) -> &'static str {
        "[eps_0]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("<i>\u{3b5}<sub><r>0</r></sub></i>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-3) }),
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.000_000_000_008_854_187_817,
            unit: "F/m",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(8_854_187_817, 1_000_000_000_000_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct PermeabilityOfVacuum;
impl UcumUnit for PermeabilityOfVacuum {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("permeability of vacuum", None)
    }
    fn primary_code(&self) -> &'static str {
        "[mu_0]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("<i>\u{3bc}<sub><r>0</r></sub></i>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "4.[pi].10*-7.N/A2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3_141_592_653_589_793, 2_500_000_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ElementaryCharge;
impl UcumUnit for ElementaryCharge {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("elementary charge", None)
    }
    fn primary_code(&self) -> &'static str {
        "[e]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("<i>e</i>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.000_000_000_000_000_000_160_217_733,
            unit: "C",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(160_217_733, 1_000_000_000_000_000_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ElectronMass;
impl UcumUnit for ElectronMass {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("electron mass", None)
    }
    fn primary_code(&self) -> &'static str {
        "[m_e]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("<i>m<sub><r>e</r></sub></i>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.000_000_000_000_000_000_000_000_000_910_938_97,
            unit: "g",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(91_093_897, 100_000_000_000_000_000_000_000_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ProtonMass;
impl UcumUnit for ProtonMass {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("proton mass", None)
    }
    fn primary_code(&self) -> &'static str {
        "[m_p]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("<i>m<sub><r>p</r></sub></i>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.000_000_000_000_000_000_000_001_672_623_1,
            unit: "g",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(16_726_231, 10_000_000_000_000_000_000_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct NewtonianConstantOfGravitation;
impl UcumUnit for NewtonianConstantOfGravitation {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Newtonian constant of gravitation", None)
    }
    fn primary_code(&self) -> &'static str {
        "[G]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("<i>G</i>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.000_000_000_066_725_9,
            unit: "m3.kg-1.s-2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(667_259, 10_000_000_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StandardAccelerationOfFreeFall;
impl UcumUnit for StandardAccelerationOfFreeFall {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("standard acceleration of free fall", None)
    }
    fn primary_code(&self) -> &'static str {
        "[g]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("<i>g<sub>n</sub></i>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 9.80665,
            unit: "m/s2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(196_133, 20000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StandardAtmosphere;
impl UcumUnit for StandardAtmosphere {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("standard atmosphere", None)
    }
    fn primary_code(&self) -> &'static str {
        "atm"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("atm")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 101_325.0,
            unit: "Pa",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(101_325_000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LightYear;
impl UcumUnit for LightYear {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("light-year", None)
    }
    fn primary_code(&self) -> &'static str {
        "[ly]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("l.y.")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[c].a_j",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(9_460_730_472_580_800, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct GramForce;
impl UcumUnit for GramForce {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("gram-force", None)
    }
    fn primary_code(&self) -> &'static str {
        "gf"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("gf")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "g.[g]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(196_133, 20000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PoundForce;
impl UcumUnit for PoundForce {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("pound force", None)
    }
    fn primary_code(&self) -> &'static str {
        "[lbf_av]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("lbf")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[lb_av].[g]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(8_896_443_230_521, 2_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Kayser;
impl UcumUnit for Kayser {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Kayser", None)
    }
    fn primary_code(&self) -> &'static str {
        "Ky"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("K")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "cm-1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(100, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Gal;
impl UcumUnit for Gal {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Gal", None)
    }
    fn primary_code(&self) -> &'static str {
        "Gal"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Gal")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "cm/s2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 100)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Dyne;
impl UcumUnit for Dyne {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("dyne", None)
    }
    fn primary_code(&self) -> &'static str {
        "dyn"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("dyn")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "g.cm/s2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 100)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Erg;
impl UcumUnit for Erg {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("erg", None)
    }
    fn primary_code(&self) -> &'static str {
        "erg"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("erg")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "dyn.cm",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 10000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Poise;
impl UcumUnit for Poise {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Poise", None)
    }
    fn primary_code(&self) -> &'static str {
        "P"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("P")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "dyn.s/cm2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(100, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Biot;
impl UcumUnit for Biot {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Biot", None)
    }
    fn primary_code(&self) -> &'static str {
        "Bi"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Bi")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 10.0,
            unit: "A",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(10, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Stokes;
impl UcumUnit for Stokes {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Stokes", None)
    }
    fn primary_code(&self) -> &'static str {
        "St"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("St")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "cm2/s",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 10000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Maxwell;
impl UcumUnit for Maxwell {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Maxwell", None)
    }
    fn primary_code(&self) -> &'static str {
        "Mx"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Mx")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.000_000_01,
            unit: "Wb",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 100_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Gauss;
impl UcumUnit for Gauss {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Gauss", None)
    }
    fn primary_code(&self) -> &'static str {
        "G"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Gs")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.0001,
            unit: "T",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 10)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Oersted;
impl UcumUnit for Oersted {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Oersted", None)
    }
    fn primary_code(&self) -> &'static str {
        "Oe"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Oe")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 250.0,
            unit: "/[pi].A/m",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(250_000_000_000_000_000, 3_141_592_653_589_793)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Gilbert;
impl UcumUnit for Gilbert {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Gilbert", None)
    }
    fn primary_code(&self) -> &'static str {
        "Gb"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Gb")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "Oe.cm",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(2_500_000_000_000_000, 3_141_592_653_589_793)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Stilb;
impl UcumUnit for Stilb {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("stilb", None)
    }
    fn primary_code(&self) -> &'static str {
        "sb"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("sb")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            None,
            None,
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "cd/cm2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(10000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Lambert;
impl UcumUnit for Lambert {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Lambert", None)
    }
    fn primary_code(&self) -> &'static str {
        "Lmb"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("L")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            None,
            None,
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "cd/cm2/[pi]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3_141_592_653_589_793, 100_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Phot;
impl UcumUnit for Phot {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("phot", None)
    }
    fn primary_code(&self) -> &'static str {
        "ph"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("ph")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.0001,
            unit: "lx",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 10000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Curie;
impl UcumUnit for Curie {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Curie", None)
    }
    fn primary_code(&self) -> &'static str {
        "Ci"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Ci")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 37_000_000_000.0,
            unit: "Bq",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(37_000_000_000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Roentgen;
impl UcumUnit for Roentgen {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Roentgen", None)
    }
    fn primary_code(&self) -> &'static str {
        "R"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("R")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.000_258,
            unit: "C/kg",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(129, 500_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct RadiationAbsorbedDose;
impl UcumUnit for RadiationAbsorbedDose {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("radiation absorbed dose", None)
    }
    fn primary_code(&self) -> &'static str {
        "RAD"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("RAD")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 100.0,
            unit: "erg/g",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 100)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct RadiationEquivalentMan;
impl UcumUnit for RadiationEquivalentMan {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("radiation equivalent man", None)
    }
    fn primary_code(&self) -> &'static str {
        "REM"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("REM")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "RAD",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 100)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct InchInI;
impl UcumUnit for InchInI {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("inch", None)
    }
    fn primary_code(&self) -> &'static str {
        "[in_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("in")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 2.54,
            unit: "cm",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(127, 5000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FootFtI;
impl UcumUnit for FootFtI {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("foot", None)
    }
    fn primary_code(&self) -> &'static str {
        "[ft_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("ft")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 12.0,
            unit: "[in_i]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(381, 1250)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct YardYdI;
impl UcumUnit for YardYdI {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("yard", None)
    }
    fn primary_code(&self) -> &'static str {
        "[yd_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("yd")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 3.0,
            unit: "[ft_i]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1143, 1250)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MileMiI;
impl UcumUnit for MileMiI {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("mile", None)
    }
    fn primary_code(&self) -> &'static str {
        "[mi_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("mi")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 5280.0,
            unit: "[ft_i]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(201_168, 125)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FathomFthI;
impl UcumUnit for FathomFthI {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("fathom", None)
    }
    fn primary_code(&self) -> &'static str {
        "[fth_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("fth")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 6.0,
            unit: "[ft_i]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1143, 625)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct NauticalMileNmiI;
impl UcumUnit for NauticalMileNmiI {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("nautical mile", None)
    }
    fn primary_code(&self) -> &'static str {
        "[nmi_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("n.mi")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1852.0,
            unit: "m",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1852, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct KnotKnI;
impl UcumUnit for KnotKnI {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("knot", None)
    }
    fn primary_code(&self) -> &'static str {
        "[kn_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("knot")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[nmi_i]/h",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(463, 900)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SquareInch;
impl UcumUnit for SquareInch {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("square inch", None)
    }
    fn primary_code(&self) -> &'static str {
        "[sin_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[in_i]2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(16129, 25_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SquareFoot;
impl UcumUnit for SquareFoot {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("square foot", None)
    }
    fn primary_code(&self) -> &'static str {
        "[sft_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[ft_i]2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(145_161, 1_562_500)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SquareYard;
impl UcumUnit for SquareYard {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("square yard", None)
    }
    fn primary_code(&self) -> &'static str {
        "[syd_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[yd_i]2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1_306_449, 1_562_500)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CubicInch;
impl UcumUnit for CubicInch {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("cubic inch", None)
    }
    fn primary_code(&self) -> &'static str {
        "[cin_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[in_i]3",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(2_048_383, 125_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CubicFoot;
impl UcumUnit for CubicFoot {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("cubic foot", None)
    }
    fn primary_code(&self) -> &'static str {
        "[cft_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[ft_i]3",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(55_306_341, 1_953_125_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CubicYard;
impl UcumUnit for CubicYard {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("cubic yard", None)
    }
    fn primary_code(&self) -> &'static str {
        "[cyd_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("cu.yd")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[yd_i]3",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1_493_271_207, 1_953_125_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct BoardFoot;
impl UcumUnit for BoardFoot {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("board foot", None)
    }
    fn primary_code(&self) -> &'static str {
        "[bf_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 144.0,
            unit: "[in_i]3",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(18_435_447, 7_812_500_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CordCrI;
impl UcumUnit for CordCrI {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("cord", None)
    }
    fn primary_code(&self) -> &'static str {
        "[cr_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 128.0,
            unit: "[ft_i]3",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(884_901_456, 244_140_625)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MilMilI;
impl UcumUnit for MilMilI {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("mil", None)
    }
    fn primary_code(&self) -> &'static str {
        "[mil_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("mil")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.001,
            unit: "[in_i]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(127, 5_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CircularMil;
impl UcumUnit for CircularMil {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("circular mil", None)
    }
    fn primary_code(&self) -> &'static str {
        "[cml_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("circ.mil")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[pi]/4.[mil_i]2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3_141_592_653_589_793, 2_580_640)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Hand;
impl UcumUnit for Hand {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("hand", None)
    }
    fn primary_code(&self) -> &'static str {
        "[hd_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("hd")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 4.0,
            unit: "[in_i]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(127, 1250)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FootFtUs;
impl UcumUnit for FootFtUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("foot", None)
    }
    fn primary_code(&self) -> &'static str {
        "[ft_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("ft<sub>us</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1200.0,
            unit: "m/3937",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1200, 3937)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct YardYdUs;
impl UcumUnit for YardYdUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("yard", None)
    }
    fn primary_code(&self) -> &'static str {
        "[yd_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 3.0,
            unit: "[ft_us]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3600, 3937)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct InchInUs;
impl UcumUnit for InchInUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("inch", None)
    }
    fn primary_code(&self) -> &'static str {
        "[in_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[ft_us]/12",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(100, 3937)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct RodRdUs;
impl UcumUnit for RodRdUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("rod", None)
    }
    fn primary_code(&self) -> &'static str {
        "[rd_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 16.5,
            unit: "[ft_us]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(19800, 3937)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct GuntersChainChUs;
impl UcumUnit for GuntersChainChUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Gunter's chain", Some("Surveyor's chain"))
    }
    fn primary_code(&self) -> &'static str {
        "[ch_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 4.0,
            unit: "[rd_us]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(79200, 3937)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LinkForGuntersChainLkUs;
impl UcumUnit for LinkForGuntersChainLkUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("link for Gunter's chain", None)
    }
    fn primary_code(&self) -> &'static str {
        "[lk_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[ch_us]/100",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(792, 3937)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct RamdensChain;
impl UcumUnit for RamdensChain {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Ramden's chain", Some("Engineer's chain"))
    }
    fn primary_code(&self) -> &'static str {
        "[rch_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 100.0,
            unit: "[ft_us]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(120_000, 3937)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LinkForRamdensChain;
impl UcumUnit for LinkForRamdensChain {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("link for Ramden's chain", None)
    }
    fn primary_code(&self) -> &'static str {
        "[rlk_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[rch_us]/100",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1200, 3937)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FathomFthUs;
impl UcumUnit for FathomFthUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("fathom", None)
    }
    fn primary_code(&self) -> &'static str {
        "[fth_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 6.0,
            unit: "[ft_us]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(7200, 3937)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Furlong;
impl UcumUnit for Furlong {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("furlong", None)
    }
    fn primary_code(&self) -> &'static str {
        "[fur_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 40.0,
            unit: "[rd_us]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(792_000, 3937)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MileMiUs;
impl UcumUnit for MileMiUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("mile", None)
    }
    fn primary_code(&self) -> &'static str {
        "[mi_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 8.0,
            unit: "[fur_us]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(6_336_000, 3937)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AcreAcrUs;
impl UcumUnit for AcreAcrUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("acre", None)
    }
    fn primary_code(&self) -> &'static str {
        "[acr_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 160.0,
            unit: "[rd_us]2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(62_726_400_000, 15_499_969)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SquareRod;
impl UcumUnit for SquareRod {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("square rod", None)
    }
    fn primary_code(&self) -> &'static str {
        "[srd_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[rd_us]2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(392_040_000, 15_499_969)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SquareMile;
impl UcumUnit for SquareMile {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("square mile", None)
    }
    fn primary_code(&self) -> &'static str {
        "[smi_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[mi_us]2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(40_144_896_000_000, 15_499_969)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Section;
impl UcumUnit for Section {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("section", None)
    }
    fn primary_code(&self) -> &'static str {
        "[sct]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[mi_us]2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(40_144_896_000_000, 15_499_969)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Township;
impl UcumUnit for Township {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("township", None)
    }
    fn primary_code(&self) -> &'static str {
        "[twp]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 36.0,
            unit: "[sct]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1_445_216_256_000_000, 15_499_969)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MilMilUs;
impl UcumUnit for MilMilUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("mil", None)
    }
    fn primary_code(&self) -> &'static str {
        "[mil_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.001,
            unit: "[in_us]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 39370)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct InchInBr;
impl UcumUnit for InchInBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("inch", None)
    }
    fn primary_code(&self) -> &'static str {
        "[in_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 2.539_998,
            unit: "cm",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1_269_999, 50_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FootFtBr;
impl UcumUnit for FootFtBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("foot", None)
    }
    fn primary_code(&self) -> &'static str {
        "[ft_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 12.0,
            unit: "[in_br]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3_809_997, 12_500_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct RodRdBr;
impl UcumUnit for RodRdBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("rod", None)
    }
    fn primary_code(&self) -> &'static str {
        "[rd_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 16.5,
            unit: "[ft_br]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(125_729_901, 25_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct GuntersChainChBr;
impl UcumUnit for GuntersChainChBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Gunter's chain", None)
    }
    fn primary_code(&self) -> &'static str {
        "[ch_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 4.0,
            unit: "[rd_br]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(125_729_901, 6_250_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LinkForGuntersChainLkBr;
impl UcumUnit for LinkForGuntersChainLkBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("link for Gunter's chain", None)
    }
    fn primary_code(&self) -> &'static str {
        "[lk_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[ch_br]/100",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(125_729_901, 625_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FathomFthBr;
impl UcumUnit for FathomFthBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("fathom", None)
    }
    fn primary_code(&self) -> &'static str {
        "[fth_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 6.0,
            unit: "[ft_br]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(11_429_991, 6_250_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Pace;
impl UcumUnit for Pace {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("pace", None)
    }
    fn primary_code(&self) -> &'static str {
        "[pc_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 2.5,
            unit: "[ft_br]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3_809_997, 5_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct YardYdBr;
impl UcumUnit for YardYdBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("yard", None)
    }
    fn primary_code(&self) -> &'static str {
        "[yd_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 3.0,
            unit: "[ft_br]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(11_429_991, 12_500_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MileMiBr;
impl UcumUnit for MileMiBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("mile", None)
    }
    fn primary_code(&self) -> &'static str {
        "[mi_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 5280.0,
            unit: "[ft_br]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(125_729_901, 78125)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct NauticalMileNmiBr;
impl UcumUnit for NauticalMileNmiBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("nautical mile", None)
    }
    fn primary_code(&self) -> &'static str {
        "[nmi_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 6080.0,
            unit: "[ft_br]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(144_779_886, 78125)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct KnotKnBr;
impl UcumUnit for KnotKnBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("knot", None)
    }
    fn primary_code(&self) -> &'static str {
        "[kn_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[nmi_br]/h",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(8_043_327, 15_625_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AcreAcrBr;
impl UcumUnit for AcreAcrBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("acre", None)
    }
    fn primary_code(&self) -> &'static str {
        "[acr_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 4840.0,
            unit: "[yd_br]2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(15_808_008_005_469_801, 3_906_250_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct QueenAnnesWineGallon;
impl UcumUnit for QueenAnnesWineGallon {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Queen Anne's wine gallon", None)
    }
    fn primary_code(&self) -> &'static str {
        "[gal_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 231.0,
            unit: "[in_i]3",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(473_176_473, 125_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Barrel;
impl UcumUnit for Barrel {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("barrel", None)
    }
    fn primary_code(&self) -> &'static str {
        "[bbl_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 42.0,
            unit: "[gal_us]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(9_936_705_933, 62_500_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct QuartQtUs;
impl UcumUnit for QuartQtUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("quart", None)
    }
    fn primary_code(&self) -> &'static str {
        "[qt_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[gal_us]/4",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(473_176_473, 500_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PintPtUs;
impl UcumUnit for PintPtUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("pint", None)
    }
    fn primary_code(&self) -> &'static str {
        "[pt_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[qt_us]/2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(473_176_473, 1_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct GillGilUs;
impl UcumUnit for GillGilUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("gill", None)
    }
    fn primary_code(&self) -> &'static str {
        "[gil_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[pt_us]/4",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(473_176_473, 4_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FluidOunceFozUs;
impl UcumUnit for FluidOunceFozUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("fluid ounce", None)
    }
    fn primary_code(&self) -> &'static str {
        "[foz_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("oz fl")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[gil_us]/4",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(473_176_473, 16_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FluidDramFdrUs;
impl UcumUnit for FluidDramFdrUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("fluid dram", None)
    }
    fn primary_code(&self) -> &'static str {
        "[fdr_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[foz_us]/8",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(473_176_473, 128_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MinimMinUs;
impl UcumUnit for MinimMinUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("minim", None)
    }
    fn primary_code(&self) -> &'static str {
        "[min_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[fdr_us]/60",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(157_725_491, 2_560_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CordCrdUs;
impl UcumUnit for CordCrdUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("cord", None)
    }
    fn primary_code(&self) -> &'static str {
        "[crd_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 128.0,
            unit: "[ft_i]3",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(884_901_456, 244_140_625)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct BushelBuUs;
impl UcumUnit for BushelBuUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("bushel", None)
    }
    fn primary_code(&self) -> &'static str {
        "[bu_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 2150.42,
            unit: "[in_i]3",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(220_244_188_543, 6_250_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HistoricalWinchesterGallon;
impl UcumUnit for HistoricalWinchesterGallon {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("historical winchester gallon", None)
    }
    fn primary_code(&self) -> &'static str {
        "[gal_wi]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[bu_us]/8",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(220_244_188_543, 50_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PeckPkUs;
impl UcumUnit for PeckPkUs {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("peck", None)
    }
    fn primary_code(&self) -> &'static str {
        "[pk_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[bu_us]/4",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(220_244_188_543, 25_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DryQuart;
impl UcumUnit for DryQuart {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("dry quart", None)
    }
    fn primary_code(&self) -> &'static str {
        "[dqt_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[pk_us]/8",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(220_244_188_543, 200_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DryPint;
impl UcumUnit for DryPint {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("dry pint", None)
    }
    fn primary_code(&self) -> &'static str {
        "[dpt_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[dqt_us]/2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(220_244_188_543, 400_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Tablespoon;
impl UcumUnit for Tablespoon {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("tablespoon", None)
    }
    fn primary_code(&self) -> &'static str {
        "[tbs_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[foz_us]/2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(473_176_473, 32_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Teaspoon;
impl UcumUnit for Teaspoon {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("teaspoon", None)
    }
    fn primary_code(&self) -> &'static str {
        "[tsp_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[tbs_us]/3",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(157_725_491, 32_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Cup;
impl UcumUnit for Cup {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("cup", None)
    }
    fn primary_code(&self) -> &'static str {
        "[cup_us]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 16.0,
            unit: "[tbs_us]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(473_176_473, 2_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MetricFluidOunce;
impl UcumUnit for MetricFluidOunce {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("metric fluid ounce", None)
    }
    fn primary_code(&self) -> &'static str {
        "[foz_m]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("oz fl")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 30.0,
            unit: "mL",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3, 100_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MetricCup;
impl UcumUnit for MetricCup {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("metric cup", None)
    }
    fn primary_code(&self) -> &'static str {
        "[cup_m]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 240.0,
            unit: "mL",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3, 12500)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MetricTeaspoon;
impl UcumUnit for MetricTeaspoon {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("metric teaspoon", None)
    }
    fn primary_code(&self) -> &'static str {
        "[tsp_m]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 5.0,
            unit: "mL",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 200_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MetricTablespoon;
impl UcumUnit for MetricTablespoon {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("metric tablespoon", None)
    }
    fn primary_code(&self) -> &'static str {
        "[tbs_m]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 15.0,
            unit: "mL",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3, 200_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Gallon;
impl UcumUnit for Gallon {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("gallon", None)
    }
    fn primary_code(&self) -> &'static str {
        "[gal_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 4.54609,
            unit: "l",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(454_609, 100_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PeckPkBr;
impl UcumUnit for PeckPkBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("peck", None)
    }
    fn primary_code(&self) -> &'static str {
        "[pk_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 2.0,
            unit: "[gal_br]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(454_609, 50_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct BushelBuBr;
impl UcumUnit for BushelBuBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("bushel", None)
    }
    fn primary_code(&self) -> &'static str {
        "[bu_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 4.0,
            unit: "[pk_br]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(454_609, 12_500_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct QuartQtBr;
impl UcumUnit for QuartQtBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("quart", None)
    }
    fn primary_code(&self) -> &'static str {
        "[qt_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[gal_br]/4",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(454_609, 400_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PintPtBr;
impl UcumUnit for PintPtBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("pint", None)
    }
    fn primary_code(&self) -> &'static str {
        "[pt_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[qt_br]/2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(454_609, 800_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct GillGilBr;
impl UcumUnit for GillGilBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("gill", None)
    }
    fn primary_code(&self) -> &'static str {
        "[gil_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[pt_br]/4",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(454_609, 3_200_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FluidOunceFozBr;
impl UcumUnit for FluidOunceFozBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("fluid ounce", None)
    }
    fn primary_code(&self) -> &'static str {
        "[foz_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[gil_br]/5",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(454_609, 16_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FluidDramFdrBr;
impl UcumUnit for FluidDramFdrBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("fluid dram", None)
    }
    fn primary_code(&self) -> &'static str {
        "[fdr_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[foz_br]/8",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(454_609, 128_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MinimMinBr;
impl UcumUnit for MinimMinBr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("minim", None)
    }
    fn primary_code(&self) -> &'static str {
        "[min_br]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[fdr_br]/60",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(454_609, 7_680_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Grain;
impl UcumUnit for Grain {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("grain", None)
    }
    fn primary_code(&self) -> &'static str {
        "[gr]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 64.79891,
            unit: "mg",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(6_479_891, 100_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PoundLbAv;
impl UcumUnit for PoundLbAv {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("pound", None)
    }
    fn primary_code(&self) -> &'static str {
        "[lb_av]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("lb")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 7000.0,
            unit: "[gr]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(45_359_237, 100_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct OunceOzAv;
impl UcumUnit for OunceOzAv {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("ounce", None)
    }
    fn primary_code(&self) -> &'static str {
        "[oz_av]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("oz")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[lb_av]/16",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(45_359_237, 1_600_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DramDrAv;
impl UcumUnit for DramDrAv {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("dram", None)
    }
    fn primary_code(&self) -> &'static str {
        "[dr_av]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[oz_av]/16",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(45_359_237, 25_600_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ShortHundredweight;
impl UcumUnit for ShortHundredweight {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("short hundredweight", Some("U.S. hundredweight"))
    }
    fn primary_code(&self) -> &'static str {
        "[scwt_av]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 100.0,
            unit: "[lb_av]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(45_359_237, 1000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LongHunderdweight;
impl UcumUnit for LongHunderdweight {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("long hunderdweight", Some("British hundredweight"))
    }
    fn primary_code(&self) -> &'static str {
        "[lcwt_av]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 112.0,
            unit: "[lb_av]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(317_514_659, 6250)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ShortTon;
impl UcumUnit for ShortTon {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("short ton", Some("U.S. ton"))
    }
    fn primary_code(&self) -> &'static str {
        "[ston_av]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 20.0,
            unit: "[scwt_av]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(45_359_237, 50)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LongTon;
impl UcumUnit for LongTon {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("long ton", Some("British ton"))
    }
    fn primary_code(&self) -> &'static str {
        "[lton_av]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 20.0,
            unit: "[lcwt_av]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(635_029_318, 625)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Stone;
impl UcumUnit for Stone {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("stone", Some("British stone"))
    }
    fn primary_code(&self) -> &'static str {
        "[stone_av]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 14.0,
            unit: "[lb_av]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(317_514_659, 50000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Pennyweight;
impl UcumUnit for Pennyweight {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("pennyweight", None)
    }
    fn primary_code(&self) -> &'static str {
        "[pwt_tr]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 24.0,
            unit: "[gr]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(19_439_673, 12_500_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct OunceOzTr;
impl UcumUnit for OunceOzTr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("ounce", None)
    }
    fn primary_code(&self) -> &'static str {
        "[oz_tr]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 20.0,
            unit: "[pwt_tr]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(19_439_673, 625_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PoundLbTr;
impl UcumUnit for PoundLbTr {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("pound", None)
    }
    fn primary_code(&self) -> &'static str {
        "[lb_tr]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 12.0,
            unit: "[oz_tr]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(58_319_019, 156_250)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Scruple;
impl UcumUnit for Scruple {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("scruple", None)
    }
    fn primary_code(&self) -> &'static str {
        "[sc_ap]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 20.0,
            unit: "[gr]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(6_479_891, 5_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DramDrAp;
impl UcumUnit for DramDrAp {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("dram", Some("drachm"))
    }
    fn primary_code(&self) -> &'static str {
        "[dr_ap]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 3.0,
            unit: "[sc_ap]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(19_439_673, 5_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct OunceOzAp;
impl UcumUnit for OunceOzAp {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("ounce", None)
    }
    fn primary_code(&self) -> &'static str {
        "[oz_ap]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 8.0,
            unit: "[dr_ap]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(19_439_673, 625_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PoundLbAp;
impl UcumUnit for PoundLbAp {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("pound", None)
    }
    fn primary_code(&self) -> &'static str {
        "[lb_ap]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 12.0,
            unit: "[oz_ap]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(58_319_019, 156_250)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MetricOunce;
impl UcumUnit for MetricOunce {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("metric ounce", None)
    }
    fn primary_code(&self) -> &'static str {
        "[oz_m]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 28.0,
            unit: "g",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(28, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Line;
impl UcumUnit for Line {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("line", None)
    }
    fn primary_code(&self) -> &'static str {
        "[lne]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[in_i]/12",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(127, 60000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Point;
impl UcumUnit for Point {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("point", None)
    }
    fn primary_code(&self) -> &'static str {
        "[pnt]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[lne]/6",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(127, 360_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Pica;
impl UcumUnit for Pica {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("pica", None)
    }
    fn primary_code(&self) -> &'static str {
        "[pca]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 12.0,
            unit: "[pnt]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(127, 30000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PrintersPoint;
impl UcumUnit for PrintersPoint {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Printer's point", None)
    }
    fn primary_code(&self) -> &'static str {
        "[pnt_pr]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.013_837,
            unit: "[in_i]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1_757_299, 5_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PrintersPica;
impl UcumUnit for PrintersPica {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Printer's pica", None)
    }
    fn primary_code(&self) -> &'static str {
        "[pca_pr]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 12.0,
            unit: "[pnt_pr]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(5_271_897, 1_250_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Pied;
impl UcumUnit for Pied {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("pied", Some("French foot"))
    }
    fn primary_code(&self) -> &'static str {
        "[pied]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 32.48,
            unit: "cm",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(203, 625)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Pouce;
impl UcumUnit for Pouce {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("pouce", Some("French inch"))
    }
    fn primary_code(&self) -> &'static str {
        "[pouce]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[pied]/12",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(203, 7500)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Ligne;
impl UcumUnit for Ligne {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("ligne", Some("French line"))
    }
    fn primary_code(&self) -> &'static str {
        "[ligne]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[pouce]/12",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(203, 90000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Didot;
impl UcumUnit for Didot {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("didot", Some("Didot's point"))
    }
    fn primary_code(&self) -> &'static str {
        "[didot]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[ligne]/6",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(203, 540_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Cicero;
impl UcumUnit for Cicero {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("cicero", Some("Didot's pica"))
    }
    fn primary_code(&self) -> &'static str {
        "[cicero]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 12.0,
            unit: "[didot]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(203, 45000)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct DegreeRankine;
impl UcumUnit for DegreeRankine {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("degree Rankine", None)
    }
    fn primary_code(&self) -> &'static str {
        "[degR]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("\u{b0}R")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 5.0,
            unit: "K/9",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(5, 9)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct CalorieAt15DegC;
impl UcumUnit for CalorieAt15DegC {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("calorie at 15 \u{b0}C", None)
    }
    fn primary_code(&self) -> &'static str {
        "cal_[15]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("cal<sub>15\u{b0}C</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 4.1858,
            unit: "J",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(20929, 5)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct CalorieAt20DegC;
impl UcumUnit for CalorieAt20DegC {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("calorie at 20 \u{b0}C", None)
    }
    fn primary_code(&self) -> &'static str {
        "cal_[20]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("cal<sub>20\u{b0}C</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 4.1819,
            unit: "J",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(41819, 10)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MeanCalorie;
impl UcumUnit for MeanCalorie {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("mean calorie", None)
    }
    fn primary_code(&self) -> &'static str {
        "cal_m"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("cal<sub>m</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 4.19002,
            unit: "J",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(209_501, 50)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct InternationalTableCalorie;
impl UcumUnit for InternationalTableCalorie {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("international table calorie", None)
    }
    fn primary_code(&self) -> &'static str {
        "cal_IT"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("cal<sub>IT</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 4.1868,
            unit: "J",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(20934, 5)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ThermochemicalCalorie;
impl UcumUnit for ThermochemicalCalorie {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("thermochemical calorie", None)
    }
    fn primary_code(&self) -> &'static str {
        "cal_th"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("cal<sub>th</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 4.184,
            unit: "J",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(4184, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Calorie;
impl UcumUnit for Calorie {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("calorie", None)
    }
    fn primary_code(&self) -> &'static str {
        "cal"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("cal")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "cal_th",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(4184, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct NutritionLabelCalories;
impl UcumUnit for NutritionLabelCalories {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("nutrition label Calories", None)
    }
    fn primary_code(&self) -> &'static str {
        "[Cal]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Cal")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "kcal_th",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(4_184_000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct BritishThermalUnitAt39DegF;
impl UcumUnit for BritishThermalUnitAt39DegF {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("British thermal unit at 39 \u{b0}F", None)
    }
    fn primary_code(&self) -> &'static str {
        "[Btu_39]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Btu<sub>39\u{b0}F</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.05967,
            unit: "kJ",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1_059_670, 1)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct BritishThermalUnitAt59DegF;
impl UcumUnit for BritishThermalUnitAt59DegF {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("British thermal unit at 59 \u{b0}F", None)
    }
    fn primary_code(&self) -> &'static str {
        "[Btu_59]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Btu<sub>59\u{b0}F</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0548,
            unit: "kJ",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1_054_800, 1)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct BritishThermalUnitAt60DegF;
impl UcumUnit for BritishThermalUnitAt60DegF {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("British thermal unit at 60 \u{b0}F", None)
    }
    fn primary_code(&self) -> &'static str {
        "[Btu_60]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Btu<sub>60\u{b0}F</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.05468,
            unit: "kJ",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1_054_680, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MeanBritishThermalUnit;
impl UcumUnit for MeanBritishThermalUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("mean British thermal unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[Btu_m]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Btu<sub>m</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.05587,
            unit: "kJ",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1_055_870, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct InternationalTableBritishThermalUnit;
impl UcumUnit for InternationalTableBritishThermalUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("international table British thermal unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[Btu_IT]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Btu<sub>IT</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.055_055_852_62,
            unit: "kJ",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(52_752_792_631, 50000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ThermochemicalBritishThermalUnit;
impl UcumUnit for ThermochemicalBritishThermalUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("thermochemical British thermal unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[Btu_th]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Btu<sub>th</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.05435,
            unit: "kJ",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1_054_350, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct BritishThermalUnit;
impl UcumUnit for BritishThermalUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("British thermal unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[Btu]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("btu")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[Btu_th]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1_054_350, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Horsepower;
impl UcumUnit for Horsepower {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("horsepower", None)
    }
    fn primary_code(&self) -> &'static str {
        "[HP]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            Some(unsafe { NonZeroI8::new_unchecked(-3) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 550.0,
            unit: "[ft_i].[lbf_av]/s",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(37_284_993_579_113_511, 50_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Tex;
impl UcumUnit for Tex {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("tex", None)
    }
    fn primary_code(&self) -> &'static str {
        "tex"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("tex")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "g/km",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Denier;
impl UcumUnit for Denier {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Denier", None)
    }
    fn primary_code(&self) -> &'static str {
        "[den]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("den")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "g/9/km",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1000, 9)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MeterOfWaterColumn;
impl UcumUnit for MeterOfWaterColumn {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("meter of water column", None)
    }
    fn primary_code(&self) -> &'static str {
        "m[H2O]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("m H<sub><r>2</r></sub>O")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 9.80665,
            unit: "kPa",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(9_806_650, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MeterOfMercuryColumn;
impl UcumUnit for MeterOfMercuryColumn {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("meter of mercury column", None)
    }
    fn primary_code(&self) -> &'static str {
        "m[Hg]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("m Hg")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 133.322,
            unit: "kPa",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(133_322_000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct InchOfWaterColumn;
impl UcumUnit for InchOfWaterColumn {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("inch of water column", None)
    }
    fn primary_code(&self) -> &'static str {
        "[in_i'H2O]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("in H<sub><r>2</r></sub>O")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "m[H2O].[in_i]/m",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(24_908_891, 100)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct InchOfMercuryColumn;
impl UcumUnit for InchOfMercuryColumn {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("inch of mercury column", None)
    }
    fn primary_code(&self) -> &'static str {
        "[in_i'Hg]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("in Hg")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "m[Hg].[in_i]/m",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(16_931_894, 5)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PeripheralVascularResistanceUnit;
impl UcumUnit for PeripheralVascularResistanceUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("peripheral vascular resistance unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[PRU]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("P.R.U.")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-4) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "mm[Hg].s/ml",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(133_322_000_000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct WoodUnit;
impl UcumUnit for WoodUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Wood unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[wood'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Wood U.")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-4) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "mm[Hg].min/L",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(7_999_320_000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Diopter;
impl UcumUnit for Diopter {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("diopter", None)
    }
    fn primary_code(&self) -> &'static str {
        "[diop]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("dpt")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "/m",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Mesh;
impl UcumUnit for Mesh {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("mesh", None)
    }
    fn primary_code(&self) -> &'static str {
        "[mesh_i]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "/[in_i]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(5000, 127)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct Charriere;
impl UcumUnit for Charriere {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Charri\u{e8}re", Some("french"))
    }
    fn primary_code(&self) -> &'static str {
        "[Ch]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Ch")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "mm/3",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 3000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Drop;
impl UcumUnit for Drop {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("drop", None)
    }
    fn primary_code(&self) -> &'static str {
        "[drp]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("drp")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "ml/20",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 20_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HounsfieldUnit;
impl UcumUnit for HounsfieldUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Hounsfield unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[hnsf'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("HF")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MetabolicEquivalent;
impl UcumUnit for MetabolicEquivalent {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("metabolic equivalent", None)
    }
    fn primary_code(&self) -> &'static str {
        "[MET]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("MET")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 3.5,
            unit: "mL/min/kg",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(7, 120_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HomeopathicPotencyOfDecimalHahnemannianSeries;
impl UcumUnit for HomeopathicPotencyOfDecimalHahnemannianSeries {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("homeopathic potency of decimal hahnemannian series", None)
    }
    fn primary_code(&self) -> &'static str {
        "[hp_X]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("X")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HomeopathicPotencyOfCentesimalHahnemannianSeries;
impl UcumUnit for HomeopathicPotencyOfCentesimalHahnemannianSeries {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        (
            "homeopathic potency of centesimal hahnemannian series",
            None,
        )
    }
    fn primary_code(&self) -> &'static str {
        "[hp_C]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("C")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HomeopathicPotencyOfMillesimalHahnemannianSeries;
impl UcumUnit for HomeopathicPotencyOfMillesimalHahnemannianSeries {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        (
            "homeopathic potency of millesimal hahnemannian series",
            None,
        )
    }
    fn primary_code(&self) -> &'static str {
        "[hp_M]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("M")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HomeopathicPotencyOfQuintamillesimalHahnemannianSeries;
impl UcumUnit for HomeopathicPotencyOfQuintamillesimalHahnemannianSeries {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        (
            "homeopathic potency of quintamillesimal hahnemannian series",
            None,
        )
    }
    fn primary_code(&self) -> &'static str {
        "[hp_Q]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Q")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HomeopathicPotencyOfDecimalKorsakovianSeries;
impl UcumUnit for HomeopathicPotencyOfDecimalKorsakovianSeries {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("homeopathic potency of decimal korsakovian series", None)
    }
    fn primary_code(&self) -> &'static str {
        "[kp_X]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("X")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HomeopathicPotencyOfCentesimalKorsakovianSeries;
impl UcumUnit for HomeopathicPotencyOfCentesimalKorsakovianSeries {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("homeopathic potency of centesimal korsakovian series", None)
    }
    fn primary_code(&self) -> &'static str {
        "[kp_C]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("C")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HomeopathicPotencyOfMillesimalKorsakovianSeries;
impl UcumUnit for HomeopathicPotencyOfMillesimalKorsakovianSeries {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("homeopathic potency of millesimal korsakovian series", None)
    }
    fn primary_code(&self) -> &'static str {
        "[kp_M]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("M")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HomeopathicPotencyOfQuintamillesimalKorsakovianSeries;
impl UcumUnit for HomeopathicPotencyOfQuintamillesimalKorsakovianSeries {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        (
            "homeopathic potency of quintamillesimal korsakovian series",
            None,
        )
    }
    fn primary_code(&self) -> &'static str {
        "[kp_Q]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Q")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Equivalents;
impl UcumUnit for Equivalents {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("equivalents", None)
    }
    fn primary_code(&self) -> &'static str {
        "eq"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("eq")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "mol",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(602_213_670_000_000_000_000_000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Osmole;
impl UcumUnit for Osmole {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("osmole", None)
    }
    fn primary_code(&self) -> &'static str {
        "osm"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("osm")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "mol",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(602_213_670_000_000_000_000_000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct GramPercent;
impl UcumUnit for GramPercent {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("gram percent", None)
    }
    fn primary_code(&self) -> &'static str {
        "g%"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("g%")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-3) }),
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "g/dl",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(10000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SvedbergUnit;
impl UcumUnit for SvedbergUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Svedberg unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[S]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("S")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "10*-13.s",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 10_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HighPowerField;
impl UcumUnit for HighPowerField {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("high power field", None)
    }
    fn primary_code(&self) -> &'static str {
        "[HPF]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("HPF")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LowPowerField;
impl UcumUnit for LowPowerField {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("low power field", None)
    }
    fn primary_code(&self) -> &'static str {
        "[LPF]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("LPF")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 100.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(100, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Katal;
impl UcumUnit for Katal {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("katal", None)
    }
    fn primary_code(&self) -> &'static str {
        "kat"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("kat")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "mol/s",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(602_213_670_000_000_000_000_000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Unit;
impl UcumUnit for Unit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "U"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("U")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "umol/min",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(10_036_894_500_000_000, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct InternationalUnitIU;
impl UcumUnit for InternationalUnitIU {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("international unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[iU]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("IU")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct InternationalUnitIu;
impl UcumUnit for InternationalUnitIu {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("international unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[IU]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("i.U.")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[iU]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ArbitraryUnit;
impl UcumUnit for ArbitraryUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("arbitrary unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[arb'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("arb. U")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct UnitedStatesPharmacopeiaUnit;
impl UcumUnit for UnitedStatesPharmacopeiaUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("United States Pharmacopeia unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[USP'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("U.S.P.")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct GplUnit;
impl UcumUnit for GplUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("GPL unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[GPL'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MplUnit;
impl UcumUnit for MplUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("MPL unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[MPL'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AplUnit;
impl UcumUnit for AplUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("APL unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[APL'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct BethesdaUnit;
impl UcumUnit for BethesdaUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Bethesda unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[beth'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AntiFactorXaUnit;
impl UcumUnit for AntiFactorXaUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("anti factor Xa unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[anti'Xa'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ToddUnit;
impl UcumUnit for ToddUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Todd unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[todd'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DyeUnit;
impl UcumUnit for DyeUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Dye unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[dye'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SomogyiUnit;
impl UcumUnit for SomogyiUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Somogyi unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[smgy'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct BodanskyUnit;
impl UcumUnit for BodanskyUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Bodansky unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[bdsk'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct KingArmstrongUnit;
impl UcumUnit for KingArmstrongUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("King-Armstrong unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[ka'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct KunkelUnit;
impl UcumUnit for KunkelUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Kunkel unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[knk'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MacLaganUnit;
impl UcumUnit for MacLaganUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Mac Lagan unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[mclg'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct TuberculinUnit;
impl UcumUnit for TuberculinUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("tuberculin unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[tb'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FiftyPercentCellCultureInfectiousDose;
impl UcumUnit for FiftyPercentCellCultureInfectiousDose {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("50% cell culture infectious dose", None)
    }
    fn primary_code(&self) -> &'static str {
        "[CCID_50]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("CCID<sub>50</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FiftyPercentTissueCultureInfectiousDose;
impl UcumUnit for FiftyPercentTissueCultureInfectiousDose {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("50% tissue culture infectious dose", None)
    }
    fn primary_code(&self) -> &'static str {
        "[TCID_50]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("TCID<sub>50</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FiftyPercentEmbryoInfectiousDose;
impl UcumUnit for FiftyPercentEmbryoInfectiousDose {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("50% embryo infectious dose", None)
    }
    fn primary_code(&self) -> &'static str {
        "[EID_50]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("EID<sub>50</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PlaqueFormingUnits;
impl UcumUnit for PlaqueFormingUnits {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("plaque forming units", None)
    }
    fn primary_code(&self) -> &'static str {
        "[PFU]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("PFU")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FocusFormingUnits;
impl UcumUnit for FocusFormingUnits {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("focus forming units", None)
    }
    fn primary_code(&self) -> &'static str {
        "[FFU]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("FFU")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ColonyFormingUnits;
impl UcumUnit for ColonyFormingUnits {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("colony forming units", None)
    }
    fn primary_code(&self) -> &'static str {
        "[CFU]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("CFU")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct IndexOfReactivity;
impl UcumUnit for IndexOfReactivity {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("index of reactivity", None)
    }
    fn primary_code(&self) -> &'static str {
        "[IR]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("IR")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct BioequivalentAllergenUnit;
impl UcumUnit for BioequivalentAllergenUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("bioequivalent allergen unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[BAU]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("BAU")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AllergenUnit;
impl UcumUnit for AllergenUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("allergen unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[AU]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("AU")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AllergenUnitForAmbrosiaArtemisiifolia;
impl UcumUnit for AllergenUnitForAmbrosiaArtemisiifolia {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("allergen unit for Ambrosia artemisiifolia", None)
    }
    fn primary_code(&self) -> &'static str {
        "[Amb'a'1'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Amb a 1 U")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ProteinNitrogenUnit;
impl UcumUnit for ProteinNitrogenUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("protein nitrogen unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[PNU]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("PNU")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LimitOfFlocculation;
impl UcumUnit for LimitOfFlocculation {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Limit of flocculation", None)
    }
    fn primary_code(&self) -> &'static str {
        "[Lf]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Lf")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DAntigenUnit;
impl UcumUnit for DAntigenUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("D-antigen unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[D'ag'U]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FibrinogenEquivalentUnit;
impl UcumUnit for FibrinogenEquivalentUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("fibrinogen equivalent unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[FEU]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ElisaUnit;
impl UcumUnit for ElisaUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("ELISA unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[ELU]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct EhrlichUnit;
impl UcumUnit for EhrlichUnit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Ehrlich unit", None)
    }
    fn primary_code(&self) -> &'static str {
        "[EU]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Stere;
impl UcumUnit for Stere {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("stere", None)
    }
    fn primary_code(&self) -> &'static str {
        "st"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("st")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(3) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "m3",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
#[allow(clippy::non_ascii_literal)]
pub struct Angstrom;
impl UcumUnit for Angstrom {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("\u{c5}ngstr\u{f6}m", None)
    }
    fn primary_code(&self) -> &'static str {
        "Ao"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("\u{c5}")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.1,
            unit: "nm",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 10_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Barn;
impl UcumUnit for Barn {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("barn", None)
    }
    fn primary_code(&self) -> &'static str {
        "b"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("b")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 100.0,
            unit: "fm2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 10_000_000_000_000_000_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct TechnicalAtmosphere;
impl UcumUnit for TechnicalAtmosphere {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("technical atmosphere", None)
    }
    fn primary_code(&self) -> &'static str {
        "att"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("at")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "kgf/cm2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(98_066_500, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Mho;
impl UcumUnit for Mho {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("mho", None)
    }
    fn primary_code(&self) -> &'static str {
        "mho"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("mho")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "S",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PoundPerSquareInch;
impl UcumUnit for PoundPerSquareInch {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("pound per square inch", None)
    }
    fn primary_code(&self) -> &'static str {
        "[psi]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("psi")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            Some(unsafe { NonZeroI8::new_unchecked(-2) }),
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "[lbf_av]/[in_i]2",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(8_896_443_230_521, 1_290_320)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Circle;
impl UcumUnit for Circle {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("circle", None)
    }
    fn primary_code(&self) -> &'static str {
        "circ"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("circ")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 2.0,
            unit: "[pi].rad",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3_141_592_653_589_793, 500_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Spere;
impl UcumUnit for Spere {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("spere", None)
    }
    fn primary_code(&self) -> &'static str {
        "sph"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("sph")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(2) }),
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 4.0,
            unit: "[pi].sr",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(3_141_592_653_589_793, 250_000_000_000_000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MetricCarat;
impl UcumUnit for MetricCarat {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("metric carat", None)
    }
    fn primary_code(&self) -> &'static str {
        "[car_m]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("ct<sub>m</sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            None,
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 0.2,
            unit: "g",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 5)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CaratOfGoldAlloys;
impl UcumUnit for CaratOfGoldAlloys {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("carat of gold alloys", None)
    }
    fn primary_code(&self) -> &'static str {
        "[car_Au]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("ct<sub><r>Au</r></sub>")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "/24",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 24)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Smoot;
impl UcumUnit for Smoot {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("Smoot", None)
    }
    fn primary_code(&self) -> &'static str {
        "[smoot]"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        None
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            Some(unsafe { NonZeroI8::new_unchecked(1) }),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        false
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 67.0,
            unit: "[in_i]",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(8509, 5000)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Bit;
impl UcumUnit for Bit {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("bit", None)
    }
    fn primary_code(&self) -> &'static str {
        "bit"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("bit")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "1",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Byte;
impl UcumUnit for Byte {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("byte", None)
    }
    fn primary_code(&self) -> &'static str {
        "By"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("B")
    }
    fn dimension(&self) -> Dimension {
        Dimension([None, None, None, None, None, None, None])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 8.0,
            unit: "bit",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(8, 1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Baud;
impl UcumUnit for Baud {
    fn names(&self) -> (&'static str, Option<&'static str>) {
        ("baud", None)
    }
    fn primary_code(&self) -> &'static str {
        "Bd"
    }
    fn print_symbol(&self) -> Option<&'static str> {
        Some("Bd")
    }
    fn dimension(&self) -> Dimension {
        Dimension([
            None,
            Some(unsafe { NonZeroI8::new_unchecked(-1) }),
            None,
            None,
            None,
            None,
            None,
        ])
    }
    fn is_metric(&self) -> bool {
        true
    }
    fn is_special(&self) -> bool {
        false
    }
    fn value(&self) -> Option<UnitValue<'static>> {
        Some(UnitValue {
            value: 1.0,
            unit: "/s",
            function: None,
        })
    }
    fn scalar(&self) -> Ratio<u128> {
        Ratio::new_raw(1, 1)
    }
}
