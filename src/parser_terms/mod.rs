mod annotatable;
mod annotation;
mod component;
mod exponent;
mod factor;
mod simple_unit;
mod term;
mod unit_sign;

pub use parser_terms::annotatable::Annotatable;
pub use parser_terms::annotation::Annotation;
pub use parser_terms::component::Component;
pub use parser_terms::exponent::Exponent;
pub use parser_terms::factor::Factor;
pub use parser_terms::simple_unit::SimpleUnit;
pub use parser_terms::term::Term;
pub use parser_terms::unit_sign::UnitSign;

#[cfg(test)]
mod tests {
    use super::*;
    use unit::Unit;
    use unit::base::{Gram, Meter};
    use parser::*;
    use prefix::PREFIXES;

    #[test]
    fn validate_prefix_symbol() {
        assert_eq!(parse_PrefixSymbol("k").unwrap(), PREFIXES[7]);
        assert_eq!(parse_PrefixSymbol("K").unwrap(), PREFIXES[7]);
    }

    #[test]
    fn validate_atom_symbol() {
        let meter = Box::new(Meter) as Box<Unit>;
        let atom_symbol = parse_AtomSymbol("m").unwrap() as Box<Unit>;
        assert_eq!(&atom_symbol, &meter);

        let atom_symbol = parse_AtomSymbol("M").unwrap() as Box<Unit>;
        assert_eq!(&atom_symbol, &meter);
    }

    #[test]
    fn validate_main_term_with_slash() {
        assert_eq!(
            parse_MainTerm("/g{tot'nit}").unwrap(),
            Term::Basic(
                Component::AnnotatedAnnotatable(
                    Annotatable::Unit(
                        SimpleUnit::Atom(Box::new(Gram))
                    ),
                    Annotation("tot'nit")
                )
            )
        );
    }
}
