use handlebars::{Handlebars, Helper, Output, RenderContext, RenderError};
use heck::CamelCase;
use rust_structs::{PestSymbolList, RustAtomList, RustClassificationList, RustFunctionSet,
                   RustMapperList, RustPropertyList, RustUnit};
use std::fs;
use std::path::PathBuf;
use toml_structs::{TomlAtom, TomlAtomList};

lazy_static! {
    pub static ref HANDLEBARS: Handlebars = {
        fn register_template(template_name: &str, extension: &str, handlebars: &mut Handlebars) {
            let file = format!(
                "../custom/src/templates/{}.{}.hbs",
                template_name, extension
            );
            let pb = PathBuf::from(file);

            if let Err(e) = handlebars.register_template_file(template_name, pb.clone()) {
                println!("file: {:?}", fs::canonicalize(&pb));
                panic!("{}", e);
            }
        }

        let mut handlebars = Handlebars::new();
        handlebars.register_escape_fn(::handlebars::no_escape);
        handlebars.register_helper("camelCase", Box::new(camel_case_helper));

        register_template("atom", "rs", &mut handlebars);
        register_template("classification", "rs", &mut handlebars);
        register_template("property", "rs", &mut handlebars);
        register_template("symbol", "pest", &mut handlebars);
        register_template("mapper", "rs", &mut handlebars);

        handlebars
    };
}

pub fn build_rust_atoms(atom_list: &TomlAtomList) -> RustAtomList {
    let mut units = build_rust_base_units(atom_list);
    units.append(&mut build_rust_units(atom_list));

    RustAtomList { atoms: units }
}

pub fn build_classification_file_body(atom_list: &RustAtomList) -> String {
    let classification_list = RustClassificationList::from(atom_list);

    HANDLEBARS
        .render("classification", &classification_list)
        .unwrap()
}

pub fn build_property_file_body(atom_list: &RustAtomList) -> String {
    let property_list = RustPropertyList::from(atom_list);

    HANDLEBARS.render("property", &property_list).unwrap()
}

pub fn build_atom_file_body(atom_list: &RustAtomList) -> String {
    HANDLEBARS.render("atom", atom_list).unwrap()
}

pub fn build_symbol_file_body(atom_list: &RustAtomList) -> String {
    let symbol_list = PestSymbolList::from(atom_list);

    HANDLEBARS.render("symbol", &symbol_list).unwrap()
}

pub fn build_mapper_file_body(atom_list: &RustAtomList) -> String {
    let mapper_list = RustMapperList::from(atom_list);

    HANDLEBARS.render("mapper", &mapper_list).unwrap()
}

fn build_rust_base_units(atom_list: &TomlAtomList) -> Vec<RustUnit> {
    atom_list
        .base_units
        .iter()
        .map(|bu| RustUnit {
            type_name: bu.type_name(),
            classification: "Si".to_string(),
            dim: Some(bu.dim.clone()),
            definition_signature: "1.0, \"1\", None".to_string(),
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

fn build_rust_units(atom_list: &TomlAtomList) -> Vec<RustUnit> {
    atom_list
        .units
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
                    "{:?}, \"{}\", Some({})",
                    function.value,
                    function.unit.clone(),
                    function_set_string
                )
            } else if &u.primary_code == "[pi]" {
                format!(
                    "::std::f64::consts::PI, \"{}\", None",
                    u.definition.unit.clone()
                )
            } else {
                format!(
                    "{:?}, \"{}\", None",
                    u.definition.value,
                    u.definition.unit.clone()
                )
            };

            RustUnit {
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

fn build_function_set_string(rust_function_set: &RustFunctionSet) -> String {
    format!(
        "FunctionSet {{ convert_from: {}, convert_to: {} }}",
        rust_function_set.convert_from, rust_function_set.convert_to
    )
}

fn build_scalar_function(primary_code: &str) -> String {
    match primary_code {
        "B"               => "|value: f64| 10_f64.powf(value)",
        "B[SPL]"          => "|value: f64| 10_f64.powf(value / 2.0)",
        "B[V]"            => "|value: f64| 10_f64.powf(value / 2.0)",
        "B[mV]"           => "|value: f64| 10_f64.powf(value / 2.0)",
        "B[uV]"           => "|value: f64| 10_f64.powf(value / 2.0)",
        "B[10.nV]"        => "|value: f64| 10_f64.powf(value / 2.0)",
        "B[W]"            => "|value: f64| 10_f64.powf(value)",
        "B[kW]"           => "|value: f64| 10_f64.powf(value)",
        "bit_s"           => "|value: f64| 2_f64.powf(value)",
        "Cel"             => "|value: f64| value + 273.15",
        "Np"              => "|value: f64| ::std::f64::consts::E.powf(value)",
        "%[slope]"        => "|value: f64| value.tan() * 100.0",
        "[hp'_X]"         => "|value: f64| 10_f64.powf(-value)",
        "[hp'_C]"         => "|value: f64| 100_f64.powf(-value)",
        "[hp'_M]"         => "|value: f64| 1_000_f64.powf(-value)",
        "[hp'_Q]"         => "|value: f64| 50_000_f64.powf(-value)",
        "[m/s2/Hz^(1/2)]" => "|value: f64| value.powi(2)",
        "[pH]"            => "|value: f64| -value.log10()",
        "[degF]"          => "|value: f64| 5.0 / 9.0 * (value + 459.67)",
        "[degRe]"         => "|value: f64| (value / 0.8) + 273.15",
        "[p'diop]"        => "|value: f64| value.tan() * 100.0",
        _                 => panic!("Unknown primary code on special unit: {}", primary_code),
    }.to_string()
}

fn build_magnitude_function(primary_code: &str) -> String {
    match primary_code {
        "B"               => "|value: f64| value.log10()",
        "B[SPL]"          => "|value: f64| 2.0 * value.log10()",
        "B[V]"            => "|value: f64| 2.0 * value.log10()",
        "B[mV]"           => "|value: f64| 2.0 * value.log10()",
        "B[uV]"           => "|value: f64| 2.0 * value.log10()",
        "B[10.nV]"        => "|value: f64| 2.0 * value.log10()",
        "B[W]"            => "|value: f64| value.log10()",
        "B[kW]"           => "|value: f64| value.log10()",
        "bit_s"           => "|value: f64| value.log2()",
        "Cel"             => "|value: f64| value - 273.15",
        "Np"              => "|value: f64| value.ln()",
        "%[slope]"        => "|value: f64| (value / 100.0).atan()",
        "[hp'_X]"         => "|value: f64| -value.log10()",
        "[hp'_C]"         => "|value: f64| -value.ln() / 100_f64.ln()",
        "[hp'_M]"         => "|value: f64| -value.ln() / 1_000_f64.ln()",
        "[hp'_Q]"         => "|value: f64| -value.ln() / 50_000_f64.ln()",
        "[m/s2/Hz^(1/2)]" => "|value: f64| value.sqrt()",
        "[pH]"            => "|value: f64| 10.0_f64.powf(-value)",
        "[degF]"          => "|value: f64| 9.0 * value / 5.0 - 459.67",
        "[degRe]"         => "|value: f64| (value - 273.15) * 0.8",
        "[p'diop]"        => "|value: f64| (value / 100.0).atan()",
        _                 => panic!("Unknown primary code on special unit: {}", primary_code),
    }.to_string()
}

fn camel_case_helper(
    h: &Helper,
    _: &Handlebars,
    _rc: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    let rendered = param.to_string().to_camel_case();

    out.write(rendered.as_ref())?;

    Ok(())
}
