use quote::{format_ident, quote};

use crate::rust_structs::{RustAtomList, RustClassificationList};

/// Uses the associated handlebars template to generate the Rust code for the
/// `Classification` enum.
///
pub(super) fn generate_file_body(atom_list: &RustAtomList) -> String {
    let classification_list = RustClassificationList::from(atom_list);
    let variants: Vec<_> = classification_list
        .type_names
        .iter()
        .map(|tn| format_ident!("{tn}"))
        .collect();

    let tokens = quote! {
        /// Classification signifies the system of units from which a unit is defined
        /// in.
        ///
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Classification {
            #(#variants),*
        }

        impl Default for Classification {
            fn default() -> Self {
                Self::Si
            }
        }

        impl std::fmt::Display for Classification {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s = match self {
                    Self::Apoth => "Apoth",
                    Self::Avoirdupois => "Avoirdupois",
                    Self::BritLength => "BritLength",
                    Self::BritVolumes => "BritVolumes",
                    Self::Cgs => "Cgs",
                    Self::Chemical => "Chemical",
                    Self::Clinical => "Clinical",
                    Self::Const => "Const",
                    Self::Dimless => "Dimless",
                    Self::Heat => "Heat",
                    Self::Infotech => "Infotech",
                    Self::Intcust => "Intcust",
                    Self::Iso1000 => "Iso1000",
                    Self::Levels => "Levels",
                    Self::Misc => "Misc",
                    Self::Si => "Si",
                    Self::Troy => "Troy",
                    Self::Typeset => "Typeset",
                    Self::UsLengths => "UsLengths",
                    Self::UsVolumes => "UsVolumes",
                };

                f.write_str(s)
            }
        }
    };

    super::pretty_format(&tokens)
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;

    use crate::rust_structs::RustAtom;

    use super::*;

    #[test]
    fn render_test() {
        let rust_atom_list = RustAtomList {
            atoms: vec![RustAtom {
                type_name: String::new(),
                classification: "CoolBeans".to_string(),
                dim: None,
                definition_signature: TokenStream::new(),
                primary_code: String::new(),
                print_symbol: None,
                property: String::new(),
                names: vec![],
                secondary_code: None,
                is_arbitrary: false,
                is_metric: false,
                is_special: false,
            }],
        };

        let output = generate_file_body(&rust_atom_list);

        assert_eq!(&output[0..4], "//--");
        assert_eq!(output.lines().count(), 44);
    }
}
