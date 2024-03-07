use quote::quote;

use crate::rust_structs::{RustAtomList, RustPropertyList};

/// Uses the associated handlebars template to generate the Rust code for the
/// `Property` enum.
///
pub(super) fn generate_file_body(atom_list: &RustAtomList) -> String {
    let property_list = RustPropertyList::from(atom_list);
    let variants = property_list.variants();
    let atoms = property_list.atoms();
    let display = property_list.display();

    let tokens = quote! {
        use crate::Atom;
        use std::fmt;

        /// Property categorizes the unit by use. Not much mention of it in the UCUM
        /// HTML spec, but is used throughout the
        /// [XML description](http://unitsofmeasure.org/ucum-essence.xml).
        ///
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum Property {
            #(#variants),*
        }

        impl Property {
            /// All `Atom`s that match the `Property` variant.
            ///
            /// ```
            /// extern crate wise_units;
            /// use wise_units::{Atom, Property};
            ///
            /// assert_eq!(Property::Acidity.atoms(), vec![Atom::PH]);
            /// ```
            ///
            #[allow(clippy::too_many_lines)]
            #[must_use]
            pub fn atoms(self) -> Vec<Atom> {
                match self {
                    #atoms
                }
            }
        }

        impl Default for Property {
            fn default() -> Self {
                Self::Unclassified
            }
        }

        impl fmt::Display for Property {
            #[allow(clippy::too_many_lines)]
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                let string = match self {
                    #display
                };

                write!(formatter, "{string}")
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn validate_display() {
                let a = format!("{}", Property::Acceleration);
                assert_eq!(&a, "Acceleration");
            }
        }
    };

    super::pretty_format(&tokens)
}
