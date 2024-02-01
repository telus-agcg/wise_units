use crate::{
    parser::definition::Definition,
    v2::{behavior_traits, type_traits::Prefix as TPrefix},
    Classification, Prefix, UcumSymbol,
};

        // Just delegate to existing implementation.
        UcumSymbol::primary_code(self)
    }

    }

    }

    }
}

impl behavior_traits::convert::ToScalar<f64> for Prefix {
    fn to_scalar(&self) -> f64 {
        self.definition_value()
    }
}
