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
    use atom::*;
    use parser::*;
    use prefix::*;

    #[test]
    fn validate_prefix_symbol() {
        assert_eq!(parse_PrefixSymbol("k").unwrap(), PREFIXES[7]);
        assert_eq!(parse_PrefixSymbol("K").unwrap(), PREFIXES[7]);
    }

    #[test]
    fn validate_atom_symbol() {
        assert_eq!(parse_AtomSymbol("m").unwrap(), ATOMS[0]);
        assert_eq!(parse_AtomSymbol("M").unwrap(), ATOMS[0]);
    }

    #[test]
    fn validate_component_with_annotations() {
        let simple_unit = SimpleUnit::PrefixedAtom(PREFIXES[7].clone(), ATOMS[0].clone());
        let negative_exp = Exponent(UnitSign::Negative, 10);
        let annotatable = Annotatable::UnitWithPower(simple_unit, negative_exp);
        let annotation = Annotation("%vol");

        assert_eq!(
            parse_Component("km-10{%vol}").unwrap(),
            Component::AnnotatedAnnotatable(annotatable.clone(), annotation.clone())
            );

        assert_eq!(
            parse_Component("km-10").unwrap(),
            Component::Annotatable(annotatable.clone())
            );

        let annotation = Annotation("wet'tis.");

        assert_eq!(
            parse_Component("{wet'tis.}").unwrap(),
            Component::Annotation(annotation.clone())
            );
    }

    #[test]
    fn validate_component_with_factor() {
        assert_eq!(
            parse_Component("123").unwrap(),
            Component::Factor(Factor(123))
            );
    }

    #[test]
    fn validate_component_with_term() {
        assert_eq!(
            parse_Component("(123)").unwrap(),
            Component::Term(Box::new(Term::Basic(Component::Factor(Factor(123)))))
            );
    }

    #[test]
    fn validate_term_with_dot() {
        assert_eq!(
            parse_Term("g.m").unwrap(),
            Term::DotCombined(
                Component::Annotatable(
                    Annotatable::Unit(
                        SimpleUnit::Atom(ATOMS[2].clone())
                    )
                ),
                Box::new(
                    Term::Basic(
                        Component::Annotatable(
                            Annotatable::Unit(
                                SimpleUnit::Atom(ATOMS[0].clone())
                            )
                        )
                    )
                )
            )
        );
    }

    #[test]
    fn validate_term_with_slash() {
        assert_eq!(
            parse_Term("kg/s").unwrap(),
            Term::SlashCombined(
                Component::Annotatable(
                    Annotatable::Unit(
                        SimpleUnit::PrefixedAtom(PREFIXES[7].clone(), ATOMS[2].clone())
                    )
                ),
                Box::new(
                    Term::Basic(
                        Component::Annotatable(
                            Annotatable::Unit(
                                SimpleUnit::Atom(ATOMS[1].clone())
                            )
                        )
                    )
                )
            )
        );
    }

    #[test]
    fn validate_term_basic() {
        assert_eq!(
            parse_Term("g").unwrap(),
            Term::Basic(
                Component::Annotatable(
                    Annotatable::Unit(
                        SimpleUnit::Atom(ATOMS[2].clone())
                    )
                )
            )
        );
    }

    #[test]
    fn validate_main_term_with_slash() {
        assert_eq!(
            parse_MainTerm("/g{tot'nit}").unwrap(),
            Term::Basic(
                Component::AnnotatedAnnotatable(
                    Annotatable::Unit(
                        SimpleUnit::Atom(ATOMS[2].clone())
                    ),
                    Annotation("tot'nit")
                )
            )
        );
    }
}
