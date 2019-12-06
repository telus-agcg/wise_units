use crate::{dimension::Dimension, number::Number, Error};
use std::convert::TryFrom;
use wise_units_parser::tokens as parser_tokens;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UnitDefinition {
    Base(BaseUnit),
    Unit(Unit),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BaseUnit {
    pub(crate) primary_code: &'static str,
    pub(crate) dimension: Dimension,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Unit {
    pub(crate) primary_code: &'static str,
    pub(crate) value: Value,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Value {
    pub(crate) value: Number,
    pub(crate) unit: Option<&'static Unit>,
}

impl UnitDefinition {
    pub fn primary_code(&self) -> &'static str {
        match self {
            UnitDefinition::Base(b) => b.primary_code,
            UnitDefinition::Unit(d) => d.primary_code,
        }
    }
}

impl TryFrom<parser_tokens::AtomSymbol<'_>> for UnitDefinition {
    type Error = Error;

    fn try_from(parser_atom: parser_tokens::AtomSymbol<'_>) -> Result<Self, Self::Error> {
        crate::units::BASE_UNITS
            .iter()
            .find(|atom| atom.primary_code() == parser_atom.0)
            .or_else(|| {
                crate::units::DERIVED_UNITS
                    .iter()
                    .find(|atom| atom.primary_code() == parser_atom.0)
            })
            .cloned()
            .ok_or_else(|| Error::UnknownAtomSymbol(parser_atom.0.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::{METER, TEN_ARBITRARY_POWERS_STAR};
    use std::convert::TryFrom;
    use wise_units_parser::tokens as parser_tokens;

    #[test]
    fn test_base_unit() {
        let atom = UnitDefinition::try_from(parser_tokens::AtomSymbol("m")).unwrap();

        pretty_assertions::assert_eq!(atom, METER);
    }

    #[test]
    fn test_derived_unit() {
        let atom = UnitDefinition::try_from(parser_tokens::AtomSymbol("10*")).unwrap();

        pretty_assertions::assert_eq!(atom, TEN_ARBITRARY_POWERS_STAR);
    }

    #[test]
    fn test_atoms_err() {
        let error = UnitDefinition::try_from(parser_tokens::AtomSymbol("1")).unwrap_err();

        pretty_assertions::assert_eq!(error, Error::UnknownAtomSymbol("1".into()));
    }
}
