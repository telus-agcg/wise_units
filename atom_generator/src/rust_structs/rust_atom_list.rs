use std::collections::BTreeSet;

use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug)]
pub(crate) struct RustAtomList {
    pub(crate) atoms: Vec<RustAtom>,
}

impl RustAtomList {
    pub(crate) fn atom_enum(&self) -> TokenStream {
        let variants = self
            .atoms
            .iter()
            .map(|atom| format_ident!("{}", &atom.type_name));

        quote! {
            #[derive(Clone, Copy, Debug, Eq, PartialOrd, Ord)]
            pub enum Atom {
                #(#variants),*
            }
        }
    }

    /// Emits the Rust code for defining `Atom::definition()`.
    ///
    pub(crate) fn definition_method(&self) -> TokenStream {
        let definitions = self.atoms.iter().map(RustAtom::definition_ts);

        quote! {
            pub(crate) fn definition(self) -> Definition {
                let result = match self {
                    #(#definitions),*
                };

                result.expect("BUG! Bad Atom -> Definition mapping!")
            }
        }
    }

    /// Emits the Rust code for defining `Atom::property()`.
    ///
    pub(crate) fn property_method(&self) -> TokenStream {
        let properties = self.atoms.iter().map(RustAtom::property_ts);

        quote! {
            #[must_use]
            pub const fn property(self) -> Property {
                match self {
                    #(#properties),*
                }
            }
        }
    }

    pub(crate) fn classification_method(&self) -> TokenStream {
        let classifications = self.atoms.iter().map(RustAtom::classification_ts);

        quote! {
            fn classification(&self) -> Classification {
                match *self {
                    #(#classifications),*
                }
            }
        }
    }

    pub(crate) fn v2_classified_impl(&self) -> TokenStream {
        let classifications = self.atoms.iter().map(RustAtom::classification_ts);

        quote! {
            #[allow(unused_qualifications)]
            #[cfg(feature = "v2")]
            impl crate::v2::ucum::UcumClassified for Atom {
                type Classification = crate::Classification;

                fn classification(&self) -> Self::Classification {
                    match *self {
                        #(#classifications),*
                    }
                }
            }
        }
    }

    pub(crate) fn names_method(&self) -> TokenStream {
        let names = self.atoms.iter().map(RustAtom::names_ts);

        quote! {
            fn names(&self) -> Vec<&'static str> {
                match *self {
                    #(#names),*
                }
            }
        }
    }

    pub(crate) fn v2_names_method(&self) -> TokenStream {
        let names = self.atoms.iter().map(RustAtom::v2_names_ts);

        quote! {
            fn names(&self) -> crate::v2::ucum::Names<&'static str> {
                match *self {
                    #(#names),*
                }
            }
        }
    }

    pub(crate) fn primary_code_method(&self) -> TokenStream {
        let primary_codes = self.atoms.iter().map(RustAtom::primary_code_ts);

        quote! {
            fn primary_code(&self) -> &'static str {
                match *self {
                    #(#primary_codes),*
                }
            }
        }
    }

    pub(crate) fn v2_primary_code_method(&self) -> TokenStream {
        let primary_codes = self.atoms.iter().map(RustAtom::primary_code_ts);

        quote! {
            fn primary_code(&self) -> Self::String {
                match *self {
                    #(#primary_codes),*
                }
            }
        }
    }

    pub(crate) fn print_symbol_method(&self) -> TokenStream {
        let print_symbols = self.atoms.iter().filter_map(RustAtom::print_symbol_ts);

        quote! {
            fn print_symbol(&self) -> Option<&'static str> {
                match *self {
                    #(#print_symbols),*,
                    _ => None,
                }
            }
        }
    }

    pub(crate) fn v2_print_symbol_method(&self) -> TokenStream {
        let print_symbols = self.atoms.iter().filter_map(RustAtom::print_symbol_ts);

        quote! {
            fn print_symbol(&self) -> Option<Self::String> {
                match *self {
                    #(#print_symbols),*,
                    _ => None,
                }
            }
        }
    }

    pub(crate) fn secondary_code_method(&self) -> TokenStream {
        let secondary_codes = self.atoms.iter().map(RustAtom::secondary_code_ts);

        quote! {
            fn secondary_code(&self) -> Option<&'static str> {
                match *self {
                    #(#secondary_codes),*,
                }
            }
        }
    }

    pub(crate) fn v2_secondary_code_method(&self) -> TokenStream {
        let secondary_codes = self.atoms.iter().map(RustAtom::secondary_code_ts);

        quote! {
            fn secondary_code(&self) -> Option<Self::String> {
                match *self {
                    #(#secondary_codes),*,
                }
            }
        }
    }

    pub(crate) fn is_arbitrary_method(&self) -> TokenStream {
        let true_variants: BTreeSet<_> = self
            .atoms
            .iter()
            .filter(|atom| atom.is_arbitrary)
            .map(|atom| format_ident!("{}", atom.type_name))
            .collect();

        let true_parts = true_variants.into_iter().map(|atom_variant| {
            quote! { Self::#atom_variant }
        });

        quote! {
            fn is_arbitrary(&self) -> bool {
                matches!(*self, #(#true_parts) |*)
            }
        }
    }

    pub(crate) fn is_special_method(&self) -> TokenStream {
        let true_variants: BTreeSet<_> = self
            .atoms
            .iter()
            .filter(|atom| atom.is_special)
            .map(|atom| format_ident!("{}", atom.type_name))
            .collect();

        let true_parts = true_variants.into_iter().map(|atom_variant| {
            quote! { Self::#atom_variant }
        });

        quote! {
            fn is_special(&self) -> bool {
                matches!(*self, #(#true_parts) |*)
            }
        }
    }

    pub(crate) fn is_metric_method(&self) -> TokenStream {
        let true_variants: BTreeSet<_> = self
            .atoms
            .iter()
            .filter(|atom| atom.is_metric)
            .map(|atom| format_ident!("{}", atom.type_name))
            .collect();

        let true_parts = true_variants.into_iter().map(|atom_variant| {
            quote! { Self::#atom_variant }
        });

        quote! {
            fn is_metric(&self) -> bool {
                matches!(*self, #(#true_parts) |*)
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct RustAtom {
    pub(crate) type_name: String,
    pub(crate) classification: String,
    pub(crate) dim: Option<String>,
    pub(crate) definition_signature: TokenStream,
    pub(crate) primary_code: String,
    pub(crate) print_symbol: Option<String>,
    pub(crate) property: String,
    pub(crate) names: Vec<String>,
    pub(crate) secondary_code: Option<String>,
    pub(crate) is_arbitrary: bool,
    pub(crate) is_metric: bool,
    pub(crate) is_special: bool,
}

impl RustAtom {
    pub(crate) fn definition_ts(&self) -> TokenStream {
        let atom_variant = format_ident!("{}", self.type_name);
        let rhs = &self.definition_signature;

        quote!(Self::#atom_variant => #rhs )
    }

    pub(crate) fn property_ts(&self) -> TokenStream {
        let atom_variant = format_ident!("{}", self.type_name);
        let property_variant = format_ident!("{}", self.property.to_upper_camel_case());

        quote! { Self::#atom_variant => Property::#property_variant }
    }

    pub(crate) fn classification_ts(&self) -> TokenStream {
        let atom_variant = format_ident!("{}", self.type_name);
        let classification_variant = format_ident!("{}", self.classification.to_upper_camel_case());

        quote! { Self::#atom_variant => Classification::#classification_variant }
    }

    pub(crate) fn names_ts(&self) -> TokenStream {
        let atom_variant = format_ident!("{}", self.type_name);
        let names = &self.names;

        quote! {
            Self::#atom_variant => vec![#(#names),*]
        }
    }

    pub(crate) fn v2_names_ts(&self) -> TokenStream {
        let atom_variant = format_ident!("{}", self.type_name);

        match self.names.len() {
            1 => {
                let name = &self.names[0];

                quote! {
                    Self::#atom_variant => crate::v2::ucum::Names::One(#name)
                }
            }
            2 => {
                let name0 = &self.names[0];
                let name1 = &self.names[1];

                quote! {
                    Self::#atom_variant => crate::v2::ucum::Names::Two((#name0, #name1))
                }
            }
            n => panic!("Unexpected number of Atom names: {n}"),
        }
    }

    pub(crate) fn primary_code_ts(&self) -> TokenStream {
        let atom_variant = format_ident!("{}", self.type_name);
        let primary_code = &self.primary_code;

        quote! { Self::#atom_variant => #primary_code }
    }

    pub(crate) fn print_symbol_ts(&self) -> Option<TokenStream> {
        let print_symbol = self.print_symbol.as_ref()?;

        let atom_variant = format_ident!("{}", self.type_name);

        Some(quote! { Self::#atom_variant => Some(#print_symbol) })
    }

    pub(crate) fn secondary_code_ts(&self) -> TokenStream {
        let atom_variant = format_ident!("{}", self.type_name);

        match self.secondary_code.as_ref() {
            Some(secondary_code) => {
                quote! { Self::#atom_variant => Some(#secondary_code) }
            }
            None => {
                quote! { Self::#atom_variant => None }
            }
        }
    }
}
