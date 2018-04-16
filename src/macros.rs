// define public macro for adding atom
// The atom needs to be added to:
//      * `enum Atom` - The type that Term holds.
//      * AtomMap - The thing used for lookup in Interpreter.
//

macro_rules! build_atoms {
    (
        $(
            name => $variant_name: ident,
            classification => $classification_variant: ident,
            definition => $definition_value: expr, $definition_string: expr,
            primary_code => $primary_code: expr
         );+
    ) => {
        use classification::Classification;
        use definition::Definition;
        use measurable::Measurable;
        use error::Error;

        #[derive(Debug)]
        pub enum TestAtom {
            $(
                $variant_name,
            )+
        }

        impl TestAtom {
            pub fn lookup(&self, primary_key: &str) -> Result<TestAtom, Error> {
                match primary_key {
                    $(
                        $primary_code => Ok(TestAtom::$variant_name),
                    )+
                    _ => Err(Error::UnknownUnitString(primary_key.to_string()))
                }
            }

            pub fn classification(&self) -> Classification {
                match *self {
                    $(
                        TestAtom::$variant_name => Classification::$classification_variant,
                    )+
                }
            }

            pub fn definition(&self) -> Definition {
                let result = match *self {
                    $(
                        TestAtom::$variant_name => {
                            Definition::new($definition_value, $definition_string)
                        },
                    )+
                };

                result.expect("BUG! Bad Definition for $variant_name")
            }
        }

        impl PartialEq for TestAtom {
            fn eq(&self, other: &TestAtom) -> bool {
                // Composition check
                self.definition().scalar() == other.definition().scalar()
            }
        }
    };
}

// macro_rules! add_atom {
//     $(
//         name => $variant_name: ident,
//         classification => $classification_variant: ident,

//     )+ => {
//     };
// }

