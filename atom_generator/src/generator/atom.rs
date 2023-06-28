use quote::quote;

use crate::rust_structs::rust_atom_list::RustAtomList;

/// Uses the associated handlebars template to generate the Rust code for the
/// `Atom` enum.
///
pub(super) fn generate_file_body(atom_list: &RustAtomList) -> String {
    let atom_enum = atom_list.atom_enum();
    let definition_method = atom_list.definition_method();
    let property_method = atom_list.property_method();

    let classification_method = atom_list.classification_method();
    let names_method = atom_list.names_method();
    let primary_code_method = atom_list.primary_code_method();
    let print_symbol_method = atom_list.print_symbol_method();
    let secondary_code_method = atom_list.secondary_code_method();

    let is_arbitrary_method = atom_list.is_arbitrary_method();
    let is_special_method = atom_list.is_special_method();
    let is_metric_method = atom_list.is_metric_method();

    let tokens = quote! {
        mod composable;
        mod display;
        mod hash;
        mod partial_eq;
        mod reducible;

        use crate::{
            is_compatible_with::DefaultCompatibility,
            parser::{
                definition::Definition, function_set::FunctionSet, Classification, Property, UcumSymbol,
            },
            reducible::Reducible,
            UcumUnit, Unit,
        };

        #atom_enum

        impl Atom {
            #definition_method

            #property_method
        }

        impl UcumSymbol for Atom {
            #classification_method

            #names_method

            #primary_code_method

            #print_symbol_method

            #secondary_code_method

            fn definition_value(&self) -> f64 {
                self.definition().value()
            }

            fn definition_unit(&self) -> Unit {
                Unit::new(self.definition().terms().clone())
            }
        }

        impl UcumUnit for Atom {
            fn scalar(&self) -> f64 {
                self.reduce_value(1.0)
            }

            fn magnitude(&self) -> f64 {
                self.calculate_magnitude(self.scalar())
            }

            #is_arbitrary_method

            #is_special_method

            #is_metric_method
        }

        impl DefaultCompatibility for Atom {}
    };

    super::pretty_format(&tokens)
}
