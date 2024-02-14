#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]

// Internal structs for mapping parser Rule data to an intermediate
// representation of a Unit.
mod annotatable;
mod annotation;
mod ast_term;
mod basic_component;
mod component;
mod digits;
mod exponent;
mod factor;
mod finishable;
mod main_term;
mod simple_unit;

use self::{
    annotatable::Annotatable, annotation::Annotation, ast_term::AstTerm,
    basic_component::BasicComponent, component::Component, digits::Digits, exponent::Exponent,
    factor::Factor, finishable::Finishable, main_term::MainTerm, simple_unit::SimpleUnit,
};
use crate::parser::{terms::term_parser::Rule, Atom, Error, Prefix, Term, Visit};
use pest::iterators::{Pair, Pairs};

#[allow(clippy::large_enum_variant)]
#[allow(clippy::result_large_err)]
pub(crate) fn map(mut pairs: Pairs<'_, Rule>) -> Result<Vec<Term>, Error> {
    fn visit_pairs(pair: Pair<'_, Rule>) -> Result<Vec<Term>, Error> {
        let main_term = if pair.as_rule() == Rule::main_term {
            MainTerm::visit(pair)?
        } else {
            return Err(Error::UnknownUnitString(
                pair.as_span().as_str().to_string(),
            ));
        };

        let mut terms: Vec<Term> = main_term.into();
        terms.shrink_to_fit();

        Ok(terms)
    }

    match pairs.next() {
        Some(pair) => Ok(visit_pairs(pair)?),
        None => Ok(vec![]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{
        terms::term_parser::{Rule, TermParser},
        Prefix,
    };
    use pest::Parser;

    macro_rules! validate_interpret {
        ($test_name:ident, $input:expr, $($terms:expr),+) => {
            #[test]
            fn $test_name() {
                let pairs = TermParser::parse(Rule::main_term, $input).unwrap();
                let actual = map(pairs).unwrap();
                let expected = vec![$($terms),+];

                assert_eq!(actual, expected);
            }
        };
    }

    #[test]
    fn validate_exponent() {
        let pairs = TermParser::parse(Rule::main_term, "m-3").unwrap();
        let actual = map(pairs).unwrap();

        let expected_term = term!(Meter, exponent: -3);
        let expected = vec![expected_term];

        assert_eq!(actual, expected);

        let pairs = TermParser::parse(Rule::main_term, "km2/m-3").unwrap();
        let actual = map(pairs).unwrap();

        let term1 = term!(Kilo, Meter, exponent: 2);
        let term2 = term!(Meter, exponent: 3);
        let expected = vec![term1, term2];

        assert_eq!(actual, expected);
    }

    validate_interpret!(validate_interpret_meter, "m", term!(Meter));
    validate_interpret!(
        validate_interpret_meter_positive_exponent,
        "m2",
        term!(Meter, exponent: 2)
    );
    validate_interpret!(
        validate_interpret_meter_negative_exponent,
        "m-2",
        term!(Meter, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_meter_factor,
        "2m",
        term!(Meter, factor: 2)
    );
    validate_interpret!(validate_interpret_kilometer, "km", term!(Kilo, Meter));

    validate_interpret!(validate_interpret_kilobyte, "kBy", term!(Kilo, Byte));
    validate_interpret!(validate_sec_interpret_klobyte, "KBY", term!(Kilo, Byte));
    validate_interpret!(validate_sec_interpret_kibibyte, "KIBBY", term!(Kibi, Byte));
    validate_interpret!(validate_interpret_kibibyte, "KiBy", term!(Kibi, Byte));
    validate_interpret!(validate_interpret_mebibyte, "MiBy", term!(Mebi, Byte));
    validate_interpret!(validate_interpret_gibibyte, "GiBy", term!(Gibi, Byte));
    validate_interpret!(validate_interpret_tebibyte, "TiBy", term!(Tebi, Byte));
    validate_interpret!(validate_interpret_yottabyte, "YBy", term!(Yotta, Byte));

    // Slash terms
    validate_interpret!(
        validate_interpret_meter_per_second,
        "m/s",
        term!(Meter),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_kilometer_per_second,
        "km/s",
        term!(Kilo, Meter),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter_per_kilosecond,
        "m/ks",
        term!(Meter),
        term!(Kilo, Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter2_per_second,
        "m2/s",
        term!(Meter, exponent: 2),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter_per_second2,
        "m/s2",
        term!(Meter),
        term!(Second, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_meter2_per_second2,
        "m2/s2",
        term!(Meter, exponent: 2),
        term!(Second, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_2meter_per_second,
        "2m/s",
        term!(Meter, factor: 2),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter_per_2second,
        "m/2s",
        term!(Meter),
        term!(Second, exponent: -1, factor: 2)
    );
    validate_interpret!(
        validate_interpret_2meter2_per_2second2,
        "2m2/2s2",
        term!(Meter, factor: 2, exponent: 2),
        term!(Second, factor: 2, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_foot_per_factor,
        "[ft_i]/12",
        term!(FootInternational),
        term!(factor: 12, exponent: -1)
    );

    // Dot terms
    validate_interpret!(
        validate_interpret_meter_second,
        "m.s",
        term!(Meter),
        term!(Second)
    );
    validate_interpret!(
        validate_interpret_meter2_second,
        "m2.s",
        term!(Meter, exponent: 2),
        term!(Second)
    );
    validate_interpret!(
        validate_interpret_meter_second2,
        "m.s2",
        term!(Meter),
        term!(Second, exponent: 2)
    );
    validate_interpret!(
        validate_interpret_meter2_second2,
        "m2.s2",
        term!(Meter, exponent: 2),
        term!(Second, exponent: 2)
    );
    validate_interpret!(
        validate_interpret_2meter_second,
        "2m.s",
        term!(Meter, factor: 2),
        term!(Second)
    );
    validate_interpret!(
        validate_interpret_meter_2second,
        "m.2s",
        term!(Meter),
        term!(Second, factor: 2)
    );
    validate_interpret!(
        validate_interpret_2meter_2second,
        "2m.2s",
        term!(Meter, factor: 2),
        term!(Second, factor: 2)
    );
    validate_interpret!(
        validate_interpret_2meter2_2second2,
        "2m2.2s2",
        term!(Meter, factor: 2, exponent: 2),
        term!(Second, factor: 2, exponent: 2)
    );

    // Dot and slash combined terms
    validate_interpret!(
        validate_interpret_acre_inch_per_acre,
        "[acr_us].[in_i]/[acr_us]",
        term!(AcreUS),
        term!(InchInternational),
        term!(AcreUS, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_2acre3_3inch4_per_4acre5,
        "2[acr_us]3.3[in_i]4/4[acr_us]5",
        term!(AcreUS, factor: 2, exponent: 3),
        term!(InchInternational, factor: 3, exponent: 4),
        term!(AcreUS, factor: 4, exponent: -5)
    );

    #[test]
    #[ignore]
    fn validate_custom_atom() {
        let pairs = TermParser::parse(Rule::main_term, "[meow]").unwrap();

        let actual = map(pairs).unwrap();
        let acre_term = term!(AcreUS);
        let inch_term = term!(InchInternational);
        let acre_inverse_term = term!(AcreUS, exponent: -1);

        let expected = vec![acre_term, inch_term, acre_inverse_term];

        assert_eq!(actual, expected);
    }
}
