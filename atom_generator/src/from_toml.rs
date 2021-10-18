//! This module handles reading .toml files into a `RustAtomList` struct, where
//! the struct is what's used for generating the code for which this crate
//! exists.
//!
//! There are two sources of atom/unit data for which code can be generated:
//!
//! * `Atoms.toml`, defined within the `wise_units` project
//! * `CustomAtoms.toml`, defined within third-party crates who wish to
//!   defined their own units (in addition to those in `Atoms.toml`).
//!
//! In both cases, this module:
//!
//! * Reads the file
//! * Deserializes the TOML into `wise_units-atom_generator` `toml_structs`
//!   structs * Transforms the `toml_structs` into
//!   `wise_units-atom_generator` `rust_structs` structs * Transforms those
//!   into a `RustAtomList` struct.
//!
//! NOTE: Code generated for "special" units can *not* be automatically deduced
//! from the UCUM.xml file; thus, any new special units that get added to the
//! UCUM spec must be manually updated in the relevant functions below, and
//! support for custom special units does not yet exist.
//!

use crate::{
    rust_structs::{RustAtom, RustFunctionSet},
    toml_structs::{TomlAtom, TomlBaseUnit, TomlUnit},
};
use heck::CamelCase;

pub(crate) mod atoms;
pub(crate) mod custom_atoms;

/// Transforms a `Vec<TomlBaseUnit>` to a `Vec<RustAtom>`.
///
fn transform_base_units(atom_list_base_units: &[TomlBaseUnit]) -> Vec<RustAtom> {
    atom_list_base_units
        .iter()
        .map(|bu| RustAtom {
            type_name: bu.type_name(),
            classification: "Si".to_string(),
            dim: Some(bu.dim.clone()),
            definition_signature: "Ok(Definition::default())".to_string(),
            primary_code: bu.primary_code.clone(),
            print_symbol: Some(bu.print_symbol.clone()),
            property: bu.property.clone(),
            names: bu.names.clone(),
            secondary_code: Some(bu.secondary_code.clone()),
            is_arbitrary: false,
            is_metric: true,
            is_special: false,
        })
        .collect()
}

/// Transforms a `Vec<TomlUnit>` to a `Vec<RustAtom>`.
///
fn transform_units(atom_list_units: &[TomlUnit]) -> Vec<RustAtom> {
    atom_list_units
        .iter()
        .map(|u| {
            let definition_signature = if u.is_special {
                let function_set = RustFunctionSet {
                    convert_from: build_magnitude_function(&u.primary_code),
                    convert_to: build_scalar_function(&u.primary_code),
                };

                let function = u.definition.function.clone().unwrap();
                let function_set_string = build_function_set_string(&function_set);

                format!(
                    "Definition::new({:?}, \"{}\", Some({}))",
                    function.value, function.unit, function_set_string
                )
            } else if &u.primary_code == "[pi]" {
                format!(
                    "Definition::new(::std::f64::consts::PI, \"{}\", None)",
                    u.definition.unit.clone()
                )
            } else if u.definition.value.eq(&1.0_f64) && &u.definition.unit == "1" {
                "Ok(Definition::default())".to_string()
            } else {
                format!(
                    "Definition::new({:?}, \"{}\", None)",
                    u.definition.value,
                    u.definition.unit.clone()
                )
            };

            RustAtom {
                type_name: u.type_name(),
                classification: u.classification.clone().to_camel_case(),
                dim: None,
                definition_signature,
                primary_code: u.primary_code.clone(),
                print_symbol: u.print_symbol.clone(),
                property: u.property.clone(),
                names: u.names.clone(),
                secondary_code: u.secondary_code.clone(),
                is_arbitrary: u.is_arbitrary,
                is_metric: u.is_metric,
                is_special: u.is_special,
            }
        })
        .collect()
}

/// Determines which function to generate for converting *from* the unit with
/// `primary_code` to its base unit.
///
fn build_scalar_function(primary_code: &str) -> String {
    match primary_code {
        "B" | "B[W]" | "B[kW]" => "|value: f64| 10_f64.powf(value)",
        "B[SPL]" | "B[V]" | "B[mV]" | "B[uV]" | "B[10.nV]" => {
            "|value: f64| 10_f64.powf(value / 2.0)"
        }
        "bit_s" => "|value: f64| value.exp2()",
        "Cel" => "|value: f64| value + 273.15",
        "Np" => "|value: f64| value.exp()",
        "%[slope]" | "[p'diop]" => "|value: f64| value.tan() * 100.0",
        "[hp'_X]" => "|value: f64| 10_f64.powf(-value)",
        "[hp'_C]" => "|value: f64| 100_f64.powf(-value)",
        "[hp'_M]" => "|value: f64| 1_000_f64.powf(-value)",
        "[hp'_Q]" => "|value: f64| 50_000_f64.powf(-value)",
        "[m/s2/Hz^(1/2)]" => "|value: f64| value * value",
        "[pH]" => "|value: f64| -value.log10()",
        "[degF]" => "|value: f64| 5.0 / 9.0 * (value + 459.67)",
        "[degRe]" => "|value: f64| (value / 0.8) + 273.15",
        _ => panic!("Unknown primary code on special unit: {}", primary_code),
    }
    .to_string()
}

/// Determines which function to generate for converting *to* the unit with
/// `primary_code` from its base unit. These are only for "special" units and
/// as far as I can tell can *not* be automatically deduced from the UCUM.xml
/// file; thus any new special units that get added to the UCUM spec must be
/// manually updated here, and support for custom special units does not yet
/// exist.
///
fn build_magnitude_function(primary_code: &str) -> String {
    match primary_code {
        "B" | "B[W]" | "B[kW]" => "|value: f64| value.log10()",
        "B[SPL]" | "B[V]" | "B[mV]" | "B[uV]" | "B[10.nV]" => "|value: f64| 2.0 * value.log10()",
        "bit_s" => "|value: f64| value.log2()",
        "Cel" => "|value: f64| value - 273.15",
        "Np" => "|value: f64| value.ln()",
        "%[slope]" | "[p'diop]" => "|value: f64| (value / 100.0).atan()",
        "[hp'_X]" => "|value: f64| -value.log10()",
        "[hp'_C]" => "|value: f64| -value.ln() / 100_f64.ln()",
        "[hp'_M]" => "|value: f64| -value.ln() / 1_000_f64.ln()",
        "[hp'_Q]" => "|value: f64| -value.ln() / 50_000_f64.ln()",
        "[m/s2/Hz^(1/2)]" => "|value: f64| value.sqrt()",
        "[pH]" => "|value: f64| 10.0_f64.powf(-value)",
        "[degF]" => "|value: f64| 9.0 * value / 5.0 - 459.67",
        "[degRe]" => "|value: f64| (value - 273.15) * 0.8",
        _ => panic!("Unknown primary code on special unit: {}", primary_code),
    }
    .to_string()
}

fn build_function_set_string(rust_function_set: &RustFunctionSet) -> String {
    format!(
        "FunctionSet {{ convert_from: {}, convert_to: {} }}",
        rust_function_set.convert_from, rust_function_set.convert_to
    )
}
