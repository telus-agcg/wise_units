use std::collections::{BTreeMap, BTreeSet};

use heck::ToUpperCamelCase;
use maplit::btreeset;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use serde::Serialize;

use super::RustAtomList;

type PropertyTypeName = String;

#[derive(Debug, Serialize)]
pub(crate) struct RustPropertyList {
    pub(crate) properties: BTreeMap<PropertyTypeName, RustProperty>,
}

impl RustPropertyList {
    pub(crate) fn variants(&self) -> impl Iterator<Item = Ident> + '_ {
        self.properties.keys().map(|key| format_ident!("{key}"))
    }

    pub(crate) fn atoms(&self) -> TokenStream {
        self.properties
            .iter()
            .map(|(k, v)| {
                let variant = format_ident!("{k}");
                let atoms = v.atoms();

                quote! {
                    Self::#variant => vec![
                        #atoms
                    ],
                }
            })
            .collect()
    }

    pub(crate) fn display(&self) -> TokenStream {
        self.properties
            .keys()
            .map(|key| {
                let variant = format_ident!("{key}");
                quote! { Self::#variant => #key, }
            })
            .collect()
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct RustProperty {
    // Using BTreeSet here just for sorting.
    pub(crate) atom_type_names: BTreeSet<String>,
    pub(crate) description: String,
}

impl RustProperty {
    fn atoms(&self) -> TokenStream {
        self.atom_type_names
            .iter()
            .map(|name| {
                let name = format_ident!("{name}");
                quote! { Atom::#name, }
            })
            .collect()
    }
}

impl<'a> From<&'a RustAtomList> for RustPropertyList {
    fn from(atom_list: &'a RustAtomList) -> Self {
        let mut properties: BTreeMap<PropertyTypeName, RustProperty> = BTreeMap::new();

        for atom in &atom_list.atoms {
            let key = atom.property.to_upper_camel_case();

            let _ = properties
                .entry(key)
                .and_modify(|entry| {
                    let _ = entry.atom_type_names.insert(atom.type_name.clone());
                })
                .or_insert_with(|| RustProperty {
                    atom_type_names: btreeset![atom.type_name.clone()],
                    description: atom.property.clone(),
                });
        }

        Self { properties }
    }
}
